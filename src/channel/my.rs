use super::get_runable_core;
use chrono::prelude::*;

pub fn exec() {
    let running_cores = get_runable_core();
    use std::sync::mpsc;
    use std::thread;

    let mut thread_handles = Vec::new();

    // 是一个汇总的实例，用于接收其他线程的消息
    let (tx, rx) = mpsc::channel::<String>();

    for core in 0..running_cores {
        // 每个迭代将tx克隆一份，用move传递给每一个线程
        let tx = tx.clone();

        let thread_handle = thread::spawn(move || {
            let time = Local::now();
            let message = time.to_string();

            tx.send(format!(
                "Hello from core <<<\"{}\">>> : I want to say {}",
                core, message
            ))
            .unwrap();
        });
        thread_handles.push(thread_handle);
    }

    let mut storage_message = Vec::new();
    for _ in 0..running_cores {
        storage_message.push(rx.recv());
    }

    for child in thread_handles {
        child.join().expect("Oh No! child thread panicked");
    }

    for message in storage_message {
        match message {
            Ok(x) => println!("message: {}", x),
            Err(e) => println!("error: {:?}", e),
        }
    }
}
