use crate::PLane2D;
use plotters::prelude::*;
pub fn plotter(plane: &PLane2D, output_path: &str) -> anyhow::Result<()> {
    let root_area = BitMapBackend::new(output_path, (800, 800)).into_drawing_area();
    root_area.fill(&BLACK)?;

    let mut chart = ChartBuilder::on(&root_area)
        .margin(10)
        .caption("Poisson Sampling", ("0xProto Nerd Font", 40))
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..plane.width, 0.0..plane.height)?;

    chart.configure_mesh().draw()?;

    for point in &plane.points {
        chart.draw_series(PointSeries::of_element(
            vec![(point.x, point.y)],
            2.5,
            &RED,
            &|c, s, st| {
                return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled());
            },
        ))?;
    }

    root_area.present()?;
    Ok(())
}
