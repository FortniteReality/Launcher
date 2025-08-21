use std::path::{Path, PathBuf};
use std::process::{Command, Child, Stdio};
use std::time::Duration;
use std::thread;

#[cfg(target_os = "windows")]
use winapi::um::processthreadsapi::{OpenProcess, TerminateProcess};
#[cfg(target_os = "windows")]
use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_TERMINATE};
#[cfg(target_os = "windows")]
use winapi::um::handleapi::CloseHandle;
#[cfg(target_os = "windows")]
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS};

use crate::launcher::errors::LaunchError;
use crate::launcher::anticheat::download_anticheat;
use crate::launcher::launcher::download_launcher;

/// Configuration for game launch parameters
#[derive(Debug, Clone)]
pub struct LaunchConfig {
    pub game_path: PathBuf,
    pub launch_args: Vec<String>,
    pub initialization_delay: Duration,
}

impl Default for LaunchConfig {
    fn default() -> Self {
        Self {
            game_path: PathBuf::new(),
            launch_args: Vec::new(),
            initialization_delay: Duration::from_secs(1),
        }
    }
}

/// Process management utilities
#[cfg(target_os = "windows")]
pub struct ProcessUtils;

#[cfg(target_os = "windows")]
impl ProcessUtils {
    /// Check if Fortnite game is currently running
    pub fn is_fortnite_running() -> bool {
        Self::is_process_running("FortniteClient-Win64-Shipping.exe")
    }

    /// Check if a specific process is running by name
    pub fn is_process_running(process_name: &str) -> bool {
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
            if snapshot.is_null() {
                return false;
            }

            let mut process_entry: PROCESSENTRY32 = std::mem::zeroed();
            process_entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

            if Process32First(snapshot, &mut process_entry) == 0 {
                CloseHandle(snapshot);
                return false;
            }

            loop {
                let current_process_name = std::ffi::CStr::from_ptr(process_entry.szExeFile.as_ptr())
                    .to_string_lossy()
                    .to_lowercase();

                if current_process_name == process_name.to_lowercase() {
                    CloseHandle(snapshot);
                    return true;
                }

                if Process32Next(snapshot, &mut process_entry) == 0 {
                    break;
                }
            }

            CloseHandle(snapshot);
            false
        }
    }

    /// Kill specified processes before game launch
    pub fn kill_game_processes() -> Result<(), LaunchError> {
        let processes_to_kill = vec![
            "EpicGamesLauncher.exe",
            "FortniteLauncher.exe",
            "FortniteClient-Win64-Shipping_EAC.exe",
            "FortniteClient-Win64-Shipping.exe",
            "EasyAntiCheat_EOS.exe",
            "EpicWebHelper.exe",
            "RealityLauncher.exe",
        ];

        for process_name in processes_to_kill {
            Self::kill_process_by_name(process_name)?;
        }

        // Wait a moment for processes to fully terminate
        thread::sleep(Duration::from_secs(1));
        Ok(())
    }

    /// Kill a process by name
    pub fn kill_process_by_name(process_name: &str) -> Result<(), LaunchError> {
        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
            if snapshot.is_null() {
                return Ok(()); // If we can't get snapshot, just continue
            }

            let mut process_entry: PROCESSENTRY32 = std::mem::zeroed();
            process_entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

            if Process32First(snapshot, &mut process_entry) == 0 {
                CloseHandle(snapshot);
                return Ok(());
            }

            let mut processes_killed = 0;

            loop {
                let current_process_name = std::ffi::CStr::from_ptr(process_entry.szExeFile.as_ptr())
                    .to_string_lossy()
                    .to_lowercase();

                if current_process_name == process_name.to_lowercase() {
                    let process_handle = OpenProcess(
                        PROCESS_TERMINATE | PROCESS_QUERY_INFORMATION,
                        0,
                        process_entry.th32ProcessID,
                    );

                    if !process_handle.is_null() {
                        TerminateProcess(process_handle, 0);
                        CloseHandle(process_handle);
                        processes_killed += 1;
                    }
                }

                if Process32Next(snapshot, &mut process_entry) == 0 {
                    break;
                }
            }

            CloseHandle(snapshot);

            if processes_killed > 0 {
                println!("Killed {} instances of {}", processes_killed, process_name);
            }

            Ok(())
        }
    }
}

// Non-Windows stub implementations
#[cfg(not(target_os = "windows"))]
pub struct ProcessUtils;

#[cfg(not(target_os = "windows"))]
impl ProcessUtils {
    pub fn is_fortnite_running() -> bool {
        false // Stub for non-Windows
    }

    pub fn is_process_running(_process_name: &str) -> bool {
        false // Stub for non-Windows
    }

    pub fn kill_game_processes() -> Result<(), LaunchError> {
        Ok(()) // Stub for non-Windows
    }

    pub fn kill_process_by_name(_process_name: &str) -> Result<(), LaunchError> {
        Ok(()) // Stub for non-Windows
    }
}

/// Process manager for tracking the launched game
pub struct ProcessManager {
    child_process: Option<Child>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            child_process: None,
        }
    }

    pub fn launch_process(&mut self, executable_path: &Path, args: &[String], working_dir: &Path) -> Result<(), LaunchError> {
        let mut command = Command::new(executable_path);
        command
            .args(args)
            .current_dir(working_dir)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())  // Capture output for debugging
            .stderr(Stdio::piped()); // Capture errors for debugging

        // Inherit environment variables from parent process
        command.env_clear();
        for (key, value) in std::env::vars() {
            command.env(key, value);
        }
        
        // Add specific environment variables that might be needed
        command.env("PATH", std::env::var("PATH").unwrap_or_default());
        command.env("TEMP", std::env::var("TEMP").unwrap_or_else(|_| "C:\\temp".to_string()));
        command.env("TMP", std::env::var("TMP").unwrap_or_else(|_| "C:\\temp".to_string()));

        let child = command
            .spawn()
            .map_err(|e| LaunchError::LaunchFailed(format!("Failed to launch RealityLauncher.exe: {}", e)))?;

        self.child_process = Some(child);
        Ok(())
    }

    pub fn is_running(&mut self) -> bool {
        if let Some(child) = &mut self.child_process {
            match child.try_wait() {
                Ok(Some(_)) => false, // Process has exited
                Ok(None) => true,     // Process is still running
                Err(_) => false,      // Error occurred, assume not running
            }
        } else {
            false
        }
    }

    pub fn wait_for_exit(&mut self) -> Result<(), LaunchError> {
        if let Some(mut child) = self.child_process.take() {
            child.wait()
                .map_err(|e| LaunchError::LaunchFailed(format!("Failed to wait for process: {}", e)))?;
        }
        Ok(())
    }

    pub fn terminate(&mut self) -> Result<(), LaunchError> {
        if let Some(mut child) = self.child_process.take() {
            child.kill()
                .map_err(|e| LaunchError::LaunchFailed(format!("Failed to terminate process: {}", e)))?;
            child.wait()
                .map_err(|e| LaunchError::LaunchFailed(format!("Failed to wait for process termination: {}", e)))?;
        }
        Ok(())
    }

    pub fn get_process_id(&self) -> Option<u32> {
        self.child_process.as_ref().map(|child| child.id())
    }

    /// Launch process through CMD (mimics manual launch)
    pub fn launch_process_via_cmd(&mut self, executable_path: &Path, args: &[String], working_dir: &Path) -> Result<(), LaunchError> {
        // Build the command string as it would appear in CMD
        let exe_name = executable_path.file_name()
            .ok_or_else(|| LaunchError::InvalidPath("Invalid executable path".to_string()))?
            .to_string_lossy();
        
        let args_str = args.join(" ");
        let full_command = format!("{} {}", exe_name, args_str);

        let mut command = Command::new("cmd");
        command
            .args(&["/C", &full_command])
            .current_dir(working_dir)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // Inherit full environment
        for (key, value) in std::env::vars() {
            command.env(key, value);
        }

        let child = command
            .spawn()
            .map_err(|e| LaunchError::LaunchFailed(format!("Failed to launch via CMD: {}", e)))?;

        self.child_process = Some(child);
        Ok(())
    }
}

impl Drop for ProcessManager {
    fn drop(&mut self) {
        let _ = self.terminate();
    }
}

/// Main game launcher that launches RealityLauncher.exe
pub struct GameLauncher {
    config: LaunchConfig,
    process_manager: ProcessManager,
}

impl GameLauncher {
    pub fn new(config: LaunchConfig) -> Result<Self, LaunchError> {
        // Validate game path
        if !config.game_path.exists() {
            return Err(LaunchError::InvalidPath(
                config.game_path.to_string_lossy().to_string(),
            ));
        }

        Ok(Self {
            config,
            process_manager: ProcessManager::new(),
        })
    }

    /// Check if Fortnite is currently running
    pub fn is_fortnite_running() -> bool {
        ProcessUtils::is_fortnite_running()
    }

    /// Launch the game using RealityLauncher.exe
    pub async fn launch(&mut self) -> Result<bool, LaunchError> {
        // Kill existing game processes first
        ProcessUtils::kill_game_processes()?;

        // Download the latest anti-cheat before launching
        download_anticheat(&self.config.game_path.to_string_lossy().to_string()).await?;

        // Download the latest launcher before launching
        download_launcher(&self.config.game_path.to_string_lossy().to_string()).await?;

        self.validate_paths()?;
        self.validate_launch_arguments()?;

        let binaries_path = self.get_binaries_directory();
        let reality_launcher_path = binaries_path.join("RealityLauncher.exe");
        let launch_args = self.build_launch_arguments();

        // Launch RealityLauncher.exe with binaries directory as working directory
        self.process_manager.launch_process_via_cmd(&reality_launcher_path, &launch_args, &binaries_path)?;

        // Wait for initialization
        thread::sleep(self.config.initialization_delay);

        // Monitor the game process
        while self.process_manager.is_running() {
            thread::sleep(Duration::from_secs(1));
        }

        // Process has exited
        Ok(true)
    }

    /// Launch the game and return immediately without monitoring
    pub async fn launch_detached(&mut self) -> Result<bool, LaunchError> {
        // Kill existing game processes first
        ProcessUtils::kill_game_processes()?;

        // Download the latest anti-cheat before launching
        download_anticheat(&self.config.game_path.to_string_lossy().to_string()).await?;

        // Download the latest launcher before launching
        download_launcher(&self.config.game_path.to_string_lossy().to_string()).await?;

        self.validate_paths()?;
        self.validate_launch_arguments()?;

        let binaries_path = self.get_binaries_directory();
        let reality_launcher_path = binaries_path.join("RealityLauncher.exe");
        let launch_args = self.build_launch_arguments();

        // Launch RealityLauncher.exe with binaries directory as working directory
        self.process_manager.launch_process_via_cmd(&reality_launcher_path, &launch_args, &binaries_path)?;

        // Wait for initialization
        thread::sleep(self.config.initialization_delay);

        Ok(true)
    }

    /// Check if the game is currently running (checks for Fortnite process)
    pub fn is_game_running(&mut self) -> bool {
        // Check both our tracked process and the actual Fortnite game process
        self.process_manager.is_running() || ProcessUtils::is_fortnite_running()
    }

    /// Force cleanup/termination of the game process
    pub fn cleanup(&mut self) -> Result<(), LaunchError> {
        // Kill our tracked process
        self.process_manager.terminate()?;
        
        // Also kill all game-related processes
        ProcessUtils::kill_game_processes()
    }

    /// Wait for the game process to exit
    pub fn wait_for_exit(&mut self) -> Result<(), LaunchError> {
        self.process_manager.wait_for_exit()
    }

    /// Get the process ID of the launched game
    pub fn get_process_id(&self) -> Option<u32> {
        self.process_manager.get_process_id()
    }

    fn validate_paths(&self) -> Result<(), LaunchError> {
        let binaries_path = self.get_binaries_directory();
        
        // Check that the binaries directory exists
        if !binaries_path.exists() {
            return Err(LaunchError::InvalidPath(
                format!("Binaries directory not found: {}", binaries_path.display()),
            ));
        }

        // Check for RealityLauncher.exe (this replaces the DLL check)
        let reality_launcher = binaries_path.join("RealityLauncher.exe");
        if !reality_launcher.exists() {
            return Err(LaunchError::MissingLauncher);
        }

        // Check for FortniteClient-Win64-Shipping.exe
        let fortnite_exe = binaries_path.join("FortniteClient-Win64-Shipping.exe");
        if !fortnite_exe.exists() {
            return Err(LaunchError::MissingFortnite);
        }

        // Check for FortniteLauncher.exe
        let launcher_exe = binaries_path.join("FortniteLauncher.exe");
        if !launcher_exe.exists() {
            return Err(LaunchError::MissingLauncher);
        }

        // Check for Reality directory and Equinox.dll
        let reality_dir = binaries_path.join("Reality");
        let equinox_dll = reality_dir.join("Equinox.dll");
        if !equinox_dll.exists() {
            return Err(LaunchError::MissingAntiCheat);
        }

        Ok(())
    }

    fn validate_launch_arguments(&self) -> Result<(), LaunchError> {
        // Add any argument validation logic here
        for arg in &self.config.launch_args {
            if arg.contains('\0') || arg.len() > 1000 {
                return Err(LaunchError::InvalidArguments(arg.clone()));
            }
        }
        Ok(())
    }

    fn build_launch_arguments(&self) -> Vec<String> {
        let mut args = vec![
            "-epicapp=Fortnite".to_string(),
            "-epicenv=Prod".to_string(),
            "-epicportal".to_string(),
            "-epiclocale=en-us".to_string(),
            "-noeac".to_string(),
            "-fromfl=be".to_string(),
            "-fltoken=8c4aa8a9b77acdcbd918874b".to_string(),
            "-skippatchcheck".to_string(),
        ];

        // Add custom launch arguments
        args.extend(self.config.launch_args.clone());
        args
    }

    /// Get the Win64 binaries directory path
    fn get_binaries_directory(&self) -> PathBuf {
        self.config.game_path.join("FortniteGame/Binaries/Win64")
    }

    /// Update the launch configuration
    pub fn update_config(&mut self, config: LaunchConfig) -> Result<(), LaunchError> {
        if !config.game_path.exists() {
            return Err(LaunchError::InvalidPath(
                config.game_path.to_string_lossy().to_string(),
            ));
        }

        self.config = config;
        Ok(())
    }

    /// Get the current configuration
    pub fn get_config(&self) -> &LaunchConfig {
        &self.config
    }

    /// Restart the launcher (terminate current process and launch again)
    pub async fn restart(&mut self) -> Result<bool, LaunchError> {
        self.cleanup()?;
        thread::sleep(Duration::from_secs(1)); // Brief pause between restart
        self.launch_detached().await
    }
}