use yew::prelude::*;


#[function_component]
pub fn About() -> Html {

  // Mouse events

  // HTML
  html! {
    <>

      <h1>{ "About" }</h1>

      <p>{ "Two dudes, looking at tropes" }</p>

      <section class="person">
        <h2>{ "Joe Mitchell" }</h2>
        <p>{ "Wow he's so cool" }</p>
      </section>

      <section class="person">
        <h2>{ "Trystan Koch" }</h2>
        <p>{ "Wow he's so cool" }</p>
      </section>

    </>
  }

}
