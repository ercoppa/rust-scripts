// cargo-deps: serde_json="*"

extern crate serde_json;

use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let csv_file = "data/test.json";
    let f = File::open(csv_file).expect("Cannot open file");
    let reader = BufReader::new(f);

    let data: HashMap<String, Value> = serde_json::from_reader(reader).expect("Cannot parse JSON");

    //println!("Data: {:?}", data);

    for (key, value) in data {
        print!("{}: ", key.as_str());
        if key == "phones" {
            println!();
            let value: &Vec<Value> = value.as_array().unwrap();
            for t in value {
                println!("\tTelephone: {}", t.as_str().unwrap());
            }
        } else {
            match value {
                Value::Number(n) => println!("{:?}", n.as_u64().unwrap()),
                Value::String(n) => println!("{}", n.as_str()),
                _ => println!("Else {:?}", value),
            }
        }
    }
}
