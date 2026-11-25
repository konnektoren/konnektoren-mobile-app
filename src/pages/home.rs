use crate::components::social_links;
use crate::route::Route;
use konnektoren_yew::components::Logo;
use konnektoren_yew::i18n::use_i18n;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let i18n = use_i18n();

    html! {
        <main class="home-page" aria-labelledby="home-title">
            <section class="home-page__hero">
                <Link<Route> to={Route::Challenges} classes="home-page__mascot-link">
                    <Logo img_src="/assets/images/Orange_Animated.svg" />
                </Link<Route>>
                <p class="home-page__eyebrow">{ i18n.t("Your Adventure in German Grammar") }</p>
                <h1 id="home-title" class="home-page__title">{ i18n.t("Welcome to Konnektoren!") }</h1>
                <div class="home-page__actions">
                    <Link<Route> to={Route::Challenges} classes="home-page__primary-action">
                        <i class="fa-solid fa-play" aria-hidden="true"></i>
                        <span>{ i18n.t("Start Learning") }</span>
                    </Link<Route>>
                </div>
            </section>
            <section class="home-page__social" aria-label={i18n.t("Follow us")}>
                <p class="home-page__social-title">{ i18n.t("Follow us") }</p>
                { social_links() }
            </section>
        </main>
    }
}
