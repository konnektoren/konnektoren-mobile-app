use konnektoren_mobile_app_lib::prelude::App;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
