use sel4_userspace::runtime::lib::mutex::Mutex;
use std::sync::Arc;

#[test]
fn new() {
    let data: usize = 100;
    let mutex = Mutex::new(data);
    drop(mutex)
}

#[test]
fn lock_single() {
    let data: usize = 100;
    let mutex = Mutex::new(data);

    let mutex_guard = mutex.lock();
    match mutex_guard {
        Ok(mut data) => {
            *data += 1;
        }
        Err(_) => {}
    }
}

#[test]
fn lock_multiple() {
    let mutex: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    const NUM_THREADS: usize = 50;
    const NUM_ITERATIONS: usize = 10_000;
    let mut threads = Vec::with_capacity(NUM_THREADS);

    for _ in 0..NUM_THREADS {
        let mutex_ref: Arc<Mutex<usize>> = mutex.clone();

        threads.push(std::thread::spawn(move || {
            for _ in 0..NUM_ITERATIONS {
                let mut counter = (*mutex_ref).lock().unwrap();
                *counter += 1;
            }
        }));
    }

    threads.into_iter().for_each(|t| t.join().unwrap());

    assert_eq!(*mutex.lock().unwrap(), NUM_THREADS * NUM_ITERATIONS);
}
