use crate::route::Route;
use konnektoren_core::game::Game;
use konnektoren_yew::i18n::use_i18n;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(ChallengesPage)]
pub fn challenges_page() -> Html {
    let i18n = use_i18n();
    let game = use_memo((), |_| Game::default());

    let items = game
        .game_paths
        .iter()
        .flat_map(|path| path.challenges.iter())
        .map(|config| {
            html! {
                <li key={config.id.clone()} class="challenges-page__item">
                    <Link<Route> to={Route::Challenge { id: config.id.clone() }} classes="challenges-page__link">
                        <span class="challenges-page__name">{ i18n.t(&config.name) }</span>
                        <span class="challenges-page__description">{ i18n.t(&config.description) }</span>
                    </Link<Route>>
                </li>
            }
        })
        .collect::<Html>();

    html! {
        <main class="challenges-page" aria-labelledby="challenges-title">
            <h1 id="challenges-title" class="challenges-page__title">{ i18n.t("Challenges") }</h1>
            <ul class="challenges-page__list">{ items }</ul>
        </main>
    }
}
