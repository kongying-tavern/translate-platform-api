use std::io;

pub type Result<T> = core::result::Result<T, RootErr>;

#[derive(Debug)]
pub enum RootErr {
    FromIO(io::Error),
    _Unknown,
}

impl From<io::Error> for RootErr {
    fn from(value: io::Error) -> Self {
        RootErr::FromIO(value)
    }
}
