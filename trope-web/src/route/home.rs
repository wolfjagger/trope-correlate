use web_sys::Element;
use yew::prelude::*;

use crate::plot::SvgPlot;


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
      let svg_plot = SvgPlot::power_fn_plot(1);
      let svg_str = svg_plot.print_to_string();
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
