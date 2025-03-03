use crate::data::models::DataPoint;
use crate::heatmap::error::HeatmapError;
use std::fs::File;
use std::io::{self, BufRead};

pub fn read_csv_data(filepath: &str) -> Result<Vec<DataPoint>, HeatmapError> {
    let file = File::open(filepath)?;
    let reader = io::BufReader::new(file);
    let mut data = Vec::new();
    let mut lines = reader.lines();

    // Skip header
    let _ = lines.next();

    for line_result in lines {
        let line = line_result?;
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() < 6 {
            continue;
        }

        let server_mtu = parts[0]
            .parse::<u16>()
            .map_err(|_| HeatmapError::CSVParse(format!("Invalid server_mtu: {}", parts[0])))?;

        let peer_mtu = parts[1]
            .parse::<u16>()
            .map_err(|_| HeatmapError::CSVParse(format!("Invalid peer_mtu: {}", parts[1])))?;

        let upload_rcv_mbps = parts[2].parse::<f64>().map_err(|_| {
            HeatmapError::CSVParse(format!("Invalid upload_rcv_mbps: {}", parts[2]))
        })?;

        let upload_send_mbps = parts[3].parse::<f64>().map_err(|_| {
            HeatmapError::CSVParse(format!("Invalid upload_send_mbps: {}", parts[3]))
        })?;

        let download_rcv_mbps = parts[4].parse::<f64>().map_err(|_| {
            HeatmapError::CSVParse(format!("Invalid download_rcv_mbps: {}", parts[4]))
        })?;

        let download_send_mbps = parts[5].parse::<f64>().map_err(|_| {
            HeatmapError::CSVParse(format!("Invalid download_send_mbps: {}", parts[5]))
        })?;

        data.push(DataPoint {
            server_mtu,
            peer_mtu,
            upload_rcv_mbps,
            upload_send_mbps,
            download_rcv_mbps,
            download_send_mbps,
        });
    }

    Ok(data)
}
