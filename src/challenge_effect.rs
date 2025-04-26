use crate::components::VibrateEffectComponent;
use gloo::timers::callback::Timeout;
use konnektoren_core::challenges::challenge_variant::ChallengeVariant;
use konnektoren_core::commands::{ChallengeCommand, Command};
use konnektoren_core::events::{ChallengeEvent, Event};
use konnektoren_core::prelude::{Challenge, ChallengeResult};
use konnektoren_yew::components::{ChallengeComponent, MusicComponent};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub challenge: Challenge,
    #[prop_or_default]
    pub variant: Option<ChallengeVariant>,
    #[prop_or_default]
    pub on_finish: Option<Callback<ChallengeResult>>,
}

#[function_component(ChallengeEffectComponent)]
pub fn challenge_effect_component(props: &Props) -> Html {
    let challenge = props.challenge.clone();
    let effect_ref = use_state(|| html! { <div></div> });
    let counter = use_state(|| 0);

    let handle_command = {
        let on_finish = props.on_finish.clone();
        Callback::from(move |command: Command| {
            if let Command::Challenge(challenge_command) = command {
                match challenge_command {
                    ChallengeCommand::Finish(result) => {
                        if let Some(on_finish) = on_finish.as_ref() {
                            if let Some(challenge_result) = result {
                                on_finish.emit(challenge_result);
                            }
                        }
                    }
                    _ => {
                        // Handle other challenge commands if needed
                    }
                }
            }
        })
    };

    let handle_event = {
        let effect_ref = effect_ref.clone();
        let counter = counter.clone();
        Callback::from(move |event: Event| {
            if let Event::Challenge(challenge_event) = event {
                match challenge_event {
                    ChallengeEvent::SolvedCorrect(_index) => {
                        let c = *counter + 1;
                        counter.set(c);
                        let effect = html! {
                            <>
                                <div class="challenge-effect__blink challenge-effect__blink--correct"></div>
                                <MusicComponent id={format!("challenge-effect__music--correct-{}", c)} url="/music/UI Positive Signal 002.wav" repeat={false} />
                            </>
                        };
                        effect_ref.set(effect);
                        let effect_ref_clone = effect_ref.clone();
                        Timeout::new(2000, move || {
                            effect_ref_clone.set(html! { <div></div> });
                        })
                        .forget();
                    }
                    ChallengeEvent::SolvedIncorrect(_index) => {
                        let c = *counter + 1;
                        counter.set(c);
                        let effect = html! {
                            <>
                                <div class="challenge-effect__blink challenge-effect__blink--incorrect"></div>
                                <MusicComponent id={format!("challenge-effect__music--incorrect-{}", c)} url="/music/UI Negative Signal 003.wav" repeat={false} />
                                <VibrateEffectComponent duration={100} key={c} />
                            </>
                        };
                        effect_ref.set(effect);
                        let effect_ref_clone = effect_ref.clone();
                        Timeout::new(2000, move || {
                            effect_ref_clone.set(html! { <div></div> });
                        })
                        .forget();
                    }
                    ChallengeEvent::Started => {
                        // Handle challenge start if needed
                    }
                    ChallengeEvent::Completed => {
                        // Handle challenge completion if needed
                    }
                }
            }
        })
    };

    html! {
        <div id={format!("challenge-effect-{}", *counter)}>
        {(*effect_ref).clone()}
        <ChallengeComponent challenge={challenge.clone()} variant={props.variant.clone()} on_command={handle_command} on_event={handle_event} />
        </div>
    }
}
