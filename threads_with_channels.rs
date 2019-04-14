use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let mut pool = Vec::new();
    let pool_count = 3;

    for _q in 0..pool_count {
        let tx_t = mpsc::Sender::clone(&tx);
        let handle = thread::spawn(move || {
            let limit = 1000;
            let mut sum = 0;
            for i in 0..limit {
                for j in 0..limit {
                    for k in 0..limit {
                        sum = sum + (i * j * k);
                        sum = sum >> 4;
                    }
                }
            }
            println!("Result: {}", sum);
            tx_t.send(sum).unwrap();
        });
        pool.push(handle);
    }

    let mut total = 0;
    for _q in 0..pool_count {
        let v = rx.recv().unwrap();
        total += v;
    }
    println!("Total: {}", total);
}
