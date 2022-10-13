use crate::RT;

ocaml::import! {
    fn internal_load_files();
}

pub fn load_files() {
    let rt = &*RT;
    unsafe {
        internal_load_files(rt).unwrap();
    }
}
