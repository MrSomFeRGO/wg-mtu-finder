use std::process::Command;
use std::thread;
use std::time::Duration;
use std::net::TcpStream;
use crate::messages::{Message, receive_message};

// Функция для установки MTU на интерфейсе
pub fn set_mtu(interface: &str, mtu: u32) {
    println!("Setting MTU {} on interface {}", mtu, interface);
    let output = Command::new("ip")
        .args(&["link", "set", interface, "mtu", &mtu.to_string()])
        .output()
        .expect("Failed to execute ip command");

    if !output.status.success() {
        eprintln!(
            "Warning: Failed to set MTU: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Небольшая пауза для применения изменений
    thread::sleep(Duration::from_millis(500));
}

// Функция для получения удаленного MTU
pub fn get_remote_mtu(stream: &mut TcpStream, _interface: &str) -> Result<u32, String> {
    match receive_message::<Message>(stream) {
        Ok(Message::MtuValue(mtu)) => Ok(mtu),
        Ok(_) => Err("Unexpected message type from server".to_string()),
        Err(e) => Err(format!("Failed to receive MTU from server: {}", e)),
    }
}
