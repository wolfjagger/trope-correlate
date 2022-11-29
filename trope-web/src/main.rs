use yew::prelude::*;
use yew_router::prelude::*;

mod tab;
mod route;


#[function_component]
fn Counter() -> Html {

  let counter = use_state(|| 0);

  let onclick = {
    let counter = counter.clone();
    move |_| {
      let value = *counter + 1;
      counter.set(value);
    }
  };

  html! {
    <BrowserRouter>
      <div>

        <tab::Tab route={route::Route::Home} title="Home"/>
        <tab::Tab route={route::Route::About} title="About"/>

        <button { onclick }>{ "+1" }</button>
        <p>{ *counter }</p>

        <Switch<route::Route> render={route::switch} />

      </div>
    </BrowserRouter>
  }

}

#[function_component]
fn App() -> Html {

  html! {
    <Counter/>
  }

}

fn main() {
  yew::Renderer::<App>::new().render();
}
