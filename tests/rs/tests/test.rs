use common::exports::typst_community::unicode_math_class::unicode_math_class::MathClass;
use common::instantiate;

mod common;

#[test]
fn revision_test() {
    let (bindings, mut store) = instantiate();
    let unicode_math_class = bindings.typst_community_unicode_math_class_unicode_math_class();

    assert_eq!(unicode_math_class.call_revision(&mut store).unwrap(), 15);
}

#[test]
fn class_test() {
    let (bindings, mut store) = instantiate();
    let unicode_math_class = bindings.typst_community_unicode_math_class_unicode_math_class();

    assert_eq!(
        unicode_math_class.call_class(&mut store, "0").unwrap(),
        Some(MathClass::Normal)
    );
}
