cargo_component_bindings::generate!();
use bindings::exports::typst_community::unicode_math_class::unicode_math_class as wit;
use unicode_math_class as rs;

mod math_class;

struct Component;

impl wit::Guest for Component {
    fn revision() -> u8 {
        rs::REVISION
    }
    fn class(c: String) -> Option<wit::MathClass> {
        let c = c.chars().next().unwrap();
        rs::class(c).map(|x| x.into())
    }
}
