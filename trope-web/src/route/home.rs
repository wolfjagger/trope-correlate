use gloo_timers::callback::Interval;
use web_sys::Element;
use yew::prelude::*;

use crate::{message::Message, plot::{SvgPlot, PlotType}};


const SIM_TIME_MILLI: u32 = 10;
const SIM_SPEED: f64 = 0.5;
const SIM_PER_TICK: f64 = SIM_SPEED * (SIM_TIME_MILLI as f64) / 1000.;
const MAX_SIM_MILLI: u32 = 5000;


pub struct Home {
  svg_anim: SvgPlot,
  div_svg_ref: NodeRef,
  anim_time: u32,
  _interval: Interval,
}

impl Component for Home {

  type Message = Message;
  type Properties = ();

  fn create(ctx: &Context<Self>) -> Self {

    let svg_anim = SvgPlot::undirected_graph();
    // div_svg_ref a NodeRef to the element in html that will contain the svg
    let div_svg_ref = NodeRef::default();
    let anim_time = 0;

    let callback = ctx.link().callback(|_| {
      Message::Tick
    });
    // TODO: Change to timeout & remove when over MAX_SIM_MILLI
    let interval = Interval::new(SIM_TIME_MILLI, move || callback.emit(()));

    Self {
      svg_anim,
      div_svg_ref,
      anim_time,
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
            if self.anim_time < MAX_SIM_MILLI {
              g.update_simulation(SIM_PER_TICK);
              self.anim_time += SIM_TIME_MILLI;
              true
            } else {
              false
            }
          },
          PlotType::UndirectedPetGraph(g) => {
            if self.anim_time < MAX_SIM_MILLI {
              g.update_simulation(SIM_PER_TICK);
              self.anim_time += SIM_TIME_MILLI;
              true
            } else {
              false
            }
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
