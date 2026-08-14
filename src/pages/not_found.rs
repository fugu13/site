use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <Title text="Page not found"/>
        <main>
            <h1>"404"</h1>
            <p>
                "That page does not exist. "
                <a href="/">"Go home"</a>
                "."
            </p>
        </main>
    }
}
