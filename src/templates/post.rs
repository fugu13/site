use crate::data::Post;
use perseus::prelude::*;
use sycamore::prelude::*;

#[auto_scope]
fn post_page<G: Html>(cx: Scope, state: &crate::data::PostRx) -> View<G> {
    view! { cx,
        div {
            h6 { a(href="/") { "home" }}
            h1 { (state.title.get()) }
            div(
                dangerously_set_inner_html = &state.html.get()
            )
            h6 { a(href="/") { "home" }}
            script(src="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/components/prism-core.min.js")
            script(src="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/plugins/autoloader/prism-autoloader.min.js")
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope, state: Post) -> View<SsrNode> {
    view! { cx,
        title { (state.title) }
        link(rel="stylesheet", href="https://unpkg.com/sakura.css/css/sakura.css", media="screen")
        link(rel="stylesheet", href=".perseus/static/extra.css")
        link(rel="stylesheet", href="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/themes/prism.min.css")
    }
}

#[engine_only_fn]
async fn get_build_paths() -> BuildPaths {
    BuildPaths {
        // These will all become URLs at `/post/<name>`
        paths: crate::data::get_blog_directories(),
        // Perseus supports helper state, but we don't need it here
        extra: ().into(),
    }
}

#[engine_only_fn]
async fn get_build_state(StateGeneratorInfo { path, .. }: StateGeneratorInfo<()>) -> Post {
    // TODO do I need a way to 404 here? What happens if we request something missing?
    crate::data::get_post_for_path(&path)
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("post")
        .build_paths_fn(get_build_paths)
        .build_state_fn(get_build_state)
        .view_with_state(post_page)
        .head_with_state(head)
        .build()
}
