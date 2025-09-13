macro_rules! ok {
    () => {
        Ok(())
    };
}
pub(crate) use ok;

macro_rules! err {
    () => {
        Err(())
    };
}
pub(crate) use err;
