use plotters::{prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea, LineSeries }, style::{IntoFont, RED, WHITE}};

pub fn graph_data(data: Vec<i32>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("test.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(10, 10, 10, 10);
    // After this point, we should be able to draw construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("Nano Seconds of time", ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(0f32..25000f32, 0f32..25000f32)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    let mut draw_vec = vec![];
    for (i, k) in data.into_iter().enumerate() {
        draw_vec.push((i as f32, k as f32/100.0));
    }

    // And we can draw something in the drawing area
    chart.draw_series(LineSeries::new(
        draw_vec,
        &RED,
    ))?;
    Ok(())
}