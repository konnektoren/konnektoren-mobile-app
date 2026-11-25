use crate::route::Route;
use konnektoren_core::challenges::ChallengeConfig;
use konnektoren_core::game::Game;
use konnektoren_yew::components::challenge::ChallengeCard;
use konnektoren_yew::i18n::use_i18n;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(ChallengesPage)]
pub fn challenges_page() -> Html {
    let i18n = use_i18n();
    let game = use_memo((), |_| Game::default());
    let navigator = use_navigator();

    let on_select = Callback::from(move |config: ChallengeConfig| {
        if let Some(navigator) = &navigator {
            navigator.push(&Route::Challenge { id: config.id });
        }
    });

    let sections = game
        .game_paths
        .iter()
        .map(|path| {
            let cards = path
                .challenges
                .iter()
                .map(|config| {
                    html! {
                        <li key={config.id.clone()} class="challenges-page__item">
                            <ChallengeCard
                                challenge_config={config.clone()}
                                on_select={on_select.clone()}
                                flippable={false}
                            />
                        </li>
                    }
                })
                .collect::<Html>();
            html! {
                <section key={path.id.clone()} class="challenges-page__section">
                    <h2 class="challenges-page__level">{ i18n.t(&path.name) }</h2>
                    <ul class="challenges-page__list">{ cards }</ul>
                </section>
            }
        })
        .collect::<Html>();

    html! {
        <main class="challenges-page" aria-labelledby="challenges-title">
            <header class="challenges-page__header">
                <h1 id="challenges-title" class="challenges-page__title">
                    <i class="fa-solid fa-tasks" aria-hidden="true"></i>
                    <span>{ i18n.t("Challenges") }</span>
                </h1>
            </header>
            { sections }
        </main>
    }
}
