#[macro_export]
// https://stackoverflow.com/questions/53866508/how-to-make-a-public-struct-where-all-fields-are-public-without-repeating-pub#comment94587233_53869723
macro_rules! pub_fields {
    {
        $(#[derive($macros:tt)])*
        struct $name:ident {
            $($field:ident: $t:ty,)*
        }
    } => {
        $(#[derive($macros)])*
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}
