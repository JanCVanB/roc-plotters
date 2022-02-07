use crate::options::Options;
use plotters::prelude::*;

pub fn plot(options: Options) -> Result<(), Box<dyn std::error::Error>> {
    let area = BitMapBackend::new(options.outputFilePath.as_str(), (1024, 768)).into_drawing_area();
    area.fill(&WHITE)?;
    let area = area.titled(options.title.as_str(), ("sans-serif", 60))?;
    let x_axis = (-3.4f32..3.4).step(0.1);
    let mut cc = ChartBuilder::on(&area)
        .margin(5)
        .set_all_label_area_size(50)
        .caption(options.subtitle.as_str(), ("sans-serif", 40))
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
        options.outputFilePath.as_str(),
    ));
    println!("I drew your plot to {}", options.outputFilePath.as_str());
    Ok(())
}
