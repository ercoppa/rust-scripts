use std::process::Command;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::io::{self, Write};

fn main() {
    let now = Instant::now();

    // run some process
    let status = Command::new("/bin/bash")
        .arg("-c")
        .arg("echo RUNNING && sleep 10 && echo DONE")
        .status()
        .expect("failed to execute process");

    // print elapsed time
    println!("Elapsed time: {} ms", now.elapsed().as_millis());
}
