use crate::config::Config;
use plotters::coord::Shift;
use plotters::prelude::*;

pub fn plot(config: &Config) {
    let is_svg = config.output_file_path.as_str().ends_with(".svg");
    if is_svg {
        let backend = construct_svg_backend(config);
        plot_with(backend, config).expect("failed to plot with svg backend");
    } else {
        let backend = construct_bitmap_backend(config);
        plot_with(backend, config).expect("failed to plot with bitmap backend");
    }
}

fn construct_area<Backend: DrawingBackend>(
    backend: Backend,
    config: &Config,
) -> Result<DrawingArea<Backend, Shift>, DrawingAreaErrorKind<Backend::ErrorType>> {
    let area = backend.into_drawing_area();
    area.fill(&WHITE)?;
    let area = area.titled(config.title.as_str(), ("sans-serif", 60))?;
    Ok(area)
}

fn construct_bitmap_backend(config: &Config) -> BitMapBackend {
    BitMapBackend::new(
        config.output_file_path.as_str(),
        (config.width, config.height),
    )
}

fn construct_svg_backend(config: &Config) -> SVGBackend {
    SVGBackend::new(
        config.output_file_path.as_str(),
        (config.width, config.height),
    )
}

fn plot_with<Backend: DrawingBackend>(
    backend: Backend,
    config: &Config,
) -> Result<(), DrawingAreaErrorKind<Backend::ErrorType>> {
    let area = construct_area(backend, config)?;
    let mut cc = ChartBuilder::on(&area)
        .margin(5)
        .set_all_label_area_size(50)
        .caption(config.subtitle.as_str(), ("sans-serif", 40))
        .build_cartesian_2d(-3.2f64..3.2f64, -2.1f64..2.1f64)?;
    cc.configure_mesh()
        .x_labels(20)
        .y_labels(10)
        .disable_mesh()
        .x_label_formatter(&|v| format!("{:.1}", v))
        .y_label_formatter(&|v| format!("{:.1}", v))
        .draw()?;
    for line in config.lines.iter() {
        let v: Vec<(f64, f64)> = line.points.iter().map(|point| (point.x, point.y)).collect();
        let color = RGBColor(line.color.r, line.color.g, line.color.b);
        cc.draw_series(LineSeries::new(v, color))?
            .label(format!("Line {:?}", line.name))
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
    }
    cc.configure_series_labels().border_style(&BLACK).draw()?;
    area.present().unwrap_or_else(|_| {
        panic!(
            "I failed to draw your plot to {} !",
            config.output_file_path.as_str(),
        )
    });
    println!("I drew your plot to {}", config.output_file_path.as_str());
    Ok(())
}
