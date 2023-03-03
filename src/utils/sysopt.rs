use anyhow::{Result};
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use std::sync::Arc;
use sysproxy::Sysproxy;
pub struct Sysopt {
    /// recover it when exit
    old_sysproxy: Arc<Mutex<Option<Sysproxy>>>,

}

impl Sysopt {
    pub fn global() -> &'static Sysopt {
        static SYSOPT: OnceCell<Sysopt> = OnceCell::new();

        SYSOPT.get_or_init(|| Sysopt {
            old_sysproxy: Arc::new(Mutex::new(None)),
        })
    }

    /// init the sysproxy
    pub fn init_sysproxy(&self) -> Result<()> {
        let old = Sysproxy::get_system_proxy().map_or(None, |p| Some(p));
        if !old.is_some() {
            log::info!("No system proxy got");
        }
        *self.old_sysproxy.lock() = old;
        Ok(())
    }


    /// reset the sysproxy
    pub fn reset_sysproxy(&self) -> Result<()> {
        let mut cur_sysproxy = Sysproxy::get_system_proxy().map_or(None, |p| Some(p));
        let mut old_sysproxy = self.old_sysproxy.lock();

        let cur_sysproxy = cur_sysproxy.take();

        if let Some(mut old) = old_sysproxy.take() {
            // 如果原代理和当前代理 端口一致，就disable关闭，否则就恢复原代理设置
            // 当前没有设置代理的时候，不确定旧设置是否和当前一致，全关了
            let port_same = cur_sysproxy.map_or(true, |cur| old.port == cur.port);

            if old.enable && port_same {
                old.enable = false;
                log::info!(target: "app", "reset proxy by disabling the original proxy");
            } else {
                log::info!(target: "app", "reset proxy to the original proxy");
            }

            old.set_system_proxy()?;
        } 
        else {
            let clear_proxy = Sysproxy {
                enable: false,
                host: "".into(),
                port: 9090,
                bypass: "".into(),
            };
            self.clear_proxy()?;
        }
        Ok(())
    }
    pub fn clear_proxy(&self) -> Result<()> {
        let sys_proxy = Sysproxy {
            enable: false,
            host: "".into(),
            port: 9090,
            bypass: "".into(),
        };
        sys_proxy.set_system_proxy()?;
        log::info!(target: "app", "clear system proxy setting");
        Ok(())
    }
}
