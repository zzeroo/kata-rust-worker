extern crate futures; // [dependencies] futures = "0.1"

use std::io::BufRead;
use std::thread;

use futures::sync::mpsc::{channel, Sender};
use futures::{Future, Sink, Stream};

fn main() {
    let mut worker = spawn_worker();

    let stdin = ::std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let msg = if line == "stop" {
            Msg::Stop
        } else {
            Msg::Echo(line)
        };
        worker = worker.send(msg).wait().unwrap();
    }

    println!("Bye!");
}

enum Msg {
    Echo(String),
    Stop,
}

fn spawn_worker() -> Sender<Msg> {
    let (tx, rx) = channel(1);
    thread::spawn(move || {
        let _ = rx
            .for_each(|msg| match msg {
                Msg::Echo(msg) => {
                    println!("{} <3", msg);
                    Ok(())
                }
                Msg::Stop => Err(()),
            })
            .then(|result| {
                println!("The worker has stopped!");
                result
            })
            .wait();
    });
    tx
}
