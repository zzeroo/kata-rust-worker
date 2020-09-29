Workers and how to stop them

All ideas and the whole code was stolen from this beautiful [blog post].

# Create a basic worker

The task is to read the output line-by-line, sending these lines to another thread for processing (echoing the line back, with ❤️).

A possible solution is: [basic worker branch]

## Tipps

* read `lines()` from `::std::io::stdin()`
* create a `Msg` enum to handle the lines
* spawn a worker thread and process the `Msg`'s

# Stop a basic worker

When the user types stop, the worker (but not the program itself) should be halted.

[stop worker branch]

# Use `Drop` to stop a worker

The cleanest way to cancel something in Rust is to drop it. Stop the worker
by dropping the Sender.

Solution: [stop-worker-drop]

## Bonus: Make worker restartable

Find a solution to restart a stopped worker.

Solution: [restart-worker-drop]


# Async
## Async baseline

Solution: [async-baseline]

## Async with termination message

Solution: [async-termination-message]

## Async with drop

Solution: [async-drop]

## Async restart dropable worker

Find a solution to restart a stopped worker.

Solution: [async-drop-restart]





[blog post]: https://matklad.github.io/2018/03/03/stopping-a-rust-worker.html
[basic worker branch]: https://github.com/zzeroo/kata-rust-worker/tree/basic-worker
[stop worker branch]: https://github.com/zzeroo/kata-rust-worker/tree/stop-worker
[stop-worker-drop]: https://github.com/zzeroo/kata-rust-worker/tree/stop-worker-drop
[restart-worker-drop]: https://github.com/zzeroo/kata-rust-worker/tree/restart-worker-drop
[async-baseline]: https://github.com/zzeroo/kata-rust-worker/tree/async-baseline
[async-drop]:
[async-drop-restart]:
