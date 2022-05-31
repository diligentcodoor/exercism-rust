#[macro_export(local_inner_macros)]
macro_rules! hashmap {
    ( $( $k:expr => $v:expr,)* ) => {
        {
            let mut temp_hash = ::std::collections::HashMap::new();
            $(
                temp_hash.insert($k, $v);
            )*
            temp_hash
        }
    };
    ( $( $k:expr => $v:expr),* ) => {
        hashmap!($( $k => $v,)*);
    };

}
