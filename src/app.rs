use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, provide_meta_context};
use leptos_router::{
    ParamSegment, SsrMode, StaticSegment,
    components::{Route, Router, Routes},
    static_routes::{StaticParamsMap, StaticRoute},
};

pub fn shell() -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link
                    rel="alternate"
                    type="application/atom+xml"
                    title=crate::seo::SITE_TITLE
                    href=crate::seo::FEED_PATH
                />
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

/// A JSON-LD structured-data block — the one inline script form the site's
/// no-JavaScript convention permits: inert data for search engines, never
/// executed. The prerender strip step and the CI check both recognize exactly
/// this element shape, so every page emits it through this one component.
#[component]
pub fn JsonLd(data: String) -> impl IntoView {
    view! { <script type="application/ld+json" inner_html=data></script> }
}

/// The site's three analytics scripts, shared by every content page so
/// there's exactly one place that names them.
#[component]
pub fn AnalyticsScripts() -> impl IntoView {
    view! {
        <script defer=true src="https://app.tinyanalytics.io/pixel/MB6jAtnTO5M0SZ9n"></script>
        <script defer=true async=true src="https://scripts.simpleanalyticscdn.com/latest.js"></script>
        <script defer=true src="https://plausible.io/js/script.js" data-domain="russellduhon.com"></script>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="site" href=crate::assets::stylesheet_path()/>
        <Router>
            <Routes fallback=|| view! { <crate::pages::not_found::NotFoundPage/> }>
                <Route
                    path=StaticSegment("")
                    view=crate::pages::home::HomePage
                    ssr=SsrMode::Static(StaticRoute::new())
                />
                <Route
                    path=(StaticSegment(crate::routes::POST), ParamSegment("slug"))
                    view=crate::pages::post::PostPage
                    ssr=SsrMode::Static(StaticRoute::new().prerender_params(|| async {
                        let mut params = StaticParamsMap::new();
                        params.insert("slug", crate::posts::slugs());
                        params
                    }))
                />
                <Route
                    path=StaticSegment(crate::routes::NOT_FOUND)
                    view=crate::pages::not_found::NotFoundPage
                    ssr=SsrMode::Static(StaticRoute::new())
                />
            </Routes>
        </Router>
    }
}
