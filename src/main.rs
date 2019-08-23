use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

type State<'a> = Arc<Mutex<RefCell<HashMap<&'a str, i32>>>>;
type Resource = Arc<Mutex<u32>>;

fn init<'a>() -> (State<'a>, Resource, Resource) {
    (Arc::new(Mutex::new(RefCell::new(HashMap::<&str, i32>::new()))),
     Arc::new(Mutex::new(0)),
     Arc::new(Mutex::new(0)))
}

fn atomic<'a>(state: &mut State<'a>, key: &'a str, value: i32) {
    let state_lock = state.lock().unwrap();
    let mut state_t = RefCell::borrow_mut(&state_lock);
    let k = state_t.entry(&key).or_insert(0);
    *k += value;
}

fn work<'a>(mut state: &mut State<'a>, resource1: &Arc<Mutex<u32>>, resource2: &Arc<Mutex<u32>>) {
    loop {
        println!("thread:: {}", thread::current().name().unwrap());
        atomic(&mut state, "waiting", 1);
        let _resource1_lock = resource1.lock().unwrap();
        atomic(&mut state, "waiting", 1);
        {
            let _resource2_lock = &resource2.lock().unwrap();
            atomic(&mut state, "waiting", -2);
            atomic(&mut state, "tasks", 1);
        }
    }
}

fn deadlock() {
    let (mut state, resource1, resource2) = init();
    let mut handler = Vec::new();
    let mut state_t = Arc::clone(&state);
    let resource1_t = Arc::clone(&resource1);
    let resource2_t = Arc::clone(&resource2);
    let handle = thread::Builder::new().name("child".to_string()).spawn(move || {
        work(&mut state_t, &resource1_t, &resource2_t);
    }).unwrap();

    work(&mut state, &resource2, &resource1);
    handler.push(handle);
    for i in handler {
        i.join().unwrap();
    }
}

fn main() {
    deadlock();
}