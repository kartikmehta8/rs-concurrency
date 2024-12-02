use std::sync::Mutex;

pub fn simple_mutex() {
  let data = Mutex::new(0);

  {
    // Acquire the lock. This will block the current thread until the lock is available.
    let mut num = data.lock().unwrap();

    // Modify the data.
    *num += 1;
  } // Lock is released here automatically.

  println!("Data after modification: {:?}", data);
}
