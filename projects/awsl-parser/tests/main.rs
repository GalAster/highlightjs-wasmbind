mod gen;

#[test]
fn ready() {
    println!("it, works!")
}

#[test]
#[ignore]
fn gen_parser() {
    gen::gen_note_down();
}
