macro_rules! my_parallel_enum {
    ($rs:path, $wit:path, {
        $($variant:ident,)*
    }) => {
        impl From<$rs> for $wit {
            fn from(value: $rs) -> Self {
                match value {
                    $( <$rs>::$variant => Self::$variant, )*
                }
            }
        }
        impl From<$wit> for $rs {
            fn from(value: $wit) -> Self {
                match value {
                    $( <$wit>::$variant => Self::$variant, )*
                }
            }
        }
    };
  }
pub(crate) use my_parallel_enum;
