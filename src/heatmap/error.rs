use plotters::drawing::DrawingAreaErrorKind;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HeatmapError {
    #[error("IO error: {0}")]
    IO(#[from] io::Error),

    #[error("CSV parsing error: {0}")]
    CSVParse(String),

    #[error("Drawing error: {0}")]
    Drawing(String),
}

// Add implementation for drawing area errors
impl<E: std::fmt::Debug + std::error::Error + Send + Sync> From<DrawingAreaErrorKind<E>>
    for HeatmapError
{
    fn from(err: DrawingAreaErrorKind<E>) -> Self {
        HeatmapError::Drawing(format!("{:?}", err))
    }
}
