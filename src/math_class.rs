use crate::bindings::exports::typst_community::unicode_math_class::crate_ as wit;
use unicode_math_class as rs;

impl From<rs::MathClass> for wit::MathClass {
    fn from(value: rs::MathClass) -> Self {
        match value {
            rs::MathClass::Normal => Self::Normal,
            rs::MathClass::Alphabetic => Self::Alphabetic,
            rs::MathClass::Binary => Self::Binary,
            rs::MathClass::Closing => Self::Closing,
            rs::MathClass::Diacritic => Self::Diacritic,
            rs::MathClass::Fence => Self::Fence,
            rs::MathClass::GlyphPart => Self::GlyphPart,
            rs::MathClass::Large => Self::Large,
            rs::MathClass::Opening => Self::Opening,
            rs::MathClass::Punctuation => Self::Punctuation,
            rs::MathClass::Relation => Self::Relation,
            rs::MathClass::Space => Self::Space,
            rs::MathClass::Unary => Self::Unary,
            rs::MathClass::Vary => Self::Vary,
            rs::MathClass::Special => Self::Special,
        }
    }
}
impl From<wit::MathClass> for rs::MathClass {
    fn from(value: wit::MathClass) -> Self {
        match value {
            wit::MathClass::Normal => Self::Normal,
            wit::MathClass::Alphabetic => Self::Alphabetic,
            wit::MathClass::Binary => Self::Binary,
            wit::MathClass::Closing => Self::Closing,
            wit::MathClass::Diacritic => Self::Diacritic,
            wit::MathClass::Fence => Self::Fence,
            wit::MathClass::GlyphPart => Self::GlyphPart,
            wit::MathClass::Large => Self::Large,
            wit::MathClass::Opening => Self::Opening,
            wit::MathClass::Punctuation => Self::Punctuation,
            wit::MathClass::Relation => Self::Relation,
            wit::MathClass::Space => Self::Space,
            wit::MathClass::Unary => Self::Unary,
            wit::MathClass::Vary => Self::Vary,
            wit::MathClass::Special => Self::Special,
        }
    }
}
