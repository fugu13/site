---
title: Why is WebAssembly interesting?
date: '2023-11-15T13:20:00-08:00'
description: >
  What are the strategically interesting things that WebAssembly makes feasible?
  Oh, and what does it have to do with Python?
---

WebAssembly is showing up lots of places these days, but most developers haven't interacted
with it much. And even if they do, they might not notice. For example, this entire website
is powered by WebAssembly, but I didn't write any WebAssembly, and I didn't think about
WebAssembly much while I was writing the website.

But without WebAssembly, I wouldn't have
been able to write this website the way I did. What's more, WebAssembly is the first code
that runs for vast numbers of the largest sites on the web, powering critical pieces of
their CDN (Content Delivery Network) and other edge capabilities, using platforms like
Fastly and Cloudflare.

Why is it such a big deal?

![an old wooden waterwheel](.perseus/static/waterwheel.jpg)


### A brief "what"

WebAssembly is a small, simple virtual machine with an easily parsed binary format
designed to
run in browsers, fast. The virtual machine uses a simple memory layout designed so
that many memory safety vulnerabilities just are not possible, and others are much
more difficult.

There are no built-in capabilities to interact outside the virtual machine, a powerful
security boundary, and the WebAssembly System Interface (WASI) allowing that outside
communication uses capability-based security, which has a key property: platforms
using WebAssembly only provide _positive_ permission to interact with the outside
environment, so unintended access is much less likely.

### Why not Javascript?

Javascript is vastly more complicated! And while a predecessor to WebAssembly, asm.js,
was a subset of Javascript with similar properties, WebAssembly has smaller files that load
faster with clearer semantics, and is much easier to implement separately from Javascript.

Additionally, the WebAssembly road map has features that couldn't be easily implemented in
Javascript.

### Why yes WebAssembly?

WebAssembly has three strategically critical properties.

1. Compilation targeting the virtual machine is easy to implement.
2. The virtual machine starts and runs very, very quickly.
3. A very high level of security is built in, even if the code being compiled to
   WebAssembly was not written with security in mind.

The browser, which requires a very high level of security, has always been a principal
target platform for WebAssembly, and all major and most minor browsers have adopted it.

This, along with the other three properties, can be reframed into another strategically
critical property:

4. Code written in other languages for other purposes can be compiled to WebAssembly, and
   codebases in a single language can power both the backend and other places, like
   the browser frontend for web applications.

For example, imagine you've defined a data structure inside your backend application to
represent the responses to API requests. If you're using WebAssembly, you can use that
same data structure, either imported in a library or in a shared codebase, in your
frontend code, written in the same language. It'll work perfectly, because it's the same
data structure the backend serialized to send to the frontend!

Gary Bernhardt wrote an
excellent blog post explaining [how sharing types with Typescript in the backend and
browser client eliminated
bugs](https://www.executeprogram.com/blog/porting-to-typescript-solved-our-api-woes)
a few years ago, and with WebAssembly, that capability becomes possible for any language!

That's what's happening with this website--I wrote it in Rust, in a framework named Perseus
that can run a pregeneration phase, a dynamic backend, and a browser frontend,
all with the same code. Read more about it at
[Writing a Static Site with Rust and Perseus](post/static-site-rust-perseus/).

And the fast, secure virtual machine makes it an excellent platform for any user-provided
code, not just in the browser. For example, code running in [Lucet, an open
source WebAssembly runtime](https://bytecodealliance.github.io/lucet/Overview.html) can
load a user-provided WebAssembly module from a cold start in 50 microseconds, using only a
few kilobytes of memory, ready to serve a new request. Edge platforms like Fastly (who
created Lucet) and Cloudflare have very tight performance limits, so they've adopted
WebAssembly rapidly, but I expect to see WebAssembly popping up in other contexts where
secure, fast starting user-provided code is useful, from streaming data platforms to video
game scripting.

### How does that work with Python?

Python is already widely used for web application backends and for data science and
data engineering. I think there are two big ways we'll especially see Python used to write
WebAssembly.

1. Just keep writing Python, even in your existing FastAPI/Django/Favorite Web Framework Here 
   only now it runs in the browser too! Modern Python with strong typing is an amazing fit for this.
   
   Running changing business logic in the browser and backend both from a single implementation
   eliminates a big, big source of bugs, and the same devs are able to wear both hats.
   Both are very persuasive strategically.
2. Data teams can write powerful, comfortable Python, but then run it as WebAssembly in
   high performance pipelines. If it doesn't follow the security or performance boundaries
   set for the pipeline, it'll be automatically and safely stopped. Offering both
   flexibility and safety makes iteration faster, which is very persuasive strategically.

I don't think there's any data platform out there providing built-in WebAssembly
capabilities yet, so consider the second one a prediction. The amazing [Pyodide project,
for running Python as WebAssembly](https://pyodide.org/) has already ported many of the
critical packages that use C code, such as numpy, pandas, and scipy.

### Summing up

Ultimately, WebAssembly will be one of those things that is everywhere, but you don't
think about very much most of the time. Instead, you'll mostly be excited when you're
able to write fast, reactive browser code in your preferred language (no React!), or
when a platform where you upload code to announces it now supports dozens of languages,
including your preferred one, and what's more startup time and security have both
improved dramatically.