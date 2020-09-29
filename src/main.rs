use std::io::BufRead;
use std::sync::mpsc::{channel, Sender};
use std::thread;

fn main() {
    let worker = spawn_worker();

    let stdin = ::std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let msg = if line == "stop" {
            Msg::Stop
        } else {
            Msg::Echo(line)
        };

        worker.send(msg).unwrap();
    }

    println!("Bye!");
}

enum Msg {
    Echo(String),
    Stop,
}

fn spawn_worker() -> Sender<Msg> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        loop {
            let msg = rx.recv().unwrap();
            match msg {
                Msg::Echo(msg) => println!("{} <3", msg),
                Msg::Stop => break,
            }
        }
        println!("The worker has stopped");
    });

    tx
}
