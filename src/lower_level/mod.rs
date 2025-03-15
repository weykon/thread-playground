use std::{
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, Condvar, Mutex,
    },
    thread,
};

fn main() {
    atom2();
}

// 1. 原子操作(Atomics)
// 对应到不同的内存缓存中的关于ordering的设置
fn atom() {
    // https://en.wikipedia.org/wiki/Sequential_consistency
    let counter = AtomicUsize::new(0);
    counter.fetch_add(1, Ordering::SeqCst);
    // 假设双缓存下关于内存的一致性
    // 1. 读取缓存
    // 2. 读取内存
    // 3. 写入缓存
    // 4. 写入内存
    // 5. 刷新缓存
    // 6. 刷新内存

    // Acquire-Release Ordering
    // 确保在读取和写入操作之间的顺序

    let door = Arc::new(AtomicBool::new(false));
    let door_1 = door.clone();
    let door_2 = door.clone();

    let t_1 = std::thread::spawn(move || {
        println!("t_1 not even change door");
        door_1.store(true, Ordering::Release);
        let d_1 = door_1.load(Ordering::Acquire);
        println!("t_1 door is {}", d_1);
        door_1.store(false, Ordering::Release);
        door_1.store(true, Ordering::Release);
    });

    let t_2 = std::thread::spawn(move || {
        while !door_2.load(Ordering::Acquire) {
            // 忙等待
            println!("waiting door open");
        }
        println!("t_2 door is open");
    });

    t_1.join().unwrap();
    t_2.join().unwrap();
}

fn atom2() {
    let door = Arc::new(AtomicBool::new(false));
    let door_1 = door.clone();
    let door_2 = door.clone();

    let t2 = Arc::new(Mutex::new(None));

    let t2_clone_for_t1 = t2.clone();

    println!("主线程: 启动 t_2 线程");
    let t_2 = std::thread::spawn(move || {
        println!("t_2: 开始执行，存储线程句柄");
        {
            let mut t2 = t2.lock().unwrap();
            *t2 = Some(thread::current());
            println!("t_2: 线程句柄已存储");
        }

        while !door_2.load(Ordering::Acquire) {
            println!("waiting door open");
            thread::park();
            println!("t_2: 被唤醒，再次检查门");
        }
        println!("t_2: 门开了，继续执行");
    });

    println!("主线程: 启动 t_1 线程");
    let t_1 = std::thread::spawn(move || {
        println!("t_1: 开始执行");
        door_1.store(true, Ordering::Release);

        if let Some(t2) = t2_clone_for_t1.lock().unwrap().as_ref() {
            println!("t_1: 唤醒 t_2 线程");
            t2.unpark();
        } else {
            println!("t_1: 无法获取 t_2 线程句柄");
        }
        println!("t_1: 执行完成");
    });
    t_1.join().unwrap();
    t_2.join().unwrap();
    println!("主线程: 所有线程完成");
}

fn condvar() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));

    let pair_clone = pair.clone();

    let t1 = std::thread::spawn(move || {
        let (lock, cvar) = &*pair_clone;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
    });

    let t2 = std::thread::spawn(move || {
        let (lock, cvar) = &*pair;
        let mut started = lock.lock().unwrap();
        while !*started {
            // 如果进来了就wait cvar的改变

            // wait操作会:
            // 1. 释放互斥锁
            // 2. 阻塞当前线程
            // 3. 被唤醒后重新获取互斥锁
            started = cvar.wait(started).unwrap();
        }
    });
}

fn barrier() {}

fn rwlock() {}

// 信号量
fn semaphore() {}

fn once() {}

fn parking_lot() {}
