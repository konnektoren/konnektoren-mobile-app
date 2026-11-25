use crate::route::Route;
use konnektoren_yew::navigation::{NavItem, NavigationBar, NavigationBarLayout};
use konnektoren_yew::prelude::SocialLinks;
use yew::prelude::*;

pub fn primary_links() -> Vec<NavItem<Route>> {
    vec![
        NavItem::Route {
            name: "Home",
            route: Route::Home,
            icon: html! { <i class="fa-solid fa-home" /> },
        },
        NavItem::Route {
            name: "Challenges",
            route: Route::Challenges,
            icon: html! { <i class="fa-solid fa-tasks" /> },
        },
    ]
}

pub fn social_links() -> Html {
    html! {
        <SocialLinks
            bluesky="https://bsky.app/profile/konnektoren.help"
            telegram="https://t.me/KonnektorenHelpBot"
            web="https://info.konnektoren.help"
            youtube="https://www.youtube.com/@KonnektorenHelp"
        />
    }
}

/// Bottom menu: main pages plus an overflow menu with the social media links.
#[function_component(Navigation)]
pub fn navigation() -> Html {
    let secondary = vec![NavItem::Component {
        component: social_links(),
    }];
    html! {
        <NavigationBar<Route>
            layout={NavigationBarLayout::Bottom}
            items={primary_links()}
            secondary_items={secondary}
            secondary_label="Others"
        />
    }
}
