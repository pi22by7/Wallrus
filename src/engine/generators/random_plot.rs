use super::super::set_wallpaper;
use crate::engine::common::hsv_to_rgb;
use crate::errors::{Result, WallrusError};
use plotters::prelude::*;
use rand::Rng;
use std::path::Path;

pub fn generate_random_plot_wallpaper(width: u32, height: u32, file_path: &Path) -> Result<()> {
    let root = BitMapBackend::new(file_path, (width, height)).into_drawing_area();
    let mut rng = rand::thread_rng();

    let base_hue = rng.gen_range(0..360);
    let plot_color = hsv_to_rgb(base_hue, 0.7, 0.8);
    let background_color = hsv_to_rgb((base_hue + 30) % 360, 0.5, 1.0);

    root.fill(&RGBColor(
        background_color.0,
        background_color.1,
        background_color.2,
    ))
    .map_err(|e| WallrusError::ImageProcessing(format!("Failed to fill background: {}", e)))?;

    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(0..width as i32, 0..height as i32)
        .map_err(|e| WallrusError::ImageProcessing(format!("Failed to build chart: {}", e)))?;

    chart
        .configure_mesh()
        .disable_mesh()
        .disable_axes()
        .draw()
        .map_err(|e| WallrusError::ImageProcessing(format!("Failed to configure chart: {}", e)))?;

    for _ in 0..1000 {
        let data: Vec<(i32, i32)> = (0..100)
            .map(|_| {
                (
                    rng.gen_range(0..width as i32),
                    rng.gen_range(0..height as i32),
                )
            })
            .collect();

        chart
            .draw_series(PointSeries::of_element(
                data,
                1,
                &RGBColor(plot_color.0, plot_color.1, plot_color.2),
                &|coord, size, style| EmptyElement::at(coord) + Circle::new((0, 0), size, style),
            ))
            .map_err(|e| WallrusError::ImageProcessing(format!("Failed to draw series: {}", e)))?;
    }

    root.present()
        .map_err(|e| WallrusError::ImageProcessing(format!("Failed to save plot: {}", e)))?;

    println!("Random plot wallpaper generated at {:?}", file_path);
    set_wallpaper(file_path)?;
    Ok(())
}
