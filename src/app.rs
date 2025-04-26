use crate::prelude::ChallengeComp;
use konnektoren_yew::i18n::{I18nConfig, I18nProvider};
use konnektoren_yew::prelude::{
    create_repositories, DefaultSessionInitializer, RepositoryProvider,
};
use konnektoren_yew::repository::LocalStorage;
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[function_component(App)]
pub fn app() -> Html {
    let i18n_config = I18nConfig::default();

    let storage = LocalStorage::new(None);
    let session_initilizer = DefaultSessionInitializer;
    let repository_config = create_repositories(storage, Arc::new(session_initilizer));

    html! {
        <main class="container">
            <RepositoryProvider config={repository_config}>
            <I18nProvider config={i18n_config}>
                <ChallengeComp id={"konnektoren-1"} address={""} />
            </I18nProvider>
            </RepositoryProvider>
        </main>
    }
}
