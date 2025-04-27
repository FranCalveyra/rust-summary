use crate::philosophers::philosophers::{Philosopher, Table};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

mod matrix;
mod merge_sort;
mod parallel_vector_sum;
mod philosophers;
mod race_conditions;
mod bank_account;
mod queue;
mod circular_buffer;

fn main() {

    race_conditions_main();
}

fn race_conditions_main() {
    let counter = race_conditions::Counter::new();
    let counter_ref = Arc::new(counter);
    thread::scope(|s|{
        for _ in 0..10 {
            s.spawn(|| counter_ref.clone().increment());
        }
    })

}

fn philosophers_main() {
    let table = Arc::new(Table::new(5));
    let philosophers = (0..5).map(|i| Philosopher::new(i, table.clone()));

    let handles: Vec<JoinHandle<()>> = philosophers
        .map(|philosopher| thread::spawn(move || philosopher.eat()))
        .collect();

    handles
        .into_iter()
        .for_each(|handle| handle.join().unwrap());
}
