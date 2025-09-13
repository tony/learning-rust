use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Output};

#[must_use]
fn find_tmux_path(custom_path: Option<&str>) -> Option<String> {
    if let Some(path) = custom_path
        && Path::new(path).exists()
    {
        return Some(path.to_string());
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
    let output = Command::new(tmux_path).args(args).output()?;
    Ok(output)
}

fn main() {
    let tmux_path = find_tmux_path(None).expect("tmux not found in PATH");
    let output =
        run_tmux_command(&tmux_path, &["list-sessions"]).expect("Failed to run tmux command");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::{self, File};
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;

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

        let original_path = env::var("PATH").unwrap();
        let new_path = format!("{}:{}", temp_path.display(), original_path);
        // TODO: Audit that the environment access only happens in single-threaded code.
        unsafe { env::set_var("PATH", new_path) };

        let result = find_tmux_path(None);

        // Restore original PATH
        unsafe { env::set_var("PATH", &original_path) };

        assert!(result.is_some());
    }

    #[test]
    #[ignore = "This test modifies global PATH and may fail in CI due to parallel test execution"]
    fn test_find_tmux_not_found() {
        // Test with empty PATH - this ensures tmux won't be found
        let temp_dir = tempfile::tempdir().unwrap();
        let original_path = env::var("PATH").ok();

        // TODO: Audit that the environment access only happens in single-threaded code.
        unsafe { env::set_var("PATH", temp_dir.path()) };

        // Test with a non-existent custom path and empty PATH
        let non_existent = "/definitely/not/a/real/path/to/tmux";
        let result = find_tmux_path(Some(non_existent));
        assert!(result.is_none());

        // Test with None and empty PATH
        let result = find_tmux_path(None);
        assert!(result.is_none());

        // Restore original PATH
        if let Some(path) = original_path {
            unsafe { env::set_var("PATH", path) };
        }
    }

    #[test]
    fn test_find_tmux_with_invalid_custom_path() {
        // This test doesn't modify PATH, making it more reliable in CI
        // Test that a non-existent custom path doesn't get returned as-is
        let non_existent = "/definitely/not/a/real/path/to/tmux";

        // We can't guarantee PATH is empty in CI, but we can test
        // that the custom path check works correctly
        let result_from_custom = find_tmux_path(Some(non_existent));

        // The result might be Some if tmux is in PATH, or None if not
        // But we can verify the custom path itself wasn't used
        if let Some(path) = result_from_custom {
            assert_ne!(
                path, non_existent,
                "Should not return the non-existent custom path"
            );
        }
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
