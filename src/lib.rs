mod bindings;
mod my_macros;

use my_macros::my_parallel_enum;
use unicode_math_class;

pub struct Component;
impl bindings::exports::typst_community::unicode_math_class::types::Guest for Component {
    fn revision() -> u8 {
        unicode_math_class::REVISION
    }
    fn class(
        c: String,
    ) -> Option<bindings::exports::typst_community::unicode_math_class::types::MathClass> {
        let c = c.chars().next().expect("non-empty string");
        unicode_math_class::class(c).map(|x| x.into())
    }
}

my_parallel_enum!(unicode_math_class::MathClass, bindings::exports::typst_community::unicode_math_class::types::MathClass, {
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
