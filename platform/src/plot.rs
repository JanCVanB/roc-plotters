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
    let x_axis = (-3.4f32..3.4).step(0.1);
    let mut cc = ChartBuilder::on(&area)
        .margin(5)
        .set_all_label_area_size(50)
        .caption(config.subtitle.as_str(), ("sans-serif", 40))
        .build_cartesian_2d(-3.4f32..3.4, -1.2f32..1.2f32)?;
    cc.configure_mesh()
        .x_labels(20)
        .y_labels(10)
        .disable_mesh()
        .x_label_formatter(&|v| format!("{:.1}", v))
        .y_label_formatter(&|v| format!("{:.1}", v))
        .draw()?;
    cc.draw_series(LineSeries::new(x_axis.values().map(|x| (x, x.sin())), &RED))?
        .label("Sine")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    cc.draw_series(LineSeries::new(
        x_axis.values().map(|x| (x, x.cos())),
        &BLUE,
    ))?
    .label("Cosine")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    cc.configure_series_labels().border_style(&BLACK).draw()?;
    area.present().unwrap_or_else(|_| panic!(
        "I failed to draw your plot to {} !",
        config.outputFilePath.as_str(),
    ));
    println!("I drew your plot to {}", config.outputFilePath.as_str());
    Ok(())
}
