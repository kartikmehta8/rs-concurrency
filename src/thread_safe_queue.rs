use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn queue() {
  let queue = Arc::new(Mutex::new(Vec::new()));
  let queue_clone = Arc::clone(&queue);

  // Producer threads.
  for i in 0..5 {
    let queue_clone = Arc::clone(&queue);
    thread::spawn(move || {
      let mut q = queue_clone.lock().unwrap();
      q.push(i);
      println!("Produced: {}", i);
    });
  }

  // Consumer thread.
  let consumer = thread::spawn(move || {
    loop {
      let mut q = queue_clone.lock().unwrap();
      if let Some(item) = q.pop() {
        println!("Consumed: {}", item);
      } else {
        break;
      }
      thread::sleep(Duration::from_millis(100));
    }
  });

  consumer.join().unwrap();
}
