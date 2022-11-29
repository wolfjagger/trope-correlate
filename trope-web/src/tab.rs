use yew::prelude::*;
use yew_router::prelude::*;

use super::route;


#[derive(Properties, PartialEq)]
pub struct TabProps {
  pub route: route::Route,
  pub title: String,
}

#[function_component]
pub fn Tab(props: &TabProps) -> Html {

  html! {
    <Link<route::Route> classes={classes!("tab-link")} to={props.route.clone()}>
      <button class="tab-button">{ &props.title }</button>
    </Link<route::Route>>
  }

}
