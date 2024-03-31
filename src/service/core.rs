use crate::{config::Sword, utils::dirs};
use anyhow::{bail, Result, anyhow};
use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use std::{sync::Arc, env};
use tauri::api::process::{Command, CommandChild, CommandEvent};
use std::{thread, time::Duration};

#[derive(Debug, Clone)]
pub struct Core {
    pub core_handler: Arc<RwLock<Option<CommandChild>>>,
}

impl Core {
    pub fn global() -> &'static Core {
        static SERVICE: OnceCell<Core> = OnceCell::new();

        SERVICE.get_or_init(|| Core {
            core_handler: Arc::new(RwLock::new(None)),
        })
    }

    /// 检查sing box配置
    pub fn check_config(&self) -> Result<()> {
        let config_file_dir = Sword::global().profile_filepath().ok_or(anyhow!("open profile path failed"))?;
        let config_file_dir = dirs::path_to_str(&config_file_dir)?;
        
        let config_dir = env::temp_dir();// use a temp dir to check config file
        let config_dir = dirs::path_to_str(&config_dir)?;

        let core_path = current_core_path()?;
        log::debug!("checking sing-box config");
        let output = Command::new(&core_path)
            .args(["check","-c",config_file_dir, "--disable-color", "-D", config_dir])
            .output()?;

        if !output.status.success() {
            bail!("{}", &output.stderr[21..]); // 过滤掉终端颜色值
        }

        Ok(())
    }

    pub fn stop_core(&self) -> Result<()>{
        let mut core_handler = self.core_handler.write();

        core_handler.take().map(|ch| {
            let _ = ch.kill();
        });
        Ok(())
    }
    /// 启动核心
    pub fn run_core(&self) -> Result<()> {
        self.check_config()?;
        log::info!("sing-box config checked!");
        let mut core_handler = self.core_handler.write();

        core_handler.take().map(|ch| {
            log::info!(target: "app", "stop the core");
            let _ = ch.kill();
        });
        // thread::sleep(Duration::from_millis(1000));
        let config_dir = dirs::sing_box_dir();
        let config_dir = dirs::path_to_str(&config_dir)?;
        let core_path = current_core_path()?;
        let cmd = Command::new(&core_path);
        let profile_path = Sword::global().profile_filepath().ok_or(anyhow!("open profile failed"))?;
        let profile_str = dirs::path_to_str(&profile_path)?;
        #[allow(unused_mut)]
        log::info!(target: "app", "run core {core_path} with profile {profile_str}");
        let (mut rx, cmd_child) = cmd
            .args(["run", "-c", profile_str, "-D", config_dir])
            .spawn()?;

        *core_handler = Some(cmd_child);



        #[cfg(feature = "stdout-log")]
        tauri::async_runtime::spawn(async move {
            while let Some(event) = rx.recv().await {
                match event {
                    CommandEvent::Terminated(_) => break,
                    CommandEvent::Error(err) => log::error!("{err}"),
                    CommandEvent::Stdout(line) => log::info!("{line}"),
                    CommandEvent::Stderr(line) => log::error!(target: "app", "{line}"),
                    _ => {}
                }
            }
        });

        Ok(())
    }

    /// 获取所有可执行的文件
    pub fn list_core() -> Result<Vec<String>> {
        let core_dir = dirs::core_dir()?;

        let list = std::fs::read_dir(core_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().map_or(false, |f| f.is_file()))
            .map(|e| match e.path().file_stem() {
                Some(stem) => stem.to_os_string().into_string().ok(),
                None => None,
            })
            .filter_map(|e| e)
            .collect();

        Ok(list)
    }

    pub fn change_core(&self, name: String) -> Result<()> {
        let core_dir = dirs::core_dir()?;

        #[cfg(windows)]
        let core_path = format!("{name}.exe");
        #[cfg(not(windows))]
        let core_path = name.clone();
        let core_path = core_dir.join(core_path);

        if !core_path.exists() {
            bail!("core executable file not exists");
        }

        let sword = Sword::global();
        let mut config = sword.config.write();
        config.core_name = Some(name);
        drop(config);
        sword.save_config()?;

        self.run_core()?;
        Ok(())
    }
}

/// 获取当前的核心路径
pub fn current_core_path() -> Result<String> {
    let core_name = Sword::global()
        .core_name()
        .ok_or(anyhow!("failed to get core name"))?;

    let core_dir_os_str = dirs::core_dir()?.into_os_string();
    let core_dir = core_dir_os_str.to_str()
    .ok_or(anyhow!("core dir to string failed"))?;
    log::debug!("core dir:{}",core_dir);
    fn use_core_path(core_dir:&str,name: &str) -> String {
        #[cfg(target_os = "windows")]
        return format!("{core_dir}\\{name}");
        #[cfg(not(target_os = "windows"))]
        return format!("{core_dir}/{name}");
    }

    Ok(use_core_path(&core_dir,&core_name))
}
