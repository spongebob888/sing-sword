use crate::{
    config::{self, ISingBox},
    notify_err, service,
    utils::{self, dirs, init},
};
use anyhow::{Result, anyhow};
use once_cell::sync::OnceCell;
use std::net::SocketAddr;
use tauri::{
    AppHandle, CustomMenuItem, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
    SystemTraySubmenu, Manager, api
};

#[derive(Debug, Clone)]
pub struct Tray {}

impl Tray {
    pub fn global() -> &'static Tray {
        static SERVICE: OnceCell<Tray> = OnceCell::new();
        SERVICE.get_or_init(|| Tray {})
    }

    pub fn tray_menu() -> SystemTrayMenu {
        let mut service = SystemTrayMenu::new();
        let core_name = config::Sword::global().core_name();

        if let Ok(core_list) = service::Core::list_core() {
            // if core_list.len() > 0 {
            //     service = service
            //         .to_owned()
            //         .add_item(CustomMenuItem::new("core_label", "Core").disabled());
            // }

            core_list.iter().for_each(|core| {
                let core_id = format!("service_core_{core}");
                let selected = Some(core) == core_name.as_ref();
                let title = format!("{core}");
                let item = CustomMenuItem::new(core_id, title);
                let item = if selected { item.selected() } else { item };
                service = service.to_owned().add_item(item);
            });

            if core_list.len() > 0 {
                service = service.add_native_item(SystemTrayMenuItem::Separator);
            }
        }

        let config = SystemTrayMenu::new()
            .add_item(CustomMenuItem::new("open_sing_config", "SingBox Config"))
            .add_item(CustomMenuItem::new("open_core_dir", "Core Dir"))
            .add_item(CustomMenuItem::new("open_logs_dir", "Logs Dir"));

        let about = SystemTrayMenu::new().add_item(
            CustomMenuItem::new("app_version", format!("Version {}", init::app_version()))
                .disabled(),
        );

        SystemTrayMenu::new()
            .add_item(CustomMenuItem::new("sing_sheath", "sing-sheath"))
            .add_item(CustomMenuItem::new("clash_dashboard", "Clash Dashboard"))
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_submenu(SystemTraySubmenu::new(
                "Service",
                service
                    .add_item(CustomMenuItem::new("run_core", "Restart Core"))
            ))
            .add_submenu(SystemTraySubmenu::new("Config", config))
            .add_submenu(SystemTraySubmenu::new("About", about))
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(CustomMenuItem::new("quit", "Quit").accelerator("CmdOrControl+Q"))
    }

    pub fn on_event(&self, app_handle: &AppHandle, id: &str) -> Result<()> {
        Ok(match id {
            "sing_sheath" => {
                let window = match app_handle.get_window("main") {
                    Some(window) => match window.is_visible()? {
                        false => {
                            return window.show().map_err(|err|anyhow::anyhow!(err));
                        }
                        true => window,
                    },
                    None => return Err(anyhow!("Can't find main window")),
                };
                window.set_focus()?;
            },
            "clash_dashboard" => {
                let window = match app_handle.get_window("main") {
                    Some(window) => window,
                    None => return Ok(()),
                };
                #[cfg(not(target_os = "macos"))]
                {
                    window.show().unwrap();
                }
                window.set_focus().unwrap();
                app_handle.emit_all("openBoard",  "").unwrap();
            }
            "run_core" => notify_err!(service::Core::global().run_core())?,
            "run_server" => notify_err!(service::Web::global().run_web(app_handle))?,
            "open_sing_config" => utils::open_by_code(&dirs::sing_box_path())?,
            "open_core_dir" => open::that(dirs::core_dir()?)?,
            "open_logs_dir" => open::that(dirs::log_dir())?,
            "quit" => {
                utils::sysopt::Sysopt::global().reset_sysproxy().unwrap_or_else(|_|
                   { 
                    log::info!(target: "app","reset system proxy failed");
                    ()
                    }
                );
                api::process::kill_children();
                app_handle.exit(0);
            },
            _ => {
                // 更换核心
                if id.starts_with("service_core_") {
                    let core = format!("{}", &id[13..]);

                    service::Core::global().change_core(core)?;
                    app_handle.tray_handle().set_menu(Tray::tray_menu())?;
                }
            }
        })
    }
}

pub fn on_system_tray_event(app_handle: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            crate::log_err!(Tray::global().on_event(app_handle, id.as_str()))
        }
        SystemTrayEvent::LeftClick { .. } => {
            let window = match app_handle.get_window("main") {
                Some(window) => match window.is_visible().expect("winvis") {
                    true => {
                        // hide the window instead of closing due to processes not closing memory leak: https://github.com/tauri-apps/wry/issues/590
                        window.hide().expect("winhide");
                        // window.close().expect("winclose");
                        return;

                    }
                    false => window,
                },
                None => return,
            };
            #[cfg(not(target_os = "macos"))]
            {
                window.show().unwrap();
            }
            window.set_focus().unwrap();
        }
        _ => {}
    }
}
