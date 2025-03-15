use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

fn main() {
    let nthreads: usize = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(3);
    let (say, hear) = mpsc::channel::<Task>();

    let task_a = Arc::new(Mutex::new(Task {
        id: 1,
        data: vec![1, 2, 3],
    }));

    let task_b = Arc::new(Mutex::new(Task {
        id: 2,
        data: vec![4, 5, 6],
    }));

    let mut childs = Vec::new();
    for i in 0..nthreads {
        let say = say.clone();
        let task_a = task_a.clone();
        let task_b = task_b.clone();

        let child = thread::spawn(move || {
            let mut task_a = task_a.lock().unwrap();
            let mut task_b = task_b.lock().unwrap();

            task_a.data.push(i as u8);
            task_b.data.push(i as u8);

            say.send(task_a.clone()).unwrap();
            say.send(task_b.clone()).unwrap();
        });
        childs.push(child)
    }

    for i in childs {
        i.join().unwrap();
    }
}

#[derive(Debug, Clone)]
struct Task {
    id: usize,
    data: Vec<u8>,
}
