use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    thread::scope(|s| {
        let mut handles = Vec::with_capacity(worker_count);
        let chunk_size = input.len() / worker_count + 1;

        input.chunks(chunk_size).for_each(|chunk| {
            handles.push(s.spawn(|| counter(chunk)));
        });

        let mut freq = HashMap::new();
        for handle in handles {
            handle.join().unwrap().into_iter().for_each(|(k, v)| {
                *freq.entry(k).or_default() += v;
            });
        }
        freq
    })
}

fn counter(input: &[&str]) -> HashMap<char, usize> {
    let mut line_freq = HashMap::new();
    input.iter().for_each(|s| {
        s.chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_ascii_lowercase())
            .for_each(|c| {
                line_freq
                    .entry(c)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            });
    });
    line_freq
}
