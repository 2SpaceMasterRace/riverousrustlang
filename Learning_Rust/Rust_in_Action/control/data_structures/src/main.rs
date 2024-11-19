use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    let on_stack = 10;
    let on_heap = Box::new(20);
    let on_heap_with_rc = Rc::new(Box::new(30));
    let atomic_rc_mutex_lock = Arc::new(Mutex::new(40));
    println!(
        "a: {:?}, b: {:?}, c: {:?}, d: {:?}",
        on_stack, on_heap, on_heap_with_rc, atomic_rc_mutex_lock
    );
}
