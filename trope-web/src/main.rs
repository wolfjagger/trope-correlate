use yew::prelude::*;
use yew_router::prelude::*;


mod button;
mod message;
mod plot;
mod route;
mod tab;
mod graph;


#[function_component]
fn App() -> Html {
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

fn main() {
  yew::Renderer::<App>::new().render();
}
