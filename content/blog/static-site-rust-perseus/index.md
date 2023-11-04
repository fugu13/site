---
title: Writing a Static Site with Rust and Perseus
date: '2023-11-02T11:30:00-08:00'
description: The basics of defining pages, making templates, and filling them with data, all using Rust that gets turned into WebAssembly.
---

Like many a developer, I've experimented with a bunch of static site generators over time. Right now I'm using
one called [Perseus](https://framesurge.sh/perseus/en-US/), which is written in Rust and generates WASM. I like Perseus because it
can also do non-static site generation, because it gives me a chance to practice Rust, and because it focuses on
doing things that make for really fast sites while letting me pretty much do what I want with the content.

![sculpture of medusa's head](.perseus/static/medusa.jpg)

## Basics

Like most site generators, Perseus needs to be configured to answer a few big questions.

1. What are the pages?
2. What's the content on the pages?
3. What will the pages look like?

Unlike a typical static site generator, Perseus can be configured to answer most of these questions when the code
is built, or when the page is requested on the server, or when the page is requested but in the browser, or in
combinations of these.

## Details

In Perseus, all these are configured in a `Template` that ultimately makes a model of a page, and is configured
via user-defined functions that handle each step. Here's the `Template` defining the blog page you're reading
this on.

```rust
 Template::build("post")
    .build_paths_fn(get_build_paths)
    .build_state_fn(get_build_state)
    .view_with_state(post_page)
    .head_with_state(head)
    .build()
```

And here's what each part of the template does.

```rust
 Template::build("post")
```

First, I define the base path I'm building here. Side-note: `index` is special, it means the empty string, aka the root.
In this example, the base path in the URL will be `/post`.

```rust
    .build_paths_fn(get_build_paths)
```

Next, I define all the sub-paths that will happen. To do this I wrote a function I named `get_build_paths`,
and what that function does is look in a directory that my I wrote my blog posts in and find and return all the blog
post "slugs". So if a blog post has the slug `my-awesome-post`, the complete url will be `/post/my-awesome-post`.
I won't share the exact code, but a key thing is that none of the code has anything to do with Perseus, it's just
typical Rust code for reading from the file system.


```rust
    .build_state_fn(get_build_state)
```

For each sub-path, we need to know what will go on the page--the initial state. I wrote a function named `get_build_state`,
and that function takes one path from `get_build_paths` (such as `my-awesome-post`) and figures out the state for it.
In my case, it reads the markdown file for the blog post, renders the markdown to HTML, and also pulls some pieces
from the markdown header like the title and date. Here's the data structure it returns.

```rust
pub struct Post {
    pub title: String,
    pub date: chrono::DateTime<FixedOffset>,
    pub html: String,
    pub path: String,
}
```

Again, none of the code in the `get_build_state` function is Perseus-specific, it uses typical Rust file reading code and a Rust
markdown to html library, plus some code to deserialize the YAML in the markdown header.

Btw, this is called the state, not just the data, because it can change. Something on the page could update the state,
and then the page would change, all in the browser.


```rust
    .view_with_state(post_page)
    .head_with_state(head)
```

Both `post_page` and `head` are functions I wrote, which control what the page will look like. They're separated
because it's often convenient to define the html `<head>` section separately from the rest of the page, and that's
how these two are split up. In Perseus, these take a state and return the HTML--or rather, a model of the HTML,
that Perseus can use in a bunch of different ways, from build or request time rendering on the server to client
side rendering whenever the state changes. It's just as easy to write as HTML, though, once you're used to a few
details of how Rust works. Here's the `post_page` for this very page.

```rust
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
```

Even this function isn't using anything Perseus specific! All that HTML logic is an awesome library called
[Sycamore](https://sycamore-rs.netlify.app), which is kind of like React, but for Rust. The `PostRx` data
structure is a reactive version of the `Post` data structure example I gave earlier, which is basically
the same except you use `.get()` to enable the reactivity.

## Finally

There're a number of other details, which you can read about on the Perseus website and in the Perseus book,
but the above covers most of the code I had to write for generating this very page. If you want to see _all_
the code for generating this page, the entire site's code is public at https://github.com/fugu13/site.

Since I only need build-time and client-side rendering for my site (in other words, I'm generating a static site that's nothing but
HTML and Javascript and WebAssembly), I can export it and put it on a static site host, using the command
`perseus deploy --export-static`.

Want more details on any specifics? Let me know!