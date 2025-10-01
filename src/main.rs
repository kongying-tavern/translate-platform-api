use translate_platform_api::{run, get_version, get_app_name};

fn main() {
    println!("启动 {} v{}", get_app_name(), get_version());
    println!("正在从main.rs调用库函数...");
    
    // 调用库中的run函数
    run();
    
    println!("应用程序启动完成！");
}
