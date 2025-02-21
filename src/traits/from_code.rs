pub trait StandardErrorFromCodeTrait {
    fn from_code(code: usize) -> Option<Self>
    where
        Self: Sized;
}
