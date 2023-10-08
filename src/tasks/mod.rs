use std::{
    sync::{
        mpsc::{channel, Sender},
        Arc, Mutex,
    },
    thread,
};

enum Task {
    SearchPath(String),
    Search(String),
}

struct State {
    pub tasks: Vec<Arc<Mutex<Task>>>,
    pub next_id: u64,
}

// create mutex for the commom value,
// State.tasks[] to keep Task path and result
// threadd - pool -
// thread for ezch task walk dir then create task then exec task by thread
// then suc return the task result
pub fn exec() {
    let nthreads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(3);
    let (tx, tr) = channel();
    let mut handles = Vec::new();

    let mut state = State {
        tasks: Vec::new(),
        next_id: 0,
    };

    let fake_paths = vec!["./src".to_owned(), "./".to_owned()];
    let fake_paths_iter = fake_paths.iter();

    for s in fake_paths_iter {
        let arc_mutex_task = Arc::new(Mutex::new(Task::SearchPath(s.clone())));
        state.tasks.push(arc_mutex_task);
    }

    for _ in 0..nthreads {
        let tx: Sender<Task> = tx.clone();
        let handle = thread::spawn(move || {
            
        });
        handles.push(handle);
    }
}

fn main() { 
    exec();
}