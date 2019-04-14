use std::process::Command;

fn main() {
    let output = Command::new("/bin/bash")
        .arg("-c")
        .arg("ls && ls -l")
        .output()
        .expect("failed to execute process");

    let exit_code = output.status.code().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    //println!("stdout: {}", stdout);

    let lines: Vec<&str> = stdout.split('\n').collect();
    for line in lines {
        if line.len() > 0 {
            println!("line: {}", line);
        }
    }

    println!("status: {}", exit_code);
    println!("stderr: {}", stderr);

    assert!(output.status.success());
}
