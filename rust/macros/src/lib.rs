#[macro_export]
macro_rules! hashmap {
    () => {{
        crate::HashMap::new()
    }};

    ($($k:expr => $v:expr),+ $(,)?) => {{
        use crate::HashMap;
        let mut hm = HashMap::new();

        $(
            hm.insert($k, $v);
        )+

        hm
    }};
}
