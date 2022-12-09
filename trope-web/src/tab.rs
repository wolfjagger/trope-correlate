use yew::prelude::*;
use yew_router::prelude::*;

use super::{route, button::Button};


#[derive(Properties, PartialEq)]
pub struct TabProps {
  pub route: route::Route,
  pub children: Children,
}

#[function_component]
pub fn Tab(props: &TabProps) -> Html {

  // HTML
  html! {
    <Link<route::Route> classes={classes!("tab-link")} to={props.route.clone()}>
      <Button>{ for props.children.iter() }</Button>
    </Link<route::Route>>
  }

}
