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
    crate::error::Error,
    log::{error, trace},
    ocaml::Runtime as OCamlRuntime,
    once_cell::sync::Lazy,
    parking_lot::Mutex,
    std::{any::Any, sync::mpsc, thread},
};

/// Global runtime shared by all public functions
pub static RT: Lazy<Mutex<Runtime>> = Lazy::new(|| Mutex::new(Runtime::new()));

/// Default runtime thread stack size
///
/// Converting from the ARM AST takes 4-8 MiB and resulted in overflowing the default 2MiB stack size.
pub const DEFAULT_RUNTIME_THREAD_STACK_SIZE: usize = 64 * 1024 * 1024; // 64MiB

/// Closure that may be executed on the runtime
pub type ExecutableFunction<T> = Box<dyn FnOnce(&mut OCamlRuntime) -> T + Send + Sync>;

/// Dynamic type
pub type BoxAny = Box<dyn Any + Send + Sync>;

/// Wrapper around OCaml runtime
///
/// Workaround for <https://github.com/zshipko/ocaml-rs/issues/102> where ocaml::Runtime must be
/// owned and interacted with from a single thread or there are intermittent
/// memory issues.
pub struct Runtime {
    request: mpsc::Sender<ExecutableFunction<BoxAny>>,
    response: mpsc::Receiver<BoxAny>,
}

impl Runtime {
    /// Instantiate a new Runtime with corresponding worker thread
    pub fn new() -> Self {
        trace!("Creating new OCaml runtime handler");
        let (req_tx, req_rx) = mpsc::channel::<ExecutableFunction<BoxAny>>();
        let (res_tx, res_rx) = mpsc::channel::<BoxAny>();

        let builder = thread::Builder::new().stack_size(DEFAULT_RUNTIME_THREAD_STACK_SIZE);

        // handle dropped implicitly by not assigning detaches thread
        builder.spawn(move || {
            // initialise runtime *once* in a *single* thread
            let mut rt = ocaml::runtime::init();

            let requests = req_rx;
            let responses = res_tx;

            trace!("Initialised OCaml runtime handler thread, looping...");

            // loop indefinitely processing requests
            loop {
                // block on receiving a request
                match requests.recv() {
                    // if successful, execute request
                    Ok(request) => {
                        let response = request(&mut rt);

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
        }).expect("Failed to spawn runtime thread");

        Self {
            request: req_tx,
            response: res_rx,
        }
    }

    /// Execute a closure on the runtime thread
    pub fn execute<
        F: FnOnce(&mut OCamlRuntime) -> RET + Send + Sync + 'static,
        RET: Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> Result<RET, Error> {
        // coerce the concrete return type (RET) into `Box<dyn Any>`
        let boxed_return: ExecutableFunction<BoxAny> =
            Box::new(move |rt: &mut OCamlRuntime| Box::new(f(rt)));

        // send closure
        self.request.send(boxed_return)?;

        // receive Box<dyn Any>
        let boxed = self.response.recv()?;

        // downcast
        Ok(*boxed.downcast().expect("downcasting to return type"))
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}
