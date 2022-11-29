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
    <Link<route::Route> classes={classes!("tablinks")} to={props.route.clone()}>
      { &props.title }
    </Link<route::Route>>
  }

}
