#[macro_export]
macro_rules! enum_from_mirror {
    ($local_type:ty, $foreign_type:ty, { $($variant:ident),+ $(,)? }) => {
        impl From<$local_type> for $foreign_type {
            fn from(theme: $local_type) -> Self {
                match theme {
                    $( <$local_type>::$variant => <$foreign_type>::$variant, )+
                }
            }
        }

        impl From<$foreign_type> for $local_type {
            fn from(pref: $foreign_type) -> Self {
                match pref {
                    $( <$foreign_type>::$variant => <$local_type>::$variant, )+
                }
            }
        }
    };
}
