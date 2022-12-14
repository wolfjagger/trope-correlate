mod about;
mod home;
mod method;
mod trope;

use yew::prelude::*;
use yew_router::prelude::*;


#[derive(Routable, Clone, PartialEq, Eq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/trope-relationship")]
    TropeRelationship,
    #[at("/method")]
    Method,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
  match route {
    Route::Home => html! { <home::Home/> },
    Route::TropeRelationship => html! { <trope::TropeRelationship/> },
    Route::Method => html! { <method::Method/> },
    Route::About => html! { <about::About/> },
    Route::NotFound => html! { <h1>{ "404" }</h1> },
  }
}
