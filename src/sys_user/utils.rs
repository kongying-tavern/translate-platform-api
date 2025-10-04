use anyhow::{Result, anyhow};
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use once_cell::sync::Lazy;
use sqids::Sqids;
use tokio::task;

pub const VALID_PASSWORDS: [&str; 4] = [
    "Abcdef12", // 合规密码，无特殊字符
    "Abcdef1!", // 合规密码，有特殊字符
    "A1bcdefg", // 合规密码，无特殊字符
    "1Abcdefg", // 合规密码，无特殊字符
];

pub const INVALID_PASSWORDS: [&str; 4] = [
    "abcdef12", // 缺少大写字母
    "ABCDEF12", // 缺少小写字母
    "Abcdefgh", // 缺少数字
    "Ab1",      // 长度不足
];

static ARGON2: Lazy<Argon2<'static>> = Lazy::new(|| Argon2::default());

/// 使用argon2对密码进行哈希
pub async fn hash_password(password: String) -> Result<String> {
    let hashed_password = task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        // 这个argon2的Result类型很奇怪啊，没法直接用?
        ARGON2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| anyhow!("Password hashing failed: {}", e))
    })
    .await?;

    // 4. 返回哈希值，准备存储到数据库
    Ok(hashed_password?)
}

/// 构建唯一表示符生成器
/// 默认字母混合，可能有问题
pub static SQIDS: Lazy<Sqids> = Lazy::new(|| Sqids::builder().min_length(8).build().unwrap());

/// 从数据库自增id到混淆id
pub fn encode_id(id: i64) -> Result<String> {
    SQIDS
        .encode(&[id as u64])
        .map_err(|e| anyhow!("ID编码失败, {e}"))
}

/// 从混淆id到数据库自增id
pub fn decode_id(sid: &str) -> Result<i32> {
    SQIDS
        .decode(sid)
        .get(0)
        .map(|&id| id as i32)
        .ok_or_else(|| anyhow!("ID解码失败"))
}

pub use verify::*;

mod verify {
    use crate::sys_user::{MAX_USER, SQIDS};

    /// 主键检查
    /// 基于sqids解码后的自增主键小于最大支持用户数
    pub fn check_id(id: &str) -> bool {
        SQIDS.decode(id).get(0).map_or(MAX_USER + 1, |f| *f) < MAX_USER
    }

    /// 密码检查
    /// 长度8-32，必须大写字母、小写字母、数字三者同时存在，可以有!@#$%^&*
    pub fn check_password(password: &str) -> bool {
        password.len() >= 8
            && password.len() <= 32
            && password.chars().any(|c| c.is_uppercase())
            && password.chars().any(|c| c.is_lowercase())
            && password.chars().any(|c| c.is_digit(10))
            && password
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || "!@#$%^&*".contains(c))
    }

    /// 用户名检查
    /// 长度8-32，只能包含字母、数字、下划线，且必须以字母开头
    pub fn check_name(username: &str) -> bool {
        username.len() >= 8
            && username.len() <= 32
            && username
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_')
            && username
                .chars()
                .next()
                .map(|c| c.is_ascii_alphabetic())
                .unwrap_or(false)
    }

    /// 角色检查
    /// 目前仅允许0（管理员）和1（普通用户）
    /// 未来可能会扩展更多角色
    pub fn check_role(role: &str) -> bool {
        matches!(role, "0" | "1")
    }

    /// 偏好语言检查
    /// TODO: 之后给一个表表示允许的语言
    pub fn check_lang(lang: &str) -> bool {
        match lang.parse::<unic_langid::LanguageIdentifier>() {
            Ok(lang) => lang.language != "und" && lang.script.is_none() && lang.region.is_some(),
            Err(_) => false,
        }
    }

    /// 偏好时区检查
    pub fn check_tz(tz: &str) -> bool {
        match tz.parse::<chrono_tz::Tz>() {
            Ok(tz) => tz.name() != "Etc/UTC",
            Err(_) => false,
        }
    }
}
