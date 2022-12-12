use gloo_timers::callback::Interval;
use yew::prelude::*;
use yew_router::prelude::*;

mod button;
mod message;
mod plot;
mod route;
mod tab;
mod graph;


struct App {
  _interval: Interval
}

impl Component for App {

  type Message = message::Message;
  type Properties = ();

  fn create(ctx: &Context<Self>) -> Self {
    let callback = ctx.link().callback(|_| message::Message::Tick);
    let interval = Interval::new(200, move || callback.emit(()));
    Self {
      _interval: interval
    }
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {

    html! {
      <BrowserRouter>

        <nav>
          <tab::Tab route={route::Route::Home}>{ "Home" }</tab::Tab>
          <tab::Tab route={route::Route::TropeRelationship}>{ "Trope Relationships" }</tab::Tab>
          <tab::Tab route={route::Route::Method}>{ "Methods" }</tab::Tab>
          <tab::Tab route={route::Route::About}>{ "About" }</tab::Tab>
        </nav>

        <Switch<route::Route> render={route::switch} />

      </BrowserRouter>
    }
  }

}

fn main() {
  yew::Renderer::<App>::new().render();
}
