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
        type_check::Env,
        types::OCamlString,
    },
    log::error,
    ocaml::{
        interop::{BoxRoot, ToOCaml},
        List, Runtime as OCamlRuntime, Value,
    },
    std::{sync::mpsc, thread},
};

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
        let (req_tx, req_rx) = mpsc::channel();
        let (res_tx, res_rx) = mpsc::channel();

        // handle dropped implicitly by not assigning detaches thread
        thread::spawn(move || {
            // initialise runtime *once* in a *single* thread
            let mut rt = ocaml::runtime::init();

            let requests = req_rx;
            let responses = res_tx;

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

    #[cfg(test)]
    pub fn dedup(&self, l: Vec<i32>) -> Result<Vec<i32>, Error> {
        self.request.send(Request::Dedup(l))?;
        self.response.recv()?.map(|res| match res {
            Response::Dedup(l) => l,
            _ => panic!("received different response kind to request"),
        })
    }

    #[cfg(test)]
    pub fn add_num(&self, a: String, b: String) -> Result<OCamlString, Error> {
        self.request.send(Request::AddNum((a, b)))?;
        self.response.recv()?.map(|res| match res {
            Response::AddNum(c) => c,
            _ => panic!("received different response kind to request"),
        })
    }

    pub fn load_files(&self, files: Vec<String>) -> Result<(OCamlString, Ast, Env), Error> {
        self.request.send(Request::LoadFiles(files))?;

        self.response.recv()?.map(|res| match res {
            Response::LoadFiles(ret) => ret,

            #[allow(unreachable_patterns)] // remove this once more variants are added
            _ => panic!("received different response kind to request"),
        })
    }
}

ocaml::import! {
    fn internal_util_dedup(l: List<Value>) -> Result<List<i32>, WrapperError>;

    fn internal_type_check_initial_env() -> Result<Value, WrapperError>;

    // val load_files : ?check:bool -> (Arg.key * Arg.spec * Arg.doc) list -> Type_check.Env.t -> string list -> (string * Type_check.tannot ast * Type_check.Env.t)
    fn internal_process_file_load_files(check: bool, options: List<Value>, env: Value, files: List<BoxRoot<String>>) -> Result<(OCamlString, Ast, Env), WrapperError>;

    pub fn internal_bindings_to_list(input: Value) -> Result<Value, WrapperError>;

    pub fn internal_bigint_to_string(input: Value) -> Result<OCamlString, WrapperError>;

    fn internal_add_num(a: String, b: String) -> Result<OCamlString, WrapperError>;
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

    LoadFiles(Vec<String>),
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
    AddNum(OCamlString),

    LoadFiles((OCamlString, Ast, Env)),
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

        Request::LoadFiles(files) => {
            let env = unsafe { internal_type_check_initial_env(rt)?? };

            let mut file_list = List::empty();

            for file in files {
                let file_rooted: BoxRoot<String> = file.to_boxroot(rt);

                file_list = unsafe { file_list.add(rt, &file_rooted) };
            }

            Ok(Response::LoadFiles(unsafe {
                internal_process_file_load_files(rt, false, List::empty(), env, file_list)??
            }))
        }
    }
}
