use plotters::prelude::*;

use crate::graph::{DirectedTropeGraph, UndirectedTropeGraph};


#[derive(PartialEq)]
pub enum PlotType {
  PowerFn(i32),
  DirectedPetGraph(DirectedTropeGraph),
  UndirectedPetGraph(UndirectedTropeGraph),
}

#[derive(PartialEq)]
pub struct SvgPlot {
  pub size: (u32, u32),
  pub plot_type: PlotType,
}

impl SvgPlot {

  /// Create a svg string for a plot of the power function f(x) = x^power.
  pub fn power_fn_plot(power: i32) -> Self {

    // TODO: Figure out where to move the size handling
    let size = (480, 480);
    let plot_type = PlotType::PowerFn(power);

    Self {
      size,
      plot_type
    }

  }

  /// Create a svg string for a plot of a directed graph
  pub fn directed_graph() -> Self {

    // TODO: Figure out where to move the size handling
    let size = (480, 480);
    let plot_type = PlotType::DirectedPetGraph(DirectedTropeGraph::new());

    Self {
      size,
      plot_type
    }

  }

  /// Create a svg string for a plot of an undirected graph
  pub fn undirected_graph() -> Self {

    // TODO: Figure out where to move the size handling
    let size = (480, 480);
    let plot_type = PlotType::UndirectedPetGraph(UndirectedTropeGraph::new());

    Self {
      size,
      plot_type
    }

  }

  pub fn print_to_string(&self) -> String {

    let mut svg_str = String::new();
    // Get svg backend and associate it with svg_str
    let backend = SVGBackend::with_string(&mut svg_str, self.size);

    self.draw_on_backend(backend).expect("draw_on_backend error");
    svg_str

  }

  /// Draw power function on the given backend
  fn draw_on_backend<Backend>(&self, backend: Backend) -> Result<(), DrawingAreaErrorKind<Backend::ErrorType>>
  where Backend: plotters::prelude::DrawingBackend {

    match &self.plot_type {
      PlotType::PowerFn(power) => {

        // Get drawing area, set font and background
        let root = backend.into_drawing_area();
        let font: FontDesc = ("sans-serif", 20.0).into();
        root.fill(&WHITE)?;

        // Build chart components without data
        let mut chart = ChartBuilder::on(&root)
          .margin(20)
          .caption(format!("y=x^{}", power), font)
          .x_label_area_size(30)
          .y_label_area_size(30)
          .build_cartesian_2d(-1f32..1f32, -1.2f32..1.2f32)?;

        // Configure the mesh
        chart.configure_mesh().x_labels(3).y_labels(3).draw()?;

        // Plot the power function data
        chart.draw_series(LineSeries::new(
          (-50..=50)
            .map(|x| x as f32 / 50.0)
            .map(|x| (x, x.powf(*power as f32))),
          &RED,
        ))?;

        // Present changes to the backend
        root.present()?;

        Ok(())

      },
      PlotType::DirectedPetGraph(_g) => {
        todo!("Implement DirectedPetGraph plotting")
      },
      PlotType::UndirectedPetGraph(_g) => {
        todo!("Implement UndirectedPetGraph plotting")
      }
    }


  }

}