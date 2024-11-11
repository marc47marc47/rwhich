use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn is_executable(path: &Path) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        if metadata.is_file() {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let permissions = metadata.permissions();
                // Check if the file has execute permissions
                return permissions.mode() & 0o111 != 0;
            }
            #[cfg(windows)]
            {
                // Check if the file extension is in PATHEXT
                if let Some(ext) = path.extension() {
                    let ext = ext.to_string_lossy().to_lowercase();
                    let pathext = get_windows_extensions();
                    return pathext.contains(&format!(".{}", ext));
                }
                return false;
            }
            #[cfg(not(any(unix, windows)))]
            {
                // For other systems, assume all files are executable
                return true;
            }
        }
    }
    false
}

#[cfg(windows)]
pub fn get_windows_extensions() -> Vec<String> {
    // Get the PATHEXT environment variable
    let pathext_var = env::var("PATHEXT").unwrap_or_default();
    // Split the extensions and convert to lowercase
    pathext_var
        .split(';')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_lowercase())
        .collect()
}

pub fn find_command_in_paths(command: &str, paths: &[PathBuf]) -> Option<PathBuf> {
    for dir in paths {
        let cmd_path = dir.join(command);
        if is_executable(&cmd_path) {
            return Some(cmd_path);
        }
    }
    None
}

pub fn which(commands: &[String]) {
    // Get the PATH environment variable
    let path_var = env::var("PATH").unwrap_or_default();
    let paths: Vec<PathBuf> = env::split_paths(&path_var).collect();

    #[cfg(windows)]
    let extensions = get_windows_extensions();

    for command in commands {
        let mut found = false;

        // First, try to find the command directly
        if let Some(cmd_path) = find_command_in_paths(&command, &paths) {
            println!("{}", cmd_path.display());
            found = true;
        } else {
            // If not found, and on Windows, try adding extensions
            #[cfg(windows)]
            {
                if Path::new(&command).extension().is_none() {
                    for ext in &extensions {
                        let command_with_ext = format!("{}{}", command, ext);
                        if let Some(cmd_path) = find_command_in_paths(&command_with_ext, &paths) {
                            println!("{}", cmd_path.display());
                            found = true;
                            break;
                        }
                    }
                }
            }
        }

        if !found {
            eprintln!("Command not found: {}", command);
        }
    }
}

