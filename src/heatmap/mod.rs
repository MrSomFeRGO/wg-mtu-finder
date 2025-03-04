mod data_reader;
mod error;
mod renderer;

use plotters::prelude::*;
use std::collections::HashMap;
use renderer::{draw_heatmap, max_positive_value};
use crate::data::models::HeatmapParameters;
use crate::heatmap::data_reader::read_csv_data;
use crate::heatmap::error::HeatmapError;

pub fn generate_heatmap(params: HeatmapParameters) -> Result<(), HeatmapError> {
    let log_filepath = &params.log_filepath;
    let heatmap_filepath = &params.heatmap_filepath;

    println!("Generating heatmap from log file: {}", log_filepath);

    let data = read_csv_data(log_filepath)?;

    if data.is_empty() {
        return Err(HeatmapError::CSVParse(
            "No valid data found in CSV file".to_string(),
        ));
    }

    // Остальной код без изменений
    let server_mtus: Vec<u16> = data
        .iter()
        .map(|d| d.server_mtu)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let peer_mtus: Vec<u16> = data
        .iter()
        .map(|d| d.peer_mtu)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let server_mtus_sorted = {
        let mut mtus = server_mtus.clone();
        mtus.sort();
        mtus
    };

    let peer_mtus_sorted = {
        let mut mtus = peer_mtus.clone();
        mtus.sort();
        mtus
    };

    // Create a mapping for the metrics
    let mut upload_rcv_map = HashMap::new();
    let mut upload_send_map = HashMap::new();
    let mut download_rcv_map = HashMap::new();
    let mut download_send_map = HashMap::new();

    for point in &data {
        let key = (point.server_mtu, point.peer_mtu);
        upload_rcv_map.insert(key, point.upload_rcv_mbps);
        upload_send_map.insert(key, point.upload_send_mbps);
        download_rcv_map.insert(key, point.download_rcv_mbps);
        download_send_map.insert(key, point.download_send_mbps);
    }

    // Find max value for color scale
    let max_upload_rcv = max_positive_value(&upload_rcv_map);
    let max_upload_send = max_positive_value(&upload_send_map);
    let max_download_rcv = max_positive_value(&download_rcv_map);
    let max_download_send = max_positive_value(&download_send_map);

    let base_size = 1200;
    let width_multiplier = (peer_mtus_sorted.len() as f32 / 10.0).max(1.0);
    let height_multiplier = (server_mtus_sorted.len() as f32 / 10.0).max(1.0);

    let width = (base_size as f32 * width_multiplier) as u32;
    let height = (base_size as f32 * height_multiplier) as u32;

    let root = BitMapBackend::new(heatmap_filepath, (width, height)).into_drawing_area();

    root.fill(&WHITE)?;

    let areas = root.split_evenly((2, 2));

    // Upper left: Upload Receive
    draw_heatmap(
        &areas[0].margin(10, 20, 10, 20),
        "Upload Rcv Bandwidth (Mbps)",
        &server_mtus_sorted,
        &peer_mtus_sorted,
        &upload_rcv_map,
        max_upload_rcv,
    )?;

    // Upper right: Upload Send
    draw_heatmap(
        &areas[1].margin(10, 20, 20, 10),
        "Upload Send Bandwidth (Mbps)",
        &server_mtus_sorted,
        &peer_mtus_sorted,
        &upload_send_map,
        max_upload_send,
    )?;

    // Lower left: Download Receive
    draw_heatmap(
        &areas[2].margin(20, 10, 10, 20),
        "Download Rcv Bandwidth (Mbps)",
        &server_mtus_sorted,
        &peer_mtus_sorted,
        &download_rcv_map,
        max_download_rcv,
    )?;

    // Lower right: Download Send
    draw_heatmap(
        &areas[3].margin(10, 20, 20, 10),
        "Download Send Bandwidth (Mbps)",
        &server_mtus_sorted,
        &peer_mtus_sorted,
        &download_send_map,
        max_download_send,
    )?;

    root.present()?;

    println!(
        "Done generating heatmap. File saved at: {}",
        heatmap_filepath
    );
    Ok(())
}
