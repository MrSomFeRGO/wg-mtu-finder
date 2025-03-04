use plotters::coord::Shift;
use plotters::prelude::*;
use std::collections::HashMap;

use crate::heatmap::error::HeatmapError;

pub fn draw_heatmap<DB: DrawingBackend>(
    area: &DrawingArea<DB, Shift>,
    title: &str,
    server_mtus: &[u16],
    peer_mtus: &[u16],
    data_map: &HashMap<(u16, u16), f64>,
    max_value: f64,
) -> Result<(), HeatmapError> {
    // Create a chart with integer coordinates instead of segmented
    let mut chart_builder = ChartBuilder::on(area)
        .caption(title, ("sans-serif", 48))
        .margin(5)
        .x_label_area_size(60)
        .y_label_area_size(90)
        .build_cartesian_2d(0..peer_mtus.len(), 0..server_mtus.len())?;

    chart_builder
        .configure_mesh()
        .disable_mesh()
        .x_labels(peer_mtus.len())
        .y_labels(server_mtus.len())
        .x_label_formatter(&|idx| {
            peer_mtus
                .get(*idx)
                .map_or_else(|| "".to_string(), |v| v.to_string())
        })
        .y_label_formatter(&|idx| {
            server_mtus
                .get(*idx)
                .map_or_else(|| "".to_string(), |v| v.to_string())
        })
        .x_desc("Peer MTU")
        .y_desc("Server MTU")
        .axis_desc_style(("sans-serif", 32))
        .label_style(("sans-serif", 21))
        .draw()?;

    // Define a color map from white to green
    let color_mapping = |v: f64| -> RGBColor {
        if v <= 0.0 {
            // Light gray color for errors (-1.0)
            return RGBColor(240, 240, 240);
        }

        let intensity = (v / max_value).min(1.0); // Normalize from 0 to 1

        // Interpolate from white (255, 255, 255) to green (0, 155, 119)
        let r = (255.0 * (1.0 - intensity)) as u8; // From 255 to 0
        let g = (255.0 - (255.0 - 155.0) * intensity) as u8; // From 255 to 155
        let b = (255.0 - (255.0 - 119.0) * intensity) as u8; // From 255 to 119

        RGBColor(r, g, b)
    };

    // Draw the heatmap cells
    chart_builder.draw_series(server_mtus.iter().enumerate().flat_map(
        |(y_idx, &server_mtu)| {
            peer_mtus.iter().enumerate().map(move |(x_idx, &peer_mtu)| {
                let value = data_map
                    .get(&(server_mtu, peer_mtu))
                    .copied()
                    .unwrap_or(0.0);
                let color = color_mapping(value);

                Rectangle::new([(x_idx, y_idx), (x_idx + 1, y_idx + 1)], color.filled())
            })
        },
    ))?;

    // Add value labels
    chart_builder.draw_series(server_mtus.iter().enumerate().flat_map(
        |(y_idx, &server_mtu)| {
            peer_mtus.iter().enumerate().map(move |(x_idx, &peer_mtu)| {
                let value = data_map
                    .get(&(server_mtu, peer_mtu))
                    .copied()
                    .unwrap_or(0.0);
                let text_color = if value > max_value * 0.7 {
                    WHITE
                } else {
                    BLACK
                };

                let cell_count = peer_mtus.len().max(1) * server_mtus.len().max(1);
                let font_size = match cell_count {
                    0..=900 => 24,
                    901..=1500 => 18,
                    _ => 14,
                };

                Text::new(
                    format!("{:.1}", value),
                    (x_idx, y_idx + 1),
                    ("sans-serif", font_size).into_font().color(&text_color),
                )
            })
        },
    ))?;

    Ok(())
}

pub fn max_positive_value(map: &HashMap<(u16, u16), f64>) -> f64 {
    map.values()
        .filter(|&&v| v > 0.0)
        .fold(0.0, |max, &v| if v > max { v } else { max })
}
