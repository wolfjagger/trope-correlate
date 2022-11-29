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

  // Mouse events

  let is_mouseover = use_state_eq(|| false);
  let is_active = use_state_eq(|| false);

  let onmouseover = {
    let is_mouseover = is_mouseover.clone();
    move |_| {
      is_mouseover.set(true);
    }
  };

  let onmouseout = {
    let is_mouseover = is_mouseover.clone();
    move |_| {
      is_mouseover.set(false);
    }
  };

  let onmousedown = {
    let is_active = is_active.clone();
    move |_| {
      is_active.set(true);
    }
  };

  let onmouseup = {
    let is_active = is_active.clone();
    move |_| {
      is_active.set(false);
    }
  };

  // CSS
  let req_classes = vec!["label-button", "horiz"];
  let active_classes = [req_classes.clone(), vec!["active-button-style"]].concat();
  let inactive_classes = [req_classes.clone(), vec!["inactive-button-style"]].concat();
  let mouseover_classes = [req_classes, vec!["mouseover-button-style"]].concat();

  // HTML
  html! {
    <Link<route::Route> classes={classes!("tab-link")} to={props.route.clone()}>
      <label
        class={
          if *is_active {
            classes!(active_classes)
          } else if *is_mouseover {
            classes!(mouseover_classes)
          } else {
            classes!(inactive_classes)
          }
        }
        { onmouseover } { onmouseout }
        { onmousedown } { onmouseup }
      >{ &props.title }</label>
    </Link<route::Route>>
  }

}
