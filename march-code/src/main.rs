use std::io;
use std::env;

mod models;
mod tasks;
mod ui;
mod utils;

use models::task::{Task, TaskStatus};
use tasks::task_manager::TaskManager;
use ui::cli::CliInterface;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut task_manager = TaskManager::new();
    let cli = CliInterface::new();

    if args.len() > 1 {
        // 命令行参数模式
        handle_command_args(&args, &mut task_manager);
    } else {
        // 交互式模式
        run_interactive_mode(&mut task_manager, &cli);
    }
}

fn run_interactive_mode(task_manager: &mut TaskManager, cli: &CliInterface) {
    println!("欢迎使用任务管理系统");
    
    loop {
        cli.display_menu();
        let choice = cli.get_user_input("请输入你的选择: ");
        
        match choice.trim() {
            "1" => {
                let title = cli.get_user_input("任务标题: ");
                let description = cli.get_user_input("任务描述: ");
                let task = Task::new(title, description);
                task_manager.add_task(task);
                println!("任务已添加！");
            },
            "2" => {
                task_manager.list_tasks();
            },
            "3" => {
                let id_str = cli.get_user_input("请输入要更新的任务ID: ");
                match id_str.trim().parse::<usize>() {
                    Ok(id) => {
                        cli.display_status_options();
                        let status_input = cli.get_user_input("请选择新的状态 (1-3): ");
                        
                        let new_status = match status_input.trim() {
                            "1" => TaskStatus::Todo,
                            "2" => TaskStatus::InProgress,
                            "3" => TaskStatus::Done,
                            _ => {
                                println!("无效的状态选择");
                                continue;
                            }
                        };
                        
                        if task_manager.update_task_status(id, new_status) {
                            println!("任务状态已更新！");
                        } else {
                            println!("找不到指定ID的任务");
                        }
                    },
                    Err(_) => println!("无效的ID，请输入数字"),
                }
            },
            "4" => {
                let id_str = cli.get_user_input("请输入要删除的任务ID: ");
                match id_str.trim().parse::<usize>() {
                    Ok(id) => {
                        if task_manager.delete_task(id) {
                            println!("任务已删除！");
                        } else {
                            println!("找不到指定ID的任务");
                        }
                    },
                    Err(_) => println!("无效的ID，请输入数字"),
                }
            },
            "5" => {
                let id_str = cli.get_user_input("请输入要查看的任务ID: ");
                match id_str.trim().parse::<usize>() {
                    Ok(id) => {
                        task_manager.view_task(id);
                    },
                    Err(_) => println!("无效的ID，请输入数字"),
                }
            },
            "q" | "Q" => {
                println!("感谢使用，再见！");
                break;
            },
            _ => println!("无效的选择，请重试"),
        }
    }
}

fn handle_command_args(args: &[String], task_manager: &mut TaskManager) {
    match args[1].as_str() {
        "add" => {
            if args.len() < 4 {
                println!("使用方式: {} add <标题> <描述>", args[0]);
                return;
            }
            let task = Task::new(args[2].clone(), args[3].clone());
            task_manager.add_task(task);
            println!("任务已添加！");
        },
        "list" => {
            task_manager.list_tasks();
        },
        "update" => {
            if args.len() < 4 {
                println!("使用方式: {} update <ID> <状态>", args[0]);
                return;
            }
            
            match args[2].parse::<usize>() {
                Ok(id) => {
                    let new_status = match args[3].as_str() {
                        "todo" => TaskStatus::Todo,
                        "progress" => TaskStatus::InProgress,
                        "done" => TaskStatus::Done,
                        _ => {
                            println!("无效的状态，可选值：todo, progress, done");
                            return;
                        }
                    };
                    
                    if task_manager.update_task_status(id, new_status) {
                        println!("任务状态已更新！");
                    } else {
                        println!("找不到指定ID的任务");
                    }
                },
                Err(_) => println!("无效的ID，请输入数字"),
            }
        },
        "delete" => {
            if args.len() < 3 {
                println!("使用方式: {} delete <ID>", args[0]);
                return;
            }
            
            match args[2].parse::<usize>() {
                Ok(id) => {
                    if task_manager.delete_task(id) {
                        println!("任务已删除！");
                    } else {
                        println!("找不到指定ID的任务");
                    }
                },
                Err(_) => println!("无效的ID，请输入数字"),
            }
        },
        "view" => {
            if args.len() < 3 {
                println!("使用方式: {} view <ID>", args[0]);
                return;
            }
            
            match args[2].parse::<usize>() {
                Ok(id) => {
                    task_manager.view_task(id);
                },
                Err(_) => println!("无效的ID，请输入数字"),
            }
        },
        "help" => {
            println!("任务管理器 - 命令列表：");
            println!("  {} add <标题> <描述> - 添加新任务", args[0]);
            println!("  {} list - 列出所有任务", args[0]);
            println!("  {} update <ID> <状态> - 更新任务状态 (状态: todo, progress, done)", args[0]);
            println!("  {} delete <ID> - 删除任务", args[0]);
            println!("  {} view <ID> - 查看任务详情", args[0]);
            println!("  {} help - 显示此帮助", args[0]);
        },
        _ => {
            println!("未知命令。使用 '{} help' 查看可用命令", args[0]);
        }
    }
} 