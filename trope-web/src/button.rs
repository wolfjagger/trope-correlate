use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
  pub children: Children,
  pub onmouseover: Option<Callback<MouseEvent>>,
  pub onmouseout: Option<Callback<MouseEvent>>,
  pub onmousedown: Option<Callback<MouseEvent>>,
  pub onmouseup: Option<Callback<MouseEvent>>,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {

  // Mouse events

  // TODO: Move is_mouseover and is_active to calculated values.
  //  But keep the mouse events.
  let is_mouseover = use_state_eq(|| false);
  let is_active = use_state_eq(|| false);

  let onmouseover = {
    let is_mouseover = is_mouseover.clone();
    match props.onmouseover.clone() {
      Some(listener) => Callback::from(move |e| {
        is_mouseover.set(true);
        listener.emit(e);
      }),
      None => Callback::from(move |_| {
        is_mouseover.set(true);
      })
    }
  };

  let onmouseout = {
    let is_mouseover = is_mouseover.clone();
    match props.onmouseout.clone() {
      Some(listener) => Callback::from(move |e| {
        is_mouseover.set(false);
        listener.emit(e);
      }),
      None => Callback::from(move |_| {
        is_mouseover.set(false);
      })
    }
  };

  let onmousedown = {
    let is_active = is_active.clone();
    match props.onmousedown.clone() {
      Some(listener) => Callback::from(move |e| {
        is_active.set(true);
        listener.emit(e);
      }),
      None => Callback::from(move |_| {
        is_active.set(true);
      })
    }
  };

  let onmouseup = {
    let is_active = is_active.clone();
    match props.onmouseup.clone() {
      Some(listener) => Callback::from(move |e| {
        is_active.set(false);
        listener.emit(e);
      }),
      None => Callback::from(move |_| {
        is_active.set(false);
      })
    }
  };

  // CSS
  let req_classes = vec!["label-button", "horiz"];
  let active_classes = [req_classes.clone(), vec!["active-button-style"]].concat();
  let inactive_classes = [req_classes.clone(), vec!["inactive-button-style"]].concat();
  let mouseover_classes = [req_classes, vec!["mouseover-button-style"]].concat();

  // HTML
  html! {
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
    >{ for props.children.iter() }</label>
  }

}
