use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Output};
use std::os::unix::fs::PermissionsExt;

fn find_tmux_path(custom_path: Option<&str>) -> Option<String> {
    if let Some(path) = custom_path {
        if Path::new(path).exists() {
            return Some(path.to_string());
        }
    }

    if let Some(paths) = env::var_os("PATH") {
        for path in env::split_paths(&paths) {
            let tmux_path = path.join("tmux");
            if tmux_path.exists() {
                return Some(tmux_path.to_string_lossy().to_string());
            }
        }
    }
    None
}

fn run_tmux_command(tmux_path: &str, args: &[&str]) -> io::Result<Output> {
    let output = Command::new(tmux_path)
        .args(args)
        .output()?;
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::File;
    use std::io::Write;

    fn setup_mock_path(mock_dir: &Path) -> io::Result<()> {
        let mock_tmux = mock_dir.join("tmux");
        let mut file = File::create(&mock_tmux)?;
        writeln!(file, "#!/bin/sh")?;
        writeln!(file, "echo mock tmux")?;
        fs::set_permissions(&mock_tmux, fs::Permissions::from_mode(0o755))?;
        Ok(())
    }

    #[test]
    fn test_find_tmux_custom_path() {
        let mock_tmux = "./mock_tmux";
        File::create(mock_tmux).unwrap();
        let result = find_tmux_path(Some(mock_tmux));
        assert_eq!(result, Some(mock_tmux.to_string()));
        fs::remove_file(mock_tmux).unwrap();
    }

    #[test]
    fn test_find_tmux_in_path() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_path = temp_dir.path().to_path_buf();

        setup_mock_path(&temp_path).unwrap();

        let current_path = env::var("PATH").unwrap();
        let new_path = format!("{}:{}", temp_path.display(), current_path);
        env::set_var("PATH", new_path);

        let result = find_tmux_path(None);
        assert!(result.is_some());
    }

    #[test]
    fn test_find_tmux_not_found() {
        let temp_dir = tempfile::tempdir().unwrap();
        env::set_var("PATH", temp_dir.path());

        let result = find_tmux_path(None);
        assert!(result.is_none());
    }

    #[test]
    fn test_run_tmux_command() {
        let temp_dir = tempfile::tempdir().unwrap();
        setup_mock_path(temp_dir.path()).unwrap();
        let mock_tmux = temp_dir.path().join("tmux");

        let result = run_tmux_command(mock_tmux.to_str().unwrap(), &[]);
        assert!(result.is_ok());
        let output = result.unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert_eq!(stdout.trim(), "mock tmux");
    }
}

fn main() {
    let tmux_path = find_tmux_path(None).expect("tmux not found in PATH");
    let output = run_tmux_command(&tmux_path, &["list-sessions"]).expect("Failed to run tmux command");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
