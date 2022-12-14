mod home;

use yew::prelude::*;
use yew_router::prelude::*;


#[derive(Routable, Clone, PartialEq, Eq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
  match route {
    Route::Home => html! { <home::Home/> },
    Route::About => html! { <h1>{ "About" }</h1> },
    Route::NotFound => html! { <h1>{ "404" }</h1> },
  }
}
