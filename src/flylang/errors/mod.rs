use std::{
    fmt::{Debug, Display},
    process::exit,
};

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorType {
    Warn,
    Stop,
    Hint,
}
use ErrorType::*;
impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Hint => write!(f, "Hint"),
            Stop => write!(f, "Error"),
            Warn => write!(f, "Warning"),
        }
    }
}

pub trait RaisableErr {
    fn _kind(&self) -> ErrorType;
    fn _code(&self) -> i32 {
        1
    }

    fn _message(&self) -> String {
        format!("{}", self._kind())
    }
    fn print(&self) {
        println!("{} ({}):\n{}", self._kind(), self._code(), self._message())
    }
    fn kill(&self) -> ! {
        #[cfg(debug_assertions)]
        {
            panic!();
        }
        #[cfg(not(debug_assertions))]
        {
            exit(self._code())
        }
    }
    fn raise(&self) -> ! {
        self.print();
        self.kill();
    }
    /// Raise the error only if its `_kind()` if of type `Stop`.
    /// Else, it just prints to the console.
    fn controlled_raise(&self) {
        if self._kind() == ErrorType::Stop {
            self.raise();
        } else {
            self.print();
        };
    }
}

impl Debug for dyn RaisableErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (exit with code {})", self._message(), self._code())
    }
}

pub type LangResult<R> = Result<R, Box<dyn RaisableErr>>;

macro_rules! lang_err {
    ($e:expr) => {
        Err(Box::new($e))
    };
}
pub(crate) use lang_err;
