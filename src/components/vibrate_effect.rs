use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct VibrateEffectProps {
    pub duration: u32,
}

#[function_component(VibrateEffectComponent)]
pub fn vibration_effect(props: &VibrateEffectProps) -> Html {
    let duration = props.duration;

    use_effect(move || {
        if let Some(window) = web_sys::window() {
            let navigator = window.navigator();

            // Detect if the user agent is Chrome on Android
            if let Ok(user_agent) = navigator.user_agent() {
                if user_agent.contains("Android") && user_agent.contains("Chrome") {
                    if navigator.vibrate_with_duration(duration) {
                    } else {
                        log::error!("Vibration effect failed");
                    }
                } else {
                    log::debug!("Not Chrome on Android, skipping vibration");
                }
            }
        }
        || () // Return an empty cleanup function
    });

    html! {
        <>
        </>
    }
}
