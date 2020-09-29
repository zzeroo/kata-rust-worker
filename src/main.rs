use std::io::BufRead;
use std::sync::mpsc::{channel, Sender};
use std::thread;

fn main() {
    let mut worker = Some(spawn_worker());

    let stdin = ::std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line == "stop" {
            drop(worker.take());
            continue;
        };

        if let Some(ref worker) = worker {
            worker.send(Msg::Echo(line)).unwrap();
        } else {
            println!("The worker has been stopped!");
        };
    }

    println!("Bye!");
}

enum Msg {
    Echo(String),
}

fn spawn_worker() -> Sender<Msg> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            match msg {
                Msg::Echo(msg) => println!("{} <3", msg),
            }
        }
        println!("The worker has stopped");
    });

    tx
}
