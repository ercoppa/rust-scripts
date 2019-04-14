// cargo-deps: wait-timeout="0.2.0"

extern crate wait_timeout;

use std::io::Read;
use std::process::Command;
use std::str;
use std::time::Duration;
use wait_timeout::ChildExt;

fn print_buffer_by_line(buffer: &mut Vec<u8>, t: &str) {
    let lines = str::from_utf8(&buffer).unwrap();
    for line in lines.split("\n") {
        println!("{}: {}", t, line);
    }
}

fn print_cmd_output(child: &mut std::process::Child) {
    // standard output
    let stdout = child
        .stdout
        .as_mut()
        .expect("unable to open stdout of child");
    let mut buffer = Vec::new();
    let count = stdout.read_to_end(&mut buffer).unwrap();
    if count > 0 {
        print_buffer_by_line(&mut buffer, "stdout");
    }
    // standard error
    let stdout = child
        .stderr
        .as_mut()
        .expect("unable to open stderr of child");

    let mut buffer = Vec::new();
    let count = stdout.read_to_end(&mut buffer).unwrap();
    if count > 0 {
        print_buffer_by_line(&mut buffer, "stderr");
    }
}

fn main() {
    let mut child = Command::new("/bin/sh")
        .arg("-c")
        .arg("echo ciao && sleep 5 && sadsad")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    let one_sec = Duration::from_secs(10);
    let status_code = match child.wait_timeout(one_sec).unwrap() {
        Some(status) => {
            print_cmd_output(&mut child);
            status.code()
        }
        None => {
            // child hasn't exited yet
            child.kill().unwrap();
            print_cmd_output(&mut child);
            child.wait().unwrap().code()
        }
    };

    let _res = match status_code {
        Some(code) => println!("{}", code),
        None => println!("None"),
    };

    ()
}
