mod common;
pub use common::*;

#[test]
pub fn revision_test() {
    let (bindings, mut store) = instantiate();
    let unicode_math_class = bindings.typst_community_unicode_math_class_crate();

    assert_eq!(unicode_math_class.call_revision(&mut store).unwrap(), 15);
}

#[test]
pub fn class_test() {
    let (bindings, mut store) = instantiate();
    let unicode_math_class = bindings.typst_community_unicode_math_class_crate();

    assert_eq!(
        unicode_math_class.call_class(&mut store, "0").unwrap(),
        Some(wit::MathClass::Normal)
    );
}
