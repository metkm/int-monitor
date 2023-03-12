// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod network;

use tauri::{Window, WindowEvent};
use windows::Win32::NetworkManagement::IpHelper::MIB_TCPTABLE_OWNER_PID;

#[tauri::command]
fn init_process(_window: Window) {
    std::thread::spawn(move || {
        let socket_info = network::table::get_socket_info::<MIB_TCPTABLE_OWNER_PID>(network::table::Protocol::Tcp);
        
        for row in socket_info.rows {
            println!("pid: {} - {:?}", row.owning_pid, row.owning_module_info);
        }
    });
}

fn main() {
    tauri::Builder::default()
        .on_window_event(|window_event| {
            if let WindowEvent::Resized(_) = window_event.event() {
                std::thread::sleep(std::time::Duration::from_nanos(1));
            }
        })
        .invoke_handler(tauri::generate_handler![init_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
