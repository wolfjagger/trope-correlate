use web_sys::Element;
use yew::prelude::*;

use crate::plot::SvgPlot;


#[function_component]
pub fn Home() -> Html {

  let svg_plot = use_state(|| SvgPlot::undirected_graph());
  // div_svg_ref a NodeRef to the element in html that will contain the svg
  let div_svg_ref = use_node_ref();

  // Fill div_svg_ref content with calculated svg
  use_effect_with_deps(
    |(svg_plot, div_svg_ref)| {

      let div_svg = div_svg_ref.cast::<Element>()
        .expect("svg_ref not attached to svg element");

      // svg_str the calculated svg string to draw
      let svg_str = svg_plot.print_to_string();
      div_svg.set_inner_html(&svg_str);

    },
    (svg_plot.clone(), div_svg_ref.clone())
  );

  // Mouse events

  // HTML
  html! {
    <>

      <h1>{ "Home" }</h1>

      <h2>{ "Woah, incredible splash graph!" }</h2>

      <div ref={div_svg_ref}/>

    </>
  }

}
