use std::sync::{Arc, Mutex};
use std::thread;

pub fn arc_mutex() {
    let arc_data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let mut handles = Vec::new();

    for _ in 0..100 {
        let arc_data = arc_data.clone();
        handles.push(thread::spawn(move || {
            let thread_id = thread::current().id();
            println!("{thread_id:?} attempting to acquire lock...");
            if let Ok(mut guard) = arc_data.lock() {
                println!("{thread_id:?} acquired lock");
                for num in guard.iter_mut() {
                    *num += 1;
                }
                println!("{thread_id:?} dropped lock");
            };
        }));
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("Values are: {:?}", arc_data.lock().unwrap());
}
