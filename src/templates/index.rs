use crate::data::Post;
use perseus::prelude::*;
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
                    a(href="https://www.linkedin.com/in/russell-duhon-322a0244") { "hire me" }
                    "."
                }
            }
            div {
                h3 { "Blog"}

                div {
                    Keyed(
                        iterable=&state.posts,
                        view=|cx, post| view! { cx,
                            div(style="margin-bottom: 3em;") {
                                h4 {
                                    a(href=format!("post/{}/", post.path)) { (post.title.clone()) }
                                }
                                h6(style="display: inline") {
                                    span { (post.date.date_naive().format("%-d %B %C%y")) }
                                }
                                (if let Some(description) = post.description.clone() {
                                    view! { cx,
                                        blockquote {
                                            (description)
                                        }
                                    }
                                } else {
                                    view! { cx, }
                                })
                            }
                        },
                        key=|post| post.path.clone()
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
        link(rel="stylesheet", href="https://unpkg.com/sakura.css/css/sakura.css", media="screen")
        link(rel="stylesheet", href=".perseus/static/extra.css")
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
