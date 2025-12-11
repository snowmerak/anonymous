#[macro_export]
macro_rules! anon_struct {
    (derive($($derive:ident),*); $($field:ident : $ty:ty = $value:expr),* $(,)?) => {
        {
            #[derive($($derive),*)]
            struct Anon {
                $($field: $ty),*
            }
            Anon {
                $($field: $value),*
            }
        }
    };
}
