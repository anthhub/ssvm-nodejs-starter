extern crate jsonxf;
extern crate time;
use std::fs::File;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
    println!("The Rust function say() received {}", s);
    let r = String::from("hello ");
    return r + s;
}

#[wasm_bindgen]
pub fn format_json_by_path(path: &str) -> String {
    println!("The path received {}", path);

    match format_json(path) {
        Err(e) => {
            println!("format_json error {}", e);
            return "".to_string();
        }
        Ok(_) => "".to_string(),
    }
}

fn format_json(path: &str) -> Result<(), String> {
    let indent = String::from("  ");

    let mut input: Box<dyn std::io::Read> = match File::open(path) {
        Ok(f) => Box::new(f),
        Err(e) => {
            let mut estr = String::from(path);
            estr.push_str(": ");
            estr.push_str(&e.to_string());
            return Err(estr);
        }
    };

    let tmp_path = path.to_string() + ".tmp";

    let mut output: Box<dyn std::io::Write> = match File::create(&tmp_path) {
        Ok(f) => Box::new(f),
        Err(e) => {
            let mut estr = String::from(tmp_path);
            estr.push_str(": ");
            estr.push_str(&e.to_string());
            return Err(estr);
        }
    };

    let mut xf = jsonxf::Formatter::pretty_printer();
    xf.indent = indent;
    let result = xf.format_stream(&mut input, &mut output);

    std::fs::rename(tmp_path, path).unwrap();

    match result {
        Err(e) => Err(e.to_string()),
        Ok(_) => Ok(()),
    }
}
