#[macro_export]
macro_rules! implx {
    (
        struct { $($field:ident : $ty:ty = $val:expr),* $(,)? }
        impl $trait:path {
            $($item:tt)*
        }
    ) => {
        {
            struct Anon {
                $($field: $ty),*
            }

            impl $trait for Anon {
                $($item)*
            }

            Anon {
                $($field: $val),*
            }
        }
    };
}
