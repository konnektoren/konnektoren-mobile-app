use crate::pages::{ChallengesPage, HomePage};
use crate::prelude::ChallengeComp;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/challenges")]
    Challenges,
    #[at("/challenge/:id")]
    Challenge { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch_main(route: Route) -> Html {
    match route {
        Route::Home | Route::NotFound => html! { <HomePage /> },
        Route::Challenges => html! { <ChallengesPage /> },
        Route::Challenge { id } => {
            let key = id.clone();
            html! { <ChallengeComp {key} {id} address="" /> }
        }
    }
}
