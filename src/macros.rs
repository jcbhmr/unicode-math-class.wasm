macro_rules! my_parallel_enum {
  ($from:path, $to:path, {
      $($variant:ident,)*
  }) => {
      impl From<$from> for $to {
          fn from(value: $from) -> Self {
              match value {
                  $( <$from>::$variant => <$to>::$variant, )*
              }
          }
      }

      impl From<$to> for $from {
          fn from(value: $to) -> Self {
              match value {
                  $( <$to>::$variant => <$from>::$variant, )*
              }
          }
      }
  };
}
