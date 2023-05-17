#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod config;
mod service;
mod utils;

use anyhow::Result;
use std::fs;
use tauri::{Manager, SystemTray, api};
use utils::tauri_handlers;

fn main() {
    let mut app = tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.app_handle();

            utils::init::init_app(&app_handle);

            let sword = config::Sword::global();

            notify_log_err!(utils::sysopt::Sysopt::global().init_sysproxy());
            notify_log_err!(sword.init_config());
            notify_log_err!(sword.init_box_config());

            notify_log_err!(service::Core::global().run_core());
            notify_log_err!(service::Web::global().run_web(&app_handle));

            let _ = app_handle
                .tray_handle()
                .set_menu(service::Tray::tray_menu());

            let main_window = app.get_window("main").unwrap();

            // listen to the `event-name` (emitted on the `main` window)
            let id = main_window.listen("changeProfile", move |_| {
              log::debug!("change profile");
              notify_log_err!(app_handle.tray_handle().set_menu(service::Tray::tray_menu()));
            });
            Ok(())
        })
        .system_tray(SystemTray::new())
        .on_system_tray_event(service::on_system_tray_event)
        .invoke_handler(tauri::generate_handler![
            tauri_handlers::run_config, 
            tauri_handlers::get_config,
            tauri_handlers::reset_proxy,
            tauri_handlers::get_default_config,
            tauri_handlers::get_profile_list,
            tauri_handlers::get_selected_profile,
            tauri_handlers::change_profile,
            ])
            .build(tauri::generate_context!())
        .expect("failed to launch app");

    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    app.run(|app_handle, event| match event {
        tauri::RunEvent::Exit =>{
            notify_log_err!(utils::sysopt::Sysopt::global().reset_sysproxy());
            api::process::kill_children();
            app_handle.exit(0);
        }
        tauri::RunEvent::WindowEvent { label, event, .. } => {
            if label == "main" {
                match event {
                    tauri::WindowEvent::CloseRequested { api, .. } => {
                        api.prevent_close();
                        app_handle.get_window("main").map(|win| {
                            let _ = win.hide();
                        });
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    });
}
