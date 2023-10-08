use std::sync::{Arc, Mutex};
use std::thread;

// 死锁示例
pub fn dead_lock() {
    let lock1 = Arc::new(Mutex::new(0));
    let lock2 = Arc::new(Mutex::new(0));

    let lock1_ref = lock1.clone();
    let lock2_ref = lock2.clone();

    thread::spawn(move || {
        // 锁住 lock1
        let mut num = lock1_ref.lock().unwrap();
        *num = 5;

        // 尝试锁住 lock2，这里就会发生死锁
        let mut num = lock2_ref.lock().unwrap();
    });

    // 另一个线程
    // thread::spawn(move || {        // 这里报错
    //     // 锁住 lock2
    //     let mut num = lock2_ref.lock().unwrap();
    //     *num = 10;

    //     // 尝试锁住 lock1，这里也会发生死锁
    //     let mut num = lock1_ref.lock().unwrap();
    // });
}
