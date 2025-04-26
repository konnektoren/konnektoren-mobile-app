use crate::prelude::ChallengeEffectComponent;
use konnektoren_core::{challenges::ChallengeResult, game::Game};
use konnektoren_yew::components::challenge::ResultSummaryComponent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChallengeCompProps {
    pub id: String,
    pub address: String,
}

#[function_component(ChallengeComp)]
pub fn challenge_comp(props: &ChallengeCompProps) -> Html {
    let game = Game::default();
    let challenge_result = use_state(|| Option::<ChallengeResult>::None);
    let challenge = game.create_challenge(&props.id).unwrap_or_default();
    let feedback_class = use_state(|| "".to_string());
    let challenge_config = game.game_paths[0].get_challenge_config(&props.id).unwrap();

    let handle_finish = {
        let challenge_result = challenge_result.clone();
        Callback::from(move |result: ChallengeResult| {
            challenge_result.set(Some(result.clone()));
        })
    };

    let result_summary = match &*challenge_result {
        Some(result) => {
            html! {
                <ResultSummaryComponent challenge={challenge.clone()} challenge_result={result.clone()} />
            }
        }
        None => html! {},
    };
    html! {
        <div class={classes!("challenge-page", (*feedback_class).clone())}>
            {result_summary}
            <ChallengeEffectComponent challenge={challenge.clone()} variant={challenge_config.variant.clone()} on_finish={handle_finish} />
        </div>
    }
}
