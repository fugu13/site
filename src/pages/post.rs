use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos_router::hooks::use_params_map;

use crate::pages::not_found::NotFoundPage;
use crate::posts;
use crate::routes;

#[component]
pub fn PostPage() -> impl IntoView {
    let slug = use_params_map()
        .with(|params| params.get("slug"))
        .unwrap_or_default();

    match posts::by_slug(&slug) {
        None => view! { <NotFoundPage/> }.into_any(),
        Some(post) => view! {
            <Title text=post.title.clone()/>
            <Meta property="og:title" content=format!("{} by Russell Duhon", post.title)/>
            <Meta property="og:type" content="article"/>
            <Meta property="article:published_time" content=post.date.to_rfc3339()/>
            {post.image.as_ref().map(|image| {
                let content = format!("{}{}", routes::DOMAIN, image);
                view! { <Meta property="og:image" content=content/> }
            })}
            {post.description.clone().map(|description| {
                view! { <Meta property="og:description" content=description/> }
            })}
            <Meta property="og:site_name" content="Russell Duhon's Blog"/>
            <main>
                <h6><a href="/">"home"</a></h6>
                <h1>{post.title.clone()}</h1>
                <div inner_html=post.html.clone()></div>
                <h6><a href="/">"home"</a></h6>
                <crate::app::AnalyticsScripts/>
            </main>
        }
        .into_any(),
    }
}
