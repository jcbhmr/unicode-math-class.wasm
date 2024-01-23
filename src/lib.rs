#[macro_use]
mod macros;

cargo_component_bindings::generate!();
use bindings as wit;
use unicode_math_class as rs;

pub struct Component;
impl wit::Guest for Component {
    fn revision() -> u8 {
        rs::REVISION
    }
    fn class(c: String) -> Option<wit::MathClass> {
        let c = c.chars().next().unwrap();
        rs::class(c).map(|x| x.into())
    }
}

my_parallel_enum!(rs::MathClass, wit::MathClass, {
    Normal,
    Alphabetic,
    Binary,
    Closing,
    Diacritic,
    Fence,
    GlyphPart,
    Large,
    Opening,
    Punctuation,
    Relation,
    Space,
    Unary,
    Vary,
    Special,
});
