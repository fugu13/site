use crate::data::Post;
use perseus::prelude::*;
use pretty_date::pretty_date_formatter::PrettyDateFormatter;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, Clone, ReactiveState)]
#[rx(alias = "IndexRx")]
struct Index {
    posts: Vec<Post>,
}

#[auto_scope]
fn index_page<G: Html>(cx: Scope, state: &IndexRx) -> View<G> {
    view! { cx,
        // Don't worry, there are much better ways of styling in Perseus!
        div {
            h1 { "Russell Duhon" }
            div {
                p {
                    "Traditional sporadic software developer blogging. I'm currently looking for a new position, "
                    a(href="mailto:fugu13@gmail.com") { "hire me" }
                    "."
                }
            }
            div {
                h3 { "Blog"}

                ul {
                    Keyed(
                        iterable=&state.posts,
                        view=|cx, post| view! { cx,
                            li {
                                a(href=format!("post/{}", post.path)) { (post.title.clone()) }
                                " "
                                span { (post.date.clone().naive_local().format_pretty()) }
                            }
                        },
                        key=|post| post.title.clone()
                    )
                }
            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Russell Duhon" }
        link(rel="stylesheet", href="https://cdn.jsdelivr.net/npm/sakura.css/css/sakura.css")
    }
}

#[engine_only_fn]
async fn get_build_state(_generator: StateGeneratorInfo<()>) -> Index {
    let mut posts: Vec<Post> = crate::data::get_blog_directories()
        .iter()
        .map(crate::data::get_post_for_path)
        .collect();
    posts.sort_by_key(|post| post.date.clone());
    Index {
        posts: posts.into_iter().rev().collect(),
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index")
        .build_state_fn(get_build_state)
        .view_with_state(index_page)
        .head(head)
        .build()
}
