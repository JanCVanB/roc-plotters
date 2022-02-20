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
    let area = area.titled(
        config.title.as_str(),
        (config.fonts.titleFamily.as_str(), config.fonts.titleSize),
    )?;
    Ok(area)
}

fn construct_builder<'a, Backend: DrawingBackend>(
    area: &'a DrawingArea<Backend, Shift>,
    config: &'a Config,
) -> ChartBuilder<'a, 'a, Backend> {
    let subtitle = config.subtitle.as_str();
    let mut builder = ChartBuilder::on(area);
    builder.margin(config.layout.chartMargin)
        .set_all_label_area_size(config.layout.labelArea);
    if subtitle.len() > 0 {
        builder.caption(
            config.subtitle.as_str(),
            (config.fonts.subtitleFamily.as_str(), config.fonts.subtitleSize),
        );
    }
    return builder;
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
    let mut builder = construct_builder(&area, config);
    let mut context = builder.build_cartesian_2d(
        config.bounds.xMin..config.bounds.xMax,
        config.bounds.yMin..config.bounds.yMax,
    )?;
    context.configure_mesh()
        .x_labels(config.labels.xCount)
        .y_labels(config.labels.yCount)
        .disable_mesh()
        .x_label_formatter(&|v| format!("{:?}", v)) // TODO: Format labels in Roc.
        .y_label_formatter(&|v| format!("{:?}", v)) // TODO: Format labels in Roc.
        .draw()?;
    for line in config.lines.iter() {
        let v1: Vec<(f64, f64)> = line.points.iter().map(|point| (point.x, point.y)).collect();
        let v2: Vec<(f64, f64)> = v1.clone(); // TODO: Remove this with proper ownership.
        let color = RGBColor(line.color.r, line.color.g, line.color.b);
        let lineSeries = 
            if line.isLineVisible {LineSeries::new(v1, color)}
            else {LineSeries::new(v1, TRANSPARENT)};
        context.draw_series(lineSeries)?
            .label(line.name.as_str())
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
        if line.isPointVisible {
            context.draw_series(v2.iter().map(
                |(x, y)| Circle::new((*x, *y), line.pointRadius, color.filled())
            ))?;
        }
    }
    context.configure_series_labels().border_style(&BLACK).draw()?;
    area.present().unwrap_or_else(|_| {
        panic!(
            "I failed to draw your plot to {} !",
            config.output_file_path.as_str(),
        )
    });
    println!("I drew your plot to {}", config.output_file_path.as_str());
    Ok(())
}
