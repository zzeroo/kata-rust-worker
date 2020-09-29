use std::io::BufRead;
use std::sync::mpsc::{Sender, channel};
use std::thread;


fn main() {
    let worker = spawn_worker();

    let stdin = ::std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        worker.send(Msg::Echo(line))
            .unwrap();
    }

    println!("Bye!");
}

enum Msg {
    Echo(String),
}

fn spawn_worker() -> Sender<Msg> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        loop {
            let msg = rx.recv().unwrap();
            match msg {
                Msg::Echo(msg) => println!("{} ❤️", msg),
            }
        }
    });

    tx
}
