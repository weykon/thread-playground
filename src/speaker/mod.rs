use std::sync::mpsc::{self, Receiver, Sender};
struct Worker {
    name: String,
    tx: Sender<Message>,
    rx: Receiver<Message>,
}
struct Boss {
    name: String,
    tx: Sender<Message>,
    rx: Receiver<Message>,
}
fn main() {
    // 创建调度员通道 - 所有人向调度员说话
    let (dispatcher_tx, dispatcher_rx) = mpsc::channel::<Message>();

    // 创建工人的专属回复通道
    let (worker_reply_tx, worker_reply_rx) = mpsc::channel::<Message>();
    // 创建老板的专属回复通道
    let (boss_reply_tx, boss_reply_rx) = mpsc::channel::<Message>();

    let worker = Worker {
        name: "worker".to_string(),
        tx: dispatcher_tx.clone(), // 工人可以向调度员说话
        rx: worker_reply_rx,       // 工人只接收自己的消息
    };

    let boss = Boss {
        name: "boss".to_string(),
        tx: dispatcher_tx.clone(), // 老板可以向调度员说话
        rx: boss_reply_rx,         // 老板只接收自己的消息
    };

    // 调度员线程 - 接收所有消息并分发到对应的接收者
    std::thread::spawn(move || {
        while let Ok(message) = dispatcher_rx.recv() {
            match message.target {
                Target::Worker => worker_reply_tx.send(message).unwrap(),
                Target::Boss => boss_reply_tx.send(message).unwrap(),
                Target::All => {
                    // 广播给所有人
                    worker_reply_tx.send(message.clone()).unwrap();
                    boss_reply_tx.send(message).unwrap();
                }
            }
        }
    });
}

#[derive(Clone, Debug)]
enum Target {
    Worker,
    Boss,
    All,
}

#[derive(Clone, Debug)]
struct Message {
    content: String,
    sender: String,
    target: Target,
}
