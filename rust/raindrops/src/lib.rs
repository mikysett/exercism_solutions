pub fn raindrops(n: u32) -> String {
    match [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .iter()
        .filter_map(|(nb, s)| {
            if n % nb == 0 {
                Some(s.to_string())
            } else {
                None
            }
        })
        .collect::<String>()
    {
        s if s.is_empty() => n.to_string(),
        s => s,
    }
}
