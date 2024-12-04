pub trait Inspector {
    fn inspect(&self) -> Result<bool, String>;
}
