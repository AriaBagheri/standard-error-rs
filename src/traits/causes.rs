pub trait StandardErrorCausesTrait {
    fn causes(&self) -> Option<&'static str>;
}
