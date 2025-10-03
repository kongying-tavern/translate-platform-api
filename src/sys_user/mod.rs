use strum::EnumString;

mod utils;

pub use utils::*;

#[derive(Debug, Clone, PartialEq, EnumString)]
pub enum UserRole {
    Admin = 0,
    User = 1,
}