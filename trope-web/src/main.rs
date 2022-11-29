use yew::prelude::*;

mod tab;


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
    <div>

      <tab::Tab name="home" title="Home"/>
      <tab::Tab name="about" title="About"/>

      <button { onclick }>{ "+1" }</button>
      <p>{ *counter }</p>

    </div>
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
