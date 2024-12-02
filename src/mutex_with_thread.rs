use std::sync::{Arc, Mutex};
use std::thread;

pub fn mutex_with_thread() {
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  for _ in 0..1000 {
    let counter_clone = Arc::clone(&counter);
    let handle = thread::spawn(move || {
      let mut num = counter_clone.lock().unwrap();
      *num += 1;
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  println!("Final counter value: {}", *counter.lock().unwrap());
}
