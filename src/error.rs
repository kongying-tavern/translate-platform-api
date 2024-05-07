use super::user::register;
use std::io;

pub type Result<T> = core::result::Result<T, RootErr>;

#[derive(Debug)]
pub enum RootErr {
    FromIO(io::Error),
    FromUser(register::RegErr),
    _Unknown,
}

impl From<io::Error> for RootErr {
    fn from(value: io::Error) -> Self {
        RootErr::FromIO(value)
    }
}

impl From<register::RegErr> for RootErr {
    fn from(value: register::RegErr) -> Self {
        RootErr::FromUser(value)
    }
}
