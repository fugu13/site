# Code Block Scrolling

## Reaching a code block by keyboard

A reader moving through a post with the Tab key reaches each code sample as a stop in the tab order, even one short enough that it doesn't need scrolling. When a code sample receives focus, a focus outline appears around the whole sample.

## Hearing what a code block is

Each code sample announces a short description of its own — for example, "The complete test file with all four property tests" — when a screen reader focuses it or lists the page's landmarks. Because every description is distinct, a reader scanning the landmark list of a code-heavy post can tell the samples apart and jump straight to the one they want, instead of hearing an identical "code sample" many times over. A sample without a written description falls back to the generic "Code sample".

## Scrolling a wide code block into view

Some code samples are wider than the page and would otherwise overflow off the right edge, out of view. Once a wide sample has keyboard focus, the reader can use the arrow keys — or two-finger/trackpad scrolling, for mouse and touch users — to scroll its content horizontally, bringing the rest of each line into view without scrolling the whole page. The rest of the post's layout is unaffected; only the code sample's own content shifts.
