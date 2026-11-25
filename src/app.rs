use crate::components::Navigation;
use crate::route::{Route, switch_main};
use konnektoren_yew::i18n::{I18nConfig, I18nProvider};
use konnektoren_yew::prelude::{
    DefaultSessionInitializer, RepositoryProvider, create_repositories,
};
use konnektoren_yew::repository::LocalStorage;
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::HashRouter;
use yew_router::prelude::Switch;

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

    // HashRouter: the Tauri webview serves only index.html, so path routes would 404 on reload.
    html! {
        <RepositoryProvider config={repository_config}>
            <I18nProvider config={i18n_config}>
                <HashRouter>
                    <div class="container app">
                        <Switch<Route> render={switch_main} />
                    </div>
                    <Navigation />
                </HashRouter>
            </I18nProvider>
        </RepositoryProvider>
    }
}
