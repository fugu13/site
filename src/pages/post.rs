use leptos::prelude::*;
use leptos_meta::{Link, Meta, Title};
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
        Some(post) => {
            let canonical = routes::post_url(&post.slug);
            view! {
                <Title text=crate::seo::page_title(&post.title)/>
                <Link rel="canonical" href=canonical.clone()/>
                <Meta property="og:title" content=format!("{} by {}", post.title, crate::seo::AUTHOR)/>
                <Meta property="og:type" content="article"/>
                <Meta property="og:url" content=canonical/>
                <Meta property="article:published_time" content=post.date.to_rfc3339()/>
                {post.image.as_ref().map(|image| {
                    view! { <Meta property="og:image" content=routes::absolute(image)/> }
                })}
                {post.description.as_deref().map(|description| {
                    view! {
                        <Meta name="description" content=description/>
                        <Meta property="og:description" content=description/>
                    }
                })}
                <Meta property="og:site_name" content=crate::seo::SITE_TITLE/>
                <main>
                    <h6><a href="/">"home"</a></h6>
                    <h1>{post.title.clone()}</h1>
                    <div inner_html=post.html.clone()></div>
                    <h6><a href="/">"home"</a></h6>
                    <script type="application/ld+json" inner_html=crate::seo::blog_posting_json_ld(post)></script>
                    <crate::app::AnalyticsScripts/>
                </main>
            }
            .into_any()
        }
    }
}
