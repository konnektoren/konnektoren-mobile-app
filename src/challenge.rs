use crate::prelude::ChallengeEffectComponent;
use crate::route::Route;
use konnektoren_core::challenges::{Challenge, ChallengeResult, Timed};
use konnektoren_core::game::Game;
use konnektoren_yew::components::challenge::ResultSummaryComponent;
use konnektoren_yew::i18n::use_i18n;
use yew::prelude::*;
use yew_router::prelude::Link;

#[derive(Properties, PartialEq)]
pub struct ChallengeCompProps {
    pub id: String,
    pub address: String,
}

#[function_component(ChallengeComp)]
pub fn challenge_comp(props: &ChallengeCompProps) -> Html {
    let i18n = use_i18n();
    // Finished copy of the challenge (with end time) and its result.
    let finished = use_state(|| Option::<(Challenge, ChallengeResult)>::None);
    let game = Game::default();
    // Created and started once per mount (the route keys this component by id), so the
    // timer has a start time and does not reset on re-render.
    let challenge = use_state(|| {
        let mut challenge = game.create_challenge(&props.id).unwrap_or_default();
        challenge.start();
        challenge
    });

    let back = html! {
        <Link<Route> to={Route::Challenges} classes="challenge-page__back">
            <i class="fa-solid fa-arrow-left" aria-hidden="true"></i>
            <span>{ i18n.t("Challenges") }</span>
        </Link<Route>>
    };

    let Some((game_path, challenge_config)) = game.game_paths.iter().find_map(|path| {
        path.get_challenge_config(&props.id)
            .map(|config| (path, config))
    }) else {
        return html! {
            <div class="challenge-page">
                <header class="challenge-page__header">{ back }</header>
                <p class="challenge-page__missing">{ format!("{}: {}", i18n.t("Challenge not found"), props.id) }</p>
            </div>
        };
    };

    let handle_finish = {
        let finished = finished.clone();
        let challenge = (*challenge).clone();
        Callback::from(move |result: ChallengeResult| {
            let mut challenge = challenge.clone();
            challenge.update_end_time();
            challenge.challenge_result = Some(result.clone());
            finished.set(Some((challenge, result)));
        })
    };

    let result_summary = match &*finished {
        Some((finished_challenge, result)) => html! {
            <div class="challenge-page__result">
                <ResultSummaryComponent challenge={finished_challenge.clone()} challenge_result={result.clone()} />
            </div>
        },
        None => html! {},
    };

    html! {
        <div class="challenge-page">
            <header class="challenge-page__header">
                { back }
                <h1 class="challenge-page__title">
                    <span class="challenge-page__level">{ i18n.t(&game_path.name) }</span>
                    <span class="challenge-page__name">{ i18n.t(&challenge_config.name) }</span>
                </h1>
            </header>
            <div class="challenge-page__content">
                {result_summary}
                <ChallengeEffectComponent challenge={(*challenge).clone()} variant={challenge_config.variant.clone()} on_finish={handle_finish} />
            </div>
        </div>
    }
}
