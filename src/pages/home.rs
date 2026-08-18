use leptos::prelude::*;
use leptos_meta::{Link, Meta, Title};

/// The home page's meta description — the snippet search engines show for it.
const DESCRIPTION: &str = "Russell Duhon's blog on software development: Rust, property-based testing, PostgreSQL, WebAssembly, and Python.";

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text=crate::seo::AUTHOR/>
        <Link rel="canonical" href=crate::routes::absolute("/")/>
        <Meta name="description" content=DESCRIPTION/>
        <Meta property="og:title" content=crate::seo::AUTHOR/>
        <Meta property="og:type" content="website"/>
        <Meta property="og:url" content=crate::routes::absolute("/")/>
        <Meta property="og:description" content=DESCRIPTION/>
        <Meta property="og:site_name" content=crate::seo::SITE_TITLE/>
        <main>
            <h1>"Russell Duhon"</h1>
            <p>
                "Traditional sporadic software developer blogging. I'm currently looking for a new position, "
                <a href="https://www.linkedin.com/in/russell-duhon-322a0244">"hire me"</a>
                "."
            </p>
            <h2 class="h3-size">"Blog"</h2>
            {crate::posts::all()
                .iter()
                .map(|post| {
                    let date = post.date.date_naive().format("%-d %B %C%y").to_string();
                    let description = post
                        .description
                        .clone()
                        .map(|description| view! { <blockquote>{description}</blockquote> });
                    view! {
                        <div style="margin-bottom: 3em;">
                            <h4><a href=crate::routes::post(&post.slug)>{post.title.clone()}</a></h4>
                            <h6 style="display: inline"><span>{date}</span></h6>
                            {description}
                        </div>
                    }
                })
                .collect::<Vec<_>>()}
            <crate::app::AnalyticsScripts/>
        </main>
    }
}
