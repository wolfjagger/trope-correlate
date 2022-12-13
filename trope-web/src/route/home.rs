use gloo_timers::callback::Interval;
use web_sys::Element;
use yew::prelude::*;

use crate::{message::Message, plot::{SvgPlot, PlotType}};


const SIM_TIME_MILLI: u32 = 500;
const SIM_SPEED: f64 = 0.035;


pub struct Home {
  svg_anim: SvgPlot,
  div_svg_ref: NodeRef,
  _interval: Interval,
}

impl Component for Home {

  type Message = Message;
  type Properties = ();

  fn create(ctx: &Context<Self>) -> Self {

    let svg_anim = SvgPlot::undirected_graph();
    // div_svg_ref a NodeRef to the element in html that will contain the svg
    let div_svg_ref = NodeRef::default();

    let callback = ctx.link().callback(|_| {
      Message::Tick
    });
    let interval = Interval::new(SIM_TIME_MILLI, move || callback.emit(()));

    Self {
      svg_anim,
      div_svg_ref,
      _interval: interval,
    }

  }

  fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
    // Fill div_svg_ref content with calculated svg
    if let Some(div_svg) = self.div_svg_ref.cast::<Element>() {
      let svg_str = self.svg_anim.print_to_string();
      div_svg.set_inner_html(&svg_str);
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Message::Tick => {
        match &mut self.svg_anim.plot_type {
          PlotType::PowerFn(_) => false,
          PlotType::DirectedPetGraph(g) => {
            g.update_simulation(SIM_SPEED);
            true
          },
          PlotType::UndirectedPetGraph(g) => {
            g.update_simulation(SIM_SPEED);
            true
          }
        }
      }
    }
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {

    html! {
      <>

        <h1>{ "Home" }</h1>

        <h2>{ "Woah, incredible splash graph!" }</h2>

        <div ref={self.div_svg_ref.clone()}/>

      </>
    }

  }

}
