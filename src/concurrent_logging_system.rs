use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn concurrent_logging_system() {
  let file = Arc::new(Mutex::new(
    OpenOptions::new().append(true).create(true).open("log.txt").unwrap(),
  ));
  let mut handles = vec![];

  for i in 0..5 {
    let file_clone = Arc::clone(&file);
    let handle = thread::spawn(move || {
      let mut file = file_clone.lock().unwrap();
      writeln!(file, "Thread {} is logging", i).unwrap();
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  println!("Logging complete.");
}
