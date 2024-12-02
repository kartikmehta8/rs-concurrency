use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn cache() {
  let cache = Arc::new(Mutex::new(HashMap::new()));
  let mut handles = vec![];

  for i in 0..10 {
    let cache_clone = Arc::clone(&cache);
    let handle = thread::spawn(move || {
      let mut map = cache_clone.lock().unwrap();
      map.insert(i, i * 2); // Insert key-value pair.
      println!("Thread {} added value: {}", i, i * 2);
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  println!("Cache: {:?}", *cache.lock().unwrap());
}
