use web_sys::Element;
use yew::prelude::*;

use crate::{button::Button, plot::SvgPlot};


#[function_component]
pub fn Home() -> Html {

  let svg_plot = use_state(|| SvgPlot::power_fn_plot(1));
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

  let svg_plot_clone = svg_plot.clone();
  let onclick_line = move |_| {
    svg_plot_clone.set(SvgPlot::power_fn_plot(1))
  };
  let svg_plot_clone = svg_plot.clone();
  let onclick_para = move |_| {
    svg_plot_clone.set(SvgPlot::power_fn_plot(2))
  };
  let svg_plot_clone = svg_plot.clone();
  let onclick_dir_graph = move |_| {
    svg_plot_clone.set(SvgPlot::directed_graph())
  };
  let svg_plot_clone = svg_plot.clone();
  let onclick_und_graph = move |_| {
    svg_plot_clone.set(SvgPlot::undirected_graph())
  };

  // HTML
  html! {
    <>

      <h1>{ "Home" }</h1>

      <h2>{ "Plot" }</h2>

      <Button onmousedown={ onclick_line }>{ "Click to plot line" }</Button>
      <Button onmousedown={ onclick_para }>{ "Click to plot parabola" }</Button>
      <Button onmousedown={ onclick_dir_graph }>{ "Click to plot directed graph" }</Button>
      <Button onmousedown={ onclick_und_graph }>{ "Click to plot undirected graph" }</Button>

      <div ref={div_svg_ref}/>

    </>
  }

}
