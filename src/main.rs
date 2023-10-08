pub use thread_playground::*;

fn main() {
  dead_lock();
  tasks::exec();
}