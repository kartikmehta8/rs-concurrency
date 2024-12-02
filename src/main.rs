mod cache_system;
mod concurrent_logging_system;
mod counter_with_timeout;
mod mutex_with_thread;
mod simple_mutex;

fn main() {
    simple_mutex::simple_mutex();
    mutex_with_thread::mutex_with_thread();
    concurrent_logging_system::concurrent_logging_system();
    counter_with_timeout::counter_with_timeout();
    cache_system::cache();
}
