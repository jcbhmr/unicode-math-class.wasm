mod my_macros;

cargo_component_bindings::generate!();
use my_macros::rs_wit_parallel_enum;
use unicode_math_class;

pub struct Component;
impl bindings::Guest for Component {
    fn revision() -> u8 {
        unicode_math_class::REVISION
    }
    fn class(c: String) -> Option<bindings::MathClass> {
        let c = c.chars().next().expect("non-empty string");
        unicode_math_class::class(c).map(|x| x.into())
    }
}

rs_wit_parallel_enum!(unicode_math_class::MathClass, bindings::MathClass, {
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
