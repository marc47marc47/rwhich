use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn is_executable(path: &Path) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        if metadata.is_file() {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let permissions = metadata.permissions();
                // 检查文件是否具有执行权限
                return permissions.mode() & 0o111 != 0;
            }
            #[cfg(not(unix))]
            {
                // 在非 Unix 系统上，假设所有文件都可执行
                return true;
            }
        }
    }
    false
}

fn main() {
    // 获取命令行参数，跳过程序自身的名称
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("用法: which-rs 命令...");
        std::process::exit(1);
    }

    // 获取环境变量 PATH
    let path_var = env::var("PATH").unwrap_or_default();
    // 使用 env::split_paths 处理路径分隔符，跨平台兼容
    let paths: Vec<PathBuf> = env::split_paths(&path_var).collect();

    for command in args {
        let mut found = false;
        for dir in &paths {
            let cmd_path = dir.join(&command);
            if is_executable(&cmd_path) {
                println!("{}", cmd_path.display());
                found = true;
                // 如果想查找所有匹配项，可以注释掉以下这行
                break;
            }
        }
        if !found {
            eprintln!("未找到命令: {}", command);
        }
    }
}

