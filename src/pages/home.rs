use leptos::prelude::*;
use leptos_meta::{Link, Meta, Title};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text=crate::seo::SITE_TITLE/>
        <Link rel="canonical" href=crate::routes::home_url()/>
        <Meta name="description" content=crate::seo::SITE_DESCRIPTION/>
        <Meta property="og:title" content=crate::seo::SITE_TITLE/>
        <Meta property="og:type" content="website"/>
        <Meta property="og:url" content=crate::routes::home_url()/>
        <Meta property="og:description" content=crate::seo::SITE_DESCRIPTION/>
        <Meta property="og:site_name" content=crate::seo::SITE_TITLE/>
        <main>
            <h1>{crate::seo::SITE_TITLE}</h1>
            <p>
                "Traditional sporadic software developer blogging. I'm currently looking for a new position, "
                <a href=crate::seo::LINKEDIN_URL>"hire me"</a>
                ". I am also available for consulting on software design and development, increasing developer productivity on small development teams, and reducing friction between development teams and business teams."
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
                            <h3 class="h4-size"><a href=crate::routes::post(&post.slug)>{post.title.clone()}</a></h3>
                            <span class="post-date">{date}</span>
                            {description}
                        </div>
                    }
                })
                .collect::<Vec<_>>()}
            <crate::app::JsonLd data=crate::seo::home_json_ld()/>
            <crate::app::AnalyticsScripts/>
        </main>
    }
}
