use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;

pub fn daemon_pid_path() -> PathBuf {
    if let Ok(dir) = env::var("HERDR_PLUGIN_STATE_DIR") {
        let p = PathBuf::from(dir);
        let _ = fs::create_dir_all(&p);
        return p.join("theme-watcher.pid");
    }

    #[cfg(windows)]
    {
        if let Ok(appdata) = env::var("APPDATA") {
            let p = PathBuf::from(appdata).join("herdr");
            let _ = fs::create_dir_all(&p);
            return p.join("theme-watcher.pid");
        }
        PathBuf::from(r"C:\Users\Default\AppData\Roaming\herdr\theme-watcher.pid")
    }

    #[cfg(not(windows))]
    {
        if let Ok(home) = env::var("HOME") {
            let p = PathBuf::from(home).join(".config").join("herdr");
            let _ = fs::create_dir_all(&p);
            return p.join("theme-watcher.pid");
        }
        PathBuf::from("/tmp/herdr-theme-watcher.pid")
    }
}

pub fn is_pid_running(pid: u32) -> bool {
    #[cfg(windows)]
    {
        let output = Command::new("tasklist")
            .args(["/FI", &format!("PID eq {}", pid), "/NH"])
            .output();
        if let Ok(out) = output {
            let text = String::from_utf8_lossy(&out.stdout);
            text.contains(&pid.to_string())
        } else {
            false
        }
    }

    #[cfg(unix)]
    {
        extern "C" {
            fn kill(pid: i32, sig: i32) -> i32;
        }
        let res = unsafe { kill(pid as i32, 0) };
        res == 0
    }
}

pub fn kill_pid(pid: u32) -> bool {
    #[cfg(windows)]
    {
        let status = Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/F"])
            .status();
        match status {
            Ok(s) => s.success(),
            Err(_) => false,
        }
    }

    #[cfg(unix)]
    {
        extern "C" {
            fn kill(pid: i32, sig: i32) -> i32;
        }
        let res = unsafe { kill(pid as i32, 15) };
        res == 0
    }
}

pub fn start_daemon(interval_secs: u64) -> io::Result<()> {
    let pid_file = daemon_pid_path();

    if let Ok(content) = fs::read_to_string(&pid_file) {
        if let Ok(existing_pid) = content.trim().parse::<u32>() {
            if is_pid_running(existing_pid) {
                println!("Theme watcher daemon is already running (PID: {}).", existing_pid);
                return Ok(());
            }
        }
    }

    let exe = env::current_exe()?;
    let mut cmd = Command::new(exe);
    cmd.args(["watch", "--interval", &interval_secs.to_string()]);

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        // CREATE_NO_WINDOW (0x08000000) | DETACHED_PROCESS (0x00000008)
        cmd.creation_flags(0x08000000 | 0x00000008);
    }

    let child = cmd.spawn()?;
    let pid = child.id();
    let _ = fs::write(&pid_file, pid.to_string());

    println!("[+] Theme watcher daemon started in background (PID: {}).", pid);
    println!("    Monitoring solar elevation every {} seconds.", interval_secs);
    Ok(())
}

pub fn stop_daemon() -> io::Result<()> {
    let pid_file = daemon_pid_path();

    if let Ok(content) = fs::read_to_string(&pid_file) {
        if let Ok(pid) = content.trim().parse::<u32>() {
            if is_pid_running(pid) {
                let killed = kill_pid(pid);
                let _ = fs::remove_file(&pid_file);
                if killed {
                    println!("[-] Theme watcher daemon (PID: {}) stopped.", pid);
                } else {
                    println!("[-] Signaled theme watcher daemon (PID: {}) to stop.", pid);
                }
                return Ok(());
            }
        }
    }

    let _ = fs::remove_file(&pid_file);
    println!("Theme watcher daemon is not running.");
    Ok(())
}

pub fn status_daemon() -> io::Result<()> {
    let pid_file = daemon_pid_path();

    if let Ok(content) = fs::read_to_string(&pid_file) {
        if let Ok(pid) = content.trim().parse::<u32>() {
            if is_pid_running(pid) {
                println!("Theme watcher daemon is RUNNING (PID: {}).", pid);
                println!("PID file: {}", pid_file.display());
                return Ok(());
            }
        }
    }

    println!("Theme watcher daemon is STOPPED.");
    println!("PID file: {}", pid_file.display());
    Ok(())
}
