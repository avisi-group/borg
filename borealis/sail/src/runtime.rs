//! Interface to OCaml runtime infrastructure
//!
//! Due to an issue with <https://github.com/zshipko/ocaml-rs> not supporting multithreaded
//! environments, all interaction with OCaml objects *must* occur on a single
//! worker thread.
//!
//! For example, in `load_files`, the String returned by
//! `internal_process_file_load_files` is safe to cross the interface boundary
//! (in `Response::LoadFiles`) but the AST and type environment of type `Value`
//! is *not* safe to send across the interface. Doing so will result in a
//! `SIGSEGV: invalid memory reference`

use {
    crate::{
        ast::Ast,
        error::{Error, WrapperError},
        json::ModelConfig,
        type_check::Env,
    },
    log::{error, trace},
    ocaml::{
        interop::{BoxRoot, ToOCaml},
        FromValue, List, Runtime as OCamlRuntime, Value,
    },
    std::{os::unix::prelude::OsStringExt, sync::mpsc, thread},
};

ocaml::import! {
    fn internal_util_dedup(l: List<Value>) -> Result<List<i32>, WrapperError>;

    fn internal_type_check_initial_env() -> Result<Value, WrapperError>;

    // val load_files : ?check:bool -> (Arg.key * Arg.spec * Arg.doc) list -> Type_check.Env.t -> string list -> (string * Type_check.tannot ast * Type_check.Env.t)
    fn internal_process_files(check: bool, options: List<Value>, env: Value, files: List<BoxRoot<String>>) -> Result<Value, WrapperError>;

    pub fn internal_bindings_to_list(input: Value) -> Result<Value, WrapperError>;

    pub fn internal_bigint_to_string(input: Value) -> Result<String, WrapperError>;

    fn internal_add_num(a: String, b: String) -> Result<String, WrapperError>;

    fn internal_set_non_lexical_flow(b: bool) -> Result<(), WrapperError>;

    fn internal_set_no_lexp_bounds_check(b: bool) -> Result<(), WrapperError>;
}

/// Wrapper around OCaml runtime
///
/// Workaround for <https://github.com/zshipko/ocaml-rs/issues/102> where ocaml::Runtime must be
/// owned and interacted with from a single thread or there are intermittent
/// memory issues.
pub struct Runtime {
    request: mpsc::Sender<Request>,
    response: mpsc::Receiver<Result<Response, Error>>,
}

impl Runtime {
    /// Instantiate a new Runtime with corresponding worker thread
    pub fn new() -> Self {
        trace!("Creating new OCaml runtime handler");
        let (req_tx, req_rx) = mpsc::channel();
        let (res_tx, res_rx) = mpsc::channel();

        // handle dropped implicitly by not assigning detaches thread
        thread::spawn(move || {
            // initialise runtime *once* in a *single* thread
            let mut rt = ocaml::runtime::init();

            let requests = req_rx;
            let responses = res_tx;

            trace!("Initialised OCaml runtime handler thread, looping...");

            // loop indefinitely processing requests
            loop {
                // block on receiving a request
                match requests.recv() {
                    // if successful, process request
                    Ok(request) => {
                        let response = process_request(&mut rt, request);

                        // log errors if sending failed but do not terminate instead process next
                        // request
                        if let Err(e) = responses.send(response) {
                            error!("Runtime thread failed to send response with error {e}, terminating thread");
                            break;
                        }
                    }
                    // if unsuccessful, channel must be closed so report error and terminate
                    Err(e) => {
                        error!("Runtime thread receive returned error {e}, terminating thread");
                        break;
                    }
                }
            }
        });

        Self {
            request: req_tx,
            response: res_rx,
        }
    }

    /// Sends a request and receives a response
    fn request(&self, req: Request) -> Result<Response, Error> {
        self.request.send(req)?;
        self.response.recv()?
    }

    #[cfg(test)]
    pub fn dedup(&self, l: Vec<i32>) -> Result<Vec<i32>, Error> {
        if let Response::Dedup(l) = self.request(Request::Dedup(l))? {
            Ok(l)
        } else {
            panic!("received different response kind to request");
        }
    }

    #[cfg(test)]
    pub fn add_num(&self, a: String, b: String) -> Result<String, Error> {
        if let Response::AddNum(c) = self.request(Request::AddNum((a, b)))? {
            Ok(c)
        } else {
            panic!("received different response kind to request");
        }
    }

    pub fn load_files(&self, config: ModelConfig) -> Result<(String, Ast, Env), Error> {
        #[allow(irrefutable_let_patterns)] // remove when more non-test variants are added
        if let Response::LoadFiles(ret) = self.request(Request::LoadFiles(config))? {
            Ok(ret)
        } else {
            panic!("received different response kind to request");
        }
    }
}

/// Request made against runtime
///
/// Each variant corresponds to one method on the runtime, which in turn
/// correspond to one public function. Variants may *not* contain any OCaml
/// values or will introduce unsoundness.
#[doc(hidden)]
#[derive(Debug)]
pub enum Request {
    // requests only used in tests
    #[cfg(test)]
    Dedup(Vec<i32>),
    #[cfg(test)]
    AddNum((String, String)),

    LoadFiles(ModelConfig),
}

/// Response from runtime.
///
/// Each variant must correspond to one `Request` variant. Variants may *not*
/// contain any OCaml values or will introduce unsoundness.
#[derive(Debug)]
enum Response {
    // requests only used in tests
    #[cfg(test)]
    Dedup(Vec<i32>),
    #[cfg(test)]
    AddNum(String),

    LoadFiles((String, Ast, Env)),
}

/// Process a single incoming request by calling the corresponding OCaml
/// function
///
/// All interactions with OCaml runtime and objects must occur inside this
/// function; breaking the barrier will result in unsoundness as only the worker
/// thread may interact with the runtime.
fn process_request(rt: &mut OCamlRuntime, req: Request) -> Result<Response, Error> {
    match req {
        #[cfg(test)]
        Request::Dedup(list) => {
            use ocaml::ToValue;

            let mut l = List::empty();

            for element in list {
                l = unsafe { l.add(rt, &element.to_value(rt)) };
            }

            Ok(Response::Dedup(
                unsafe { internal_util_dedup(rt, l) }??.into_vec(),
            ))
        }

        #[cfg(test)]
        Request::AddNum((a, b)) => Ok(Response::AddNum(unsafe { internal_add_num(rt, a, b) }??)),

        Request::LoadFiles(ModelConfig { options, files }) => {
            let env = unsafe { internal_type_check_initial_env(rt)?? };

            let mut file_list = List::empty();

            for path in files.into_iter().rev() {
                let path = unsafe { String::from_utf8_unchecked(path.into_os_string().into_vec()) };
                let file_rooted: BoxRoot<String> = path.to_boxroot(rt);
                file_list = unsafe { file_list.add(rt, &file_rooted) };
            }

            unsafe { internal_set_non_lexical_flow(rt, options.non_lexical_flow) }??;
            unsafe { internal_set_no_lexp_bounds_check(rt, options.no_lexp_bounds_check) }??;

            trace!("Calling internal_process_files");

            let value =
                unsafe { internal_process_files(rt, false, List::empty(), env, file_list) }??;

            trace!("Converting AST from ocaml::Value");

            let resp = <(String, Ast, Env)>::from_value(value);

            trace!("Finished converting AST from ocaml::Value");

            Ok(Response::LoadFiles(resp))
        }
    }
}
