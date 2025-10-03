use reqwest;

use crate::get_addr;

#[tokio::test]
async fn test_user_register_all_cases() -> anyhow::Result<()> {
    let addr = get_addr().await?;
    //TODO: 数据库写好后加一个用户名重复的测试
    const CASES: [&'static str; 4] = [
        r#"
        {
            "name": "valid_user",
            "password": "validpassword",
            "role": 1,
            "timezone": "Asia/Shanghai",
            "locale": "zh-CN"
        }
        "#,
        r#"
        {
            "name": "",
            "password": "validpassword",
            "role": 1,
            "timezone": "Asia/Shanghai",
            "locale": "zh-CN"
        }
        "#,
        r#"
        {
            "name": "valid_user",
            "password": "",
            "role": 1,
            "timezone": "Asia/Shanghai",
            "locale": "zh-CN"
        }
        "#,
        r#"
        {
            "name": "valid_user",
            "password": "validpassword",
            "role": 99,
            "timezone": "Asia/Shanghai",
            "locale": "zh-CN"
        }
        "#,
    ];
    const EXPECTATIONS: [(u16, &'static str); 4] = [
        (201, ""),
        (400, "用户名格式错误"),
        (400, "密码格式错误"),
        (400, "角色格式错误"),
    ];

    let client = reqwest::Client::new();

    let mut result = Vec::new();

    // 执行所有测试用例
    for (case, expectation) in CASES.iter().zip(EXPECTATIONS.iter()) {
        let response = client
            .post(&format!("http://{}/api/v1/users", addr.to_string()))
            .header("Content-Type", "application/json")
            .json(case)
            .send()
            .await?;
        let status = response.status().as_u16();
        let text = response.text().await?;
        if status != expectation.0 && text != expectation.1 {
            result.push((case, expectation));
        }
    }
    assert!(result.is_empty(), "测试失败的用例: {:?}", result);
    Ok(())
}
