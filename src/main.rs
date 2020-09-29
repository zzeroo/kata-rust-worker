extern crate futures; // [dependencies] futures = "0.1"

use std::io::BufRead;
use std::thread;

use futures::sync::mpsc::{channel, Sender};
use futures::{Future, Sink, Stream};

fn main() {
    let mut worker = Some(spawn_worker());

    let stdin = ::std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line == "stop" {
            drop(worker.take());
            continue;
        } else if line == "start" {
            worker = Some(spawn_worker());
            continue;
        };


        if let Some(w) = worker {
            worker = Some(w.send(Msg::Echo(line)).wait().unwrap())
        } else {
            println!("The worker has been stopped!");
        }
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
        }).map(|()| {
            println!("The worker has stopped!");
        }).wait().unwrap();
    });
    tx
}
