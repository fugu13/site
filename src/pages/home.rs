use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="Russell Duhon"/>
        <main>
            <h1>"Russell Duhon"</h1>
            <p>
                "Traditional sporadic software developer blogging. I'm currently looking for a new position, "
                <a href="https://www.linkedin.com/in/russell-duhon-322a0244">"hire me"</a>
                "."
            </p>
            <h3>"Blog"</h3>
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
