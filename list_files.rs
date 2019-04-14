// cargo-deps: glob="*"

extern crate glob;

use glob::glob;

fn main() {
    for entry in glob("./*.rs").expect("Failed to read glob pattern") {
        let f = entry.expect("Error");
        let f = f.to_str().expect("Error to_str()");
        println!("File: {}", f);
        /*
        match entry {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("{:?}", e),
        }
        */
    }
}
