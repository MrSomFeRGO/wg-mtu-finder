use serde::{Deserialize, Serialize};

// Константы
pub const DEFAULT_MIN_MTU: u32 = 1280;
pub const DEFAULT_MAX_MTU: u32 = 1500;
pub const DEFAULT_STEP: u32 = 20;
pub const DEFAULT_CONTROL_PORT: u16 = 9876;
pub const DEFAULT_IPERF_PORT: u16 = 5201;

// Структуры для тестирования
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MtuTestResult {
    pub server_mtu: u32,
    pub client_mtu: u32,
    pub upload_rcv_mbps: f64,
    pub upload_send_mbps: f64,
    pub download_rcv_mbps: f64,
    pub download_send_mbps: f64,
}

// Структура параметров тестирования
pub struct TestParameters {
    pub interface: String,
    pub min_mtu: u32,
    pub max_mtu: u32,
    pub step: u32,
    pub control_port: u16,
    pub iperf_port: u16,
}

// Структура параметров клиента
pub struct PeerParameters {
    pub interface: String,
    pub server_ip: String,
    pub control_port: u16,
    pub iperf_port: u16,
    pub min_mtu: u32,
    pub max_mtu: u32,
    pub step: u32,
    pub csv_file: String,
}

// Структура параметров хитмапы
pub struct HeatmapParameters {
    pub log_filepath: String,
    pub heatmap_filepath: String,
}

pub struct DataPoint {
    pub server_mtu: u16,
    pub peer_mtu: u16,
    pub upload_rcv_mbps: f64,
    pub upload_send_mbps: f64,
    pub download_rcv_mbps: f64,
    pub download_send_mbps: f64,
}