use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn counter_with_timeout() {
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  for i in 0..5 {
    let counter_clone = Arc::clone(&counter);
    let handle = thread::spawn(move || {
      match counter_clone.lock().map_err(|e| e.to_string()) {
        Ok(mut num) => {
          *num += 1;
          println!("Thread {} incremented the counter.", i);
        }
        Err(e) => {
          println!("Thread {} failed to acquire the lock: {}", i, e);
        }
      }
    });
    handles.push(handle);

    thread::sleep(Duration::from_millis(10));
  }

  for handle in handles {
    handle.join().unwrap();
  }

  println!("Final counter value: {}", *counter.lock().unwrap());
}
