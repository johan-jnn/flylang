macro_rules! absolute_path {
    ($e:expr) => {
        std::path::absolute($e.clone()).unwrap_or($e.clone())
    };
}
pub(crate) use absolute_path;
