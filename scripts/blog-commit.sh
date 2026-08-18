#!/usr/bin/env bash
# Commits every change under articles/ and public/ to a new branch and pushes it,
# with a commit message auto-generated from which articles were added/updated and
# whether they're drafts. Run via `make blog`.
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

if git diff --quiet -- articles public \
  && git diff --cached --quiet -- articles public \
  && [ -z "$(git ls-files --others --exclude-standard -- articles public)" ]; then
  echo "No changes under articles/ or public/ to commit." >&2
  exit 1
fi

# Required so the new branch only ever carries blog content, never commits
# ridden along from whatever else main's checkout happened to be branched off.
current_branch="$(git rev-parse --abbrev-ref HEAD)"
if [ "$current_branch" != "main" ]; then
  echo "make blog must be run from main (currently on '$current_branch')." >&2
  exit 1
fi
git fetch origin main
if [ "$(git rev-parse HEAD)" != "$(git rev-parse origin/main)" ]; then
  echo "main is behind origin/main — pull before running make blog." >&2
  exit 1
fi

branch="blog-$(date +%Y%m%d-%H%M%S)"
git checkout -b "$branch"
git add -- articles public

# Prints "<title>\t<draft>" for a git content spec readable by `git show`
# (":path" for staged, "HEAD:path" for the prior committed version), reading
# the file's YAML front matter in one pass.
front_matter_fields() {
  git show "$1" 2>/dev/null | awk '
    NR == 1 && $0 == "---" { in_fm = 1; next }
    in_fm && $0 == "---" { exit }
    in_fm && $0 ~ /^title:/ {
      v = $0; sub(/^title:[ \t]*/, "", v); gsub(/^"|"$/, "", v); title = v
    }
    in_fm && $0 ~ /^draft:/ {
      v = $0; sub(/^draft:[ \t]*/, "", v); gsub(/^"|"$/, "", v); draft = v
    }
    END { printf "%s\t%s\n", title, draft }
  '
}

entries=()
asset_paths=()
while IFS=$'\t' read -r status path; do
  asset_paths+=("$path")
  case "$path" in
  articles/*.md) ;;
  *) continue ;;
  esac
  slug="$(basename "$path" .md)"
  spec=":$path"
  [ "$status" = "D" ] && spec="HEAD:$path"
  IFS=$'\t' read -r title draft < <(front_matter_fields "$spec")
  title="${title:-$slug}"
  case "$status" in
  A)
    if [ "$draft" = "true" ]; then
      entries+=("added draft of \"$title\"")
    else
      entries+=("added \"$title\"")
    fi
    ;;
  M)
    if [ "$draft" = "true" ]; then
      entries+=("updated draft of \"$title\"")
    else
      entries+=("published update of \"$title\"")
    fi
    ;;
  D)
    entries+=("removed \"$title\"")
    ;;
  esac
done < <(git diff --cached --name-status --no-renames -- articles public)

if [ "${#entries[@]}" -eq 0 ]; then
  subject="Update blog assets ($(
    IFS=,
    echo "${asset_paths[*]}"
  ))"
else
  printf -v joined '%s, ' "${entries[@]}"
  subject="${joined%, }"
  first="$(printf '%s' "${subject:0:1}" | tr '[:lower:]' '[:upper:]')"
  subject="$first${subject:1}"
fi

git commit -m "$subject"
git push -u origin "$branch"

echo "Pushed $branch: $subject"
