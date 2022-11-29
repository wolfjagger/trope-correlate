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

  let mouseover = use_state_eq(|| false);

  let onmouseover = {
    let mouseover = mouseover.clone();
    move |_| {
      mouseover.set(true);
    }
  };

  let onmouseout = {
    let mouseover = mouseover.clone();
    move |_| {
      mouseover.set(false);
    }
  };

  let req_classes = vec!["label-button", "horiz"];
  let inactive_classes = [req_classes.clone(), vec!["inactive-button-style"]].concat();
  let mouseover_classes = [req_classes, vec!["mouseover-button-style"]].concat();

  html! {
    <Link<route::Route> classes={classes!("tab-link")} to={props.route.clone()}>
      <label
        class={
          if *mouseover {
            classes!(mouseover_classes)
          } else {
            classes!(inactive_classes)
          }
        }
        { onmouseover } { onmouseout }
      >{ &props.title }</label>
    </Link<route::Route>>
  }

}
