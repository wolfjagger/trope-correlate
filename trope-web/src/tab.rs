use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct TabProps {
  pub name: String,
  pub title: String,
}

#[function_component]
pub fn Tab(props: &TabProps) -> Html {

  let onclick = {
    |_| {
      todo!("Implement tab navigation");
    }
  };

  html! {
    <button class="tablinks" { onclick }>{&props.title}</button>
  }

}
