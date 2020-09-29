extern crate futures; // [dependencies] futures = "0.1"

use std::io::BufRead;
use std::thread;

use futures::sync::mpsc::{Sender, channel};
use futures::{Future, Stream, Sink};

fn main() {
    let mut worker = spawn_worker();

    let stdin = ::std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        worker = worker.send(Msg::Echo(line)).wait().unwrap();
    }

    println!("Bye!");
}

enum Msg {
    Echo(String),
}

fn spawn_worker() -> Sender<Msg> {
    let (tx, rx) = channel(1);
    thread::spawn(move || {
        rx.for_each(|msg| {
            match msg {
                Msg::Echo(msg) => println!("{} <3", msg),
            }
            Ok(())
        }).wait().unwrap()
    });
    tx
}
