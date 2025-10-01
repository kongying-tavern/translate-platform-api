use tracing::info;

// 导入模块
pub mod database;
pub mod entities;
pub mod services;

// 重新导出常用类型
pub use database::{DatabaseConfig, test_connection};
pub use services::UserService;

/// 翻译平台API库的核心运行函数
pub async fn run() -> anyhow::Result<()> {
    println!("翻译平台API库正在运行...");
    println!("Translate Platform API is running from lib!");

    let span = tracing::info_span!("初始化数据库连接");
    let _enter = span.enter();

    // 使用工厂模式初始化数据库
    let db = DatabaseConfig::new()
        .url("postgresql://translate_user:translate_password@localhost:5432/translate_platform")
        .max_connections(50)
        .min_connections(5)
        .try_init()
        .await?;

    // 测试数据库连接
    test_connection(&db).await?;

    // 初始化服务
    let user_service = UserService::new(db);

    // 示例：CRUD操作
    info!("演示CRUD操作");

    // Create - 创建用户
    user_service
        .create(
            "admin".to_string(),
            "password123".to_string(),
            1, // Admin角色
            "Asia/Shanghai".to_string(),
            "zh-CN".to_string(),
            0, // 系统创建
        )
        .await?;

    // Read - 读取用户
    let user = user_service.read(1).await?;
    info!("读取用户结果: {:?}", user);

    // Update - 更新用户
    user_service
        .update(
            1,
            Some("new_admin".to_string()),
            None,
            None,
            None,
            None,
            0, // 更新者ID
        )
        .await?;

    // Delete - 删除用户
    user_service.delete(1, 0).await?;

    Ok(())
}
