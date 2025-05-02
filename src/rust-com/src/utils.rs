#[macro_export]
macro_rules! hashmap {
    ( $( $key:expr => $val:expr ),* $(,)? ) => {{
        let mut map = std::collections::HashMap::new();
        $(
            map.insert($key, $val);
        )*
        map
    }};
}
