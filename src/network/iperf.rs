use serde_json::Value;
use std::process::{Command, Stdio};

// Функция для проверки установки iperf
pub fn check_iperf_installed() -> bool {
    Command::new("which")
        .arg("iperf3")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

// Функция для запуска iperf сервера
pub fn start_iperf_server(port: u16) -> std::process::Child {
    Command::new("iperf3")
        .args(&["-s", "-p", &port.to_string()])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start iperf3 server")
}

// Функция для запуска iperf теста
pub fn run_iperf_test(server_ip: &str, iperf_port: u16, is_download: bool) -> Option<(f64, f64)> {
    if !check_iperf_installed() {
        eprintln!("Error: iperf3 is not installed or not in PATH");
        return None;
    }

    // Создаем строку один раз, чтобы избежать временных значений
    let port_str = iperf_port.to_string();

    let mut args = vec!["-c", server_ip, "-p", &port_str, "-J", "-t", "5", "-i", "5"];

    if is_download {
        args.push("-R");
    }

    let output = Command::new("iperf3")
        .args(&args)
        .output()
        .expect("Failed to execute iperf3 command");

    if output.status.success() {
        parse_iperf_output(&output.stdout)
    } else {
        eprintln!("iperf3 failed: {}", String::from_utf8_lossy(&output.stderr));
        None
    }
}

// Функция для парсинга вывода iperf
pub fn parse_iperf_output(output: &[u8]) -> Option<(f64, f64)> {
    let json_str = String::from_utf8_lossy(output);
    let json: Value = match serde_json::from_str(&json_str) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to parse iperf JSON output: {}", e);
            return None;
        }
    };

    // Извлечь значения скорости получения и отправки
    let rcv_mbps = json["end"]["sum_received"]["bits_per_second"]
        .as_f64()
        .unwrap_or(0.0)
        / 1_000_000.0;

    let send_mbps = json["end"]["sum_sent"]["bits_per_second"]
        .as_f64()
        .unwrap_or(0.0)
        / 1_000_000.0;

    Some((rcv_mbps, send_mbps))
}
