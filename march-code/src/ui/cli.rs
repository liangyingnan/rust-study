use std::io::{self, Write};

/// 命令行界面
pub struct CliInterface;

impl CliInterface {
    /// 创建新的CLI界面
    pub fn new() -> Self {
        CliInterface
    }

    /// 获取用户输入
    pub fn get_user_input(&self, prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("无法读取输入");
        input
    }

    /// 显示主菜单
    pub fn display_menu(&self) {
        println!("\n任务管理系统 - 主菜单");
        println!("1. 添加任务");
        println!("2. 列出所有任务");
        println!("3. 更新任务状态");
        println!("4. 删除任务");
        println!("5. 查看任务详情");
        println!("q. 退出程序");
    }

    /// 显示状态选项
    pub fn display_status_options(&self) {
        println!("可用的状态选项：");
        println!("1. 待办");
        println!("2. 进行中");
        println!("3. 已完成");
    }
} 