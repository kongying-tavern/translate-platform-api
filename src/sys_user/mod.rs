use strum::EnumString;

mod utils;

pub use utils::*;

// REVIEW: 前端请求中我想校验用户主键是否合法，方法是解码uuid（前端看到的是8长的字符串）后得
//         到一个自增主键。如果这个自增主键小于最大用户数就认为合法，否则不合法。
const MAX_USER: u64 = 4096;

#[derive(Debug, Clone, PartialEq, EnumString)]
pub enum UserRole {
    Admin = 0,
    User = 1,
}
