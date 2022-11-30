use plotters::prelude::*;
use web_sys::SvgElement;
use yew::prelude::*;


#[function_component]
pub fn Home() -> Html {

  let svg_str = use_state_eq(|| draw(1));
  let svg_ref = use_node_ref();
  {
    let svg_str = svg_str.clone();
    let svg_ref = svg_ref.clone();
    use_effect_with_deps(
      |(svg_str, svg_ref)| {
        let svg = svg_ref.cast::<SvgElement>()
          .expect("svg_ref not attached to svg element");
        svg.set_outer_html(svg_str);
      },
      (svg_str, svg_ref),
    );
  }

  html! {
    <>

      <h1>{ "Home" }</h1>

      <h2>{ "Plot" }</h2>

      <svg ref={svg_ref}/>

    </>
  }

}


/// Draw power function f(x) = x^power.
pub fn draw(power: i32) -> String {

  let size = (480, 480);
  let mut svg_str = String::new();
  let backend = SVGBackend::with_string(&mut svg_str, size);

  draw_on_backend(backend, power).expect("draw_on_backend error");

  svg_str

}

fn draw_on_backend<Backend>(backend: Backend, power: i32) -> Result<(), DrawingAreaErrorKind<Backend::ErrorType>>
where Backend: plotters::prelude::DrawingBackend {

  let root = backend.into_drawing_area();
  let font: FontDesc = ("sans-serif", 20.0).into();

  root.fill(&WHITE)?;

  let mut chart = ChartBuilder::on(&root)
    .margin(20)
    .caption(format!("y=x^{}", power), font)
    .x_label_area_size(30)
    .y_label_area_size(30)
    .build_cartesian_2d(-1f32..1f32, -1.2f32..1.2f32)?;

  chart.configure_mesh().x_labels(3).y_labels(3).draw()?;

  chart.draw_series(LineSeries::new(
    (-50..=50)
      .map(|x| x as f32 / 50.0)
      .map(|x| (x, x.powf(power as f32))),
    &RED,
  ))?;

  root.present()?;

  Ok(())

}
