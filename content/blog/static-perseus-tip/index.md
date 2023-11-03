---
title: Quick Tip for Static Perseus Websites
date: '2023-11-02T12:50:00-08:00'
---

It took me a bit to put all the pieces together, so I'm posting them here in one place.

If you're making a Perseus site designed for static export, instead of

```rust
#[perseus::main(perseus_axum::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        ...
}
```

use `main_export`, so it looks like

```rust
#[perseus::main_export]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        ...
}
```

But when you do that, you stop using `perseus serve` and instead use `perseus export -s`
(the `-s` stands for `--serve`), and you can include `-w` for watching too, so
`perseus export -s -w` or `perseus export --serve --watch`.

You'll export your static files like you did before, which probably won't be `perseus export`
despite the tempting `--release` option. Instead, you want `perseus deploy --export-static` (or `-e`),
which puts everything in the directory `pkg/`.

And that seems to be the happy combination for making static sites with Perseus!