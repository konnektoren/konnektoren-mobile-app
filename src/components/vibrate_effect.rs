use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct VibrateEffectProps {
    pub duration: u32,
}

#[function_component(VibrateEffectComponent)]
pub fn vibration_effect(props: &VibrateEffectProps) -> Html {
    let duration = props.duration;

    use_effect(move || {
        let duration = duration;
        if let Some(window) = web_sys::window() {
            if window.navigator().vibrate_with_duration(duration) {
                log::info!("Vibration effect started");
            } else {
                log::error!("Vibration effect failed");
            }
        }
    });
    html! {
        <>
        </>
    }
}
