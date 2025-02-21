use std::borrow::Cow;

pub trait StandardErrorMessageTrait {
    fn message(&self) -> Cow<str>;
}
