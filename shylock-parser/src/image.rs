use chrono::NaiveDate;
use plotters::prelude::*;

fn parse_date(date_str: &str) -> NaiveDate {
    let date_split = date_str.split('-');
    let fields = date_split.collect::<Vec<&str>>();
    NaiveDate::from_ymd_opt(
        fields[0].parse::<i32>().unwrap(),
        fields[1].parse::<u32>().unwrap(),
        1,
    )
    .unwrap()
}

/// Create a svg histogram with given `data` to a `file_path`.
pub fn create_svg_histogram(
    data: &[(String, u32)],
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = SVGBackend::new(file_path, (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let max_value = data.iter().map(|(_, x)| x).max();

    let (from_date, to_date) = (parse_date(&data[0].0), parse_date(&data[data.len() - 1].0));

    log::info!("parsed dates");
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(45)
        .margin(5)
        .caption("Subastas abiertas por mes", ("arial", 42.0))
        .build_cartesian_2d((from_date..to_date).monthly(), 0u32..*max_value.unwrap())?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc("Número de subastas")
        .x_desc("Fecha")
        .axis_desc_style(("arial", 15))
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .margin(0)
            .data(
                data.iter()
                    .rev()
                    .map(|(month, n): &(String, u32)| (parse_date(month), *n)),
            ),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file");
    log::info!("Result has been saved to {}", file_path);

    Ok(())
}
