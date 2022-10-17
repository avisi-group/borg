use sail::parser::load_files;

fn main() {
    color_eyre::install().unwrap();
    simple_logger::init_with_env().unwrap();

    dbg!();
    dbg!(load_files(vec![
        "/home/fm208/Documents/borealis/sail/examples/prelude.sail".to_owned()
    ])
    .unwrap());

    dbg!();
    dbg!(load_files(vec![
        "/home/fm208/Documents/borealis/sail/examples/prelude.sail".to_owned()
    ])
    .unwrap());
}
