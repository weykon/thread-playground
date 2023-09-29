use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
mod my;
pub fn main() {
    println!("--------------THE BASE EXAMPLE------------------");

    // 拿一下当前机器的核数
    let nthreads: usize = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(3);
    // Channels have two endpoints: the `Sender<T>` and the `Receiver<T>`,
    // where `T` is the type of the message to be transferred
    // (type annotation is superfluous)
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut children = Vec::new();

    for id in 0..nthreads {
        // The sender endpoint can be copied
        let thread_tx = tx.clone();

        // Each thread will send its id via the channel
        let child = thread::spawn(move || {
            // The thread takes ownership over `thread_tx`
            // Each thread queues a message in the channel
            thread_tx.send(id as i32).unwrap();

            // Sending is a non-blocking operation, the thread will continue
            // immediately after sending its message
            println!("thread {} finished", id);
        });

        children.push(child);
    }

    // Here, all the messages are collected
    let mut ids = Vec::with_capacity(nthreads as usize);
    for _ in 0..nthreads {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there are no messages available
        ids.push(rx.recv());
    }

    // Wait for the threads to complete any remaining work
    for child in children {
        child.join().expect("oops! the child thread panicked");
    }

    // Show the order in which the messages were sent
    println!("{:?}", ids);

    println!("--------------THE BASE EXAMPLE------------------");
    println!("--------------------END-------------------------");

    my::exec();
}

// 我将那个拿机器核心数的函数放在了这里
// 他说明是有应该放在不是长期生命周期的位置
pub fn get_runable_core() -> usize {
    use std::thread;
    thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(3)
}
