use decoder_harness::ffi::new_decoder;

fn main() {
    println!("{:#?}", new_decoder().into_raw());
}
