use tracing::debug;

pub fn check_option<T, F>(field: &Option<T>, check: F, field_name: &str, res: &mut String)
where
    T: AsRef<str>,
    F: Fn(&str) -> bool,
{
    if let Some(value) = field {
        if !check(value.as_ref()) {
            debug!("{}格式错误: {}", field_name, value.as_ref());
            res.push_str(&format!("{}格式错误\n", field_name));
        }
    }
}

pub fn check_field<T, F>(field: &T, check: F, field_name: &str, res: &mut String)
where
    T: AsRef<str>,
    F: Fn(&str) -> bool,
{
    if !check(field.as_ref()) {
        debug!("{}格式错误: {}", field_name, field.as_ref());
        res.push_str(&format!("{}格式错误\n", field_name));
    }
}
