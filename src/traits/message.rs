use std::borrow::Cow;

pub trait ErrorMessageTrait {
    fn message(&self) -> Cow<str>;
}
