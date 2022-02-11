use crate::config::Config;
use plotters::coord::{Shift};
use plotters::prelude::*;

pub fn plot(config: &Config) {
    let isSvg = config.outputFilePath.as_str().ends_with(".svg");
    if isSvg {
        let backend = construct_svg_backend(config);
        plot_with(backend, config);
    } else {
        let backend = construct_bitmap_backend(config);
        plot_with(backend, config);
    }
}

fn construct_area<Backend: DrawingBackend>(
    backend: Backend,
    config: &Config,
) -> Result<
    DrawingArea<Backend, Shift>,
    DrawingAreaErrorKind<Backend::ErrorType>,
> {
    let area = backend.into_drawing_area();
    area.fill(&WHITE)?;
    let area = area.titled(config.title.as_str(), ("sans-serif", 60))?;
    Ok(area)
}

fn construct_bitmap_backend(config: &Config) -> BitMapBackend {
    BitMapBackend::new(
        config.outputFilePath.as_str(),
        (config.width, config.height),
    )
}

fn construct_svg_backend(config: &Config) -> SVGBackend {
    SVGBackend::new(
        config.outputFilePath.as_str(),
        (config.width, config.height),
    )
}

fn plot_with<Backend: DrawingBackend>(
    backend: Backend,
    config: &Config,
) -> Result<(), DrawingAreaErrorKind<Backend::ErrorType>> {
    let area = construct_area(backend, config)?;
    dbg!(config);
    let xy1: Vec<(i32, i32)> = dbg!(config.x.into_iter().zip(config.y1.into_iter()).collect());
    let xy2: Vec<(i32, i32)> = dbg!(config.x.into_iter().zip(config.y2.into_iter()).collect());
    let mut cc = ChartBuilder::on(&area)
        .margin(5)
        .set_all_label_area_size(50)
        .caption(config.subtitle.as_str(), ("sans-serif", 40))
        .build_cartesian_2d(-3i32..3i32, -3i32..3i32)?;
    cc.configure_mesh()
        .x_labels(20)
        .y_labels(10)
        .disable_mesh()
        .x_label_formatter(&|v| format!("{:.1}", v))
        .y_label_formatter(&|v| format!("{:.1}", v))
        .draw()?;
    cc.draw_series(LineSeries::new(xy1, &RED))?
        .label("Line 1")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    cc.draw_series(LineSeries::new(xy2, &BLUE))?
        .label("Line 2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    cc.configure_series_labels().border_style(&BLACK).draw()?;
    area.present().unwrap_or_else(|_| panic!(
        "I failed to draw your plot to {} !",
        config.outputFilePath.as_str(),
    ));
    println!("I drew your plot to {}", config.outputFilePath.as_str());
    Ok(())
}
