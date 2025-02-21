pub trait StandardErrorDescriptionTrait {
    fn description(&self) -> Option<&'static str>;
}
