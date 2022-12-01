use plotters::prelude::*;
use web_sys::Element;
use yew::prelude::*;


#[function_component]
pub fn Home() -> Html {

  // div_svg_ref a NodeRef to the element in html that will contain the svg
  let div_svg_ref = use_node_ref();

  // Fill div_svg_ref content with calculated svg
  use_effect_with_deps(
    |div_svg_ref| {

      let div_svg = div_svg_ref.cast::<Element>()
        .expect("svg_ref not attached to svg element");

      // svg_str the calculated svg string to draw
      let svg_str = create_plot_svg_str(1);
      div_svg.set_inner_html(&svg_str);

    },
    div_svg_ref.clone()
  );

  html! {
    <>

      <h1>{ "Home" }</h1>

      <h2>{ "Plot" }</h2>

      <div ref={div_svg_ref}/>

    </>
  }

}


/// Create a svg string for a plot of the power function f(x) = x^power.
pub fn create_plot_svg_str(power: i32) -> String {

  // TODO: Figure out where to move the size handling
  let size = (480, 480);
  let mut svg_str = String::new();

  // Get svg backend and associate it with svg_str
  let backend = SVGBackend::with_string(&mut svg_str, size);

  draw_on_backend(backend, power).expect("draw_on_backend error");

  svg_str

}

/// Draw power function on the given backend
fn draw_on_backend<Backend>(backend: Backend, power: i32) -> Result<(), DrawingAreaErrorKind<Backend::ErrorType>>
where Backend: plotters::prelude::DrawingBackend {

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
      .map(|x| (x, x.powf(power as f32))),
    &RED,
  ))?;

  // Present changes to the backend
  root.present()?;

  Ok(())

}
