use pest_generator::derive_parser;
use quote::quote;
use std::{fs::File, io::prelude::*, path::Path};

pub fn gen_note_down() {
    let pest = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/awsl.pest"));
    let rs = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/awsl.rs"));

    let derived = {
        let path = pest.to_string_lossy();
        let pest = quote! {
            #[grammar = #path]
            pub struct AwslParser;
        };
        derive_parser(pest, false)
    };
    let mut file = File::create(rs).unwrap();
    let out = format!("pub struct AwslParser;{}", derived);
    writeln!(file, "{}", out).unwrap();
}
