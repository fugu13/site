#!/usr/bin/env bash
# Creates articles/draft.md: a placeholder draft post, timestamped now, ready to fill in.
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
target="$repo_root/articles/draft.md"

if [ -e "$target" ]; then
  echo "articles/draft.md already exists — rename or remove it before creating a new draft." >&2
  exit 1
fi

timestamp="$(date -Iseconds)"

cat > "$target" <<EOF
---
title: "TODO"
date: "$timestamp"
description: >
  TODO
draft: true
---

TODO

![TODO](/todo.jpg)

## TODO

TODO

\`\`\`python TODO one-line screen-reader description of this code sample
TODO
\`\`\`
EOF

echo "Created articles/draft.md"
