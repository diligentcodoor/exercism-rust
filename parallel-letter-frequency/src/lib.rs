use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.len() == 0 {
        return HashMap::new();
    }
    let input = input.iter().map(|x| x.to_string()).collect::<Vec<String>>();
    let freq: Arc<Mutex<HashMap<char, usize>>> = Arc::new(Mutex::new(HashMap::new()));

    let handles = input
        .chunks((input.len() as f64 / worker_count as f64).ceil() as usize)
        .map(|input_chunk| {
            let input_chunk = Box::new(input_chunk.to_vec());
            let freq = Arc::clone(&freq);

            thread::spawn(move || {
                for word in input_chunk.iter() {
                    for c in word.to_lowercase().chars().filter(|c| c.is_alphabetic()) {
                        let mut freq = freq.lock().unwrap();
                        *freq.entry(c).or_insert(0) += 1;
                    }
                }
            })
        });
    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(freq)
        .ok()
        .unwrap()
        .into_inner()
        .ok()
        .unwrap()
}
