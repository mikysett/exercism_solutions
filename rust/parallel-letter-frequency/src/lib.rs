use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let freq = Arc::new(Mutex::new(HashMap::<char, usize>::new()));
    let mut handles = vec![];
    let chunk_size = input.len() / worker_count + 1;

    input.chunks(chunk_size).for_each(|chunk| {
        let chunk = chunk.join("");
        let freq = Arc::clone(&freq);

        let handle = thread::spawn(move || {
            let mut line_freq = HashMap::new();
            chunk
                .chars()
                .filter(|c| c.is_alphabetic())
                .flat_map(|c| c.to_lowercase())
                .for_each(|c| {
                    line_freq
                        .entry(c)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                });

            let mut freq = freq.lock().unwrap();
            for (k, v) in line_freq {
                freq.entry(k).and_modify(|count| *count += v).or_insert(v);
            }
        });
        handles.push(handle);
    });

    for handle in handles {
        handle.join().unwrap();
    }
    let freq = Arc::clone(&freq);
    let freq = freq.lock().unwrap();
    freq.clone()
}
