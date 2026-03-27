# shirenode-commitlint

A fast git commit message linter that enforces [Conventional Commits](https://www.conventionalcommits.org/).

## Install

```
> bash
cargo install --path .

```

## Usage

```
> bash
# Lint a message directly

shirenode-commitlint --message "feat(cli): add commit linting"

# Lint from a file (useful as a git hook)
shirenode-commitlint --file .git/COMMIT_EDITMSG

```

## Git Hook

```
> bash
# Add as a commit-msg hook
echo '#!/bin/sh\nshirenode-commitlint --file "$1" || exit 1' > .git/hooks/commit-msg
chmod +x .git/hooks/commit-msg
```

## Rules
| Rule | Description |
|------|-------------|
| format | Subject matches `<type>(scope): <description>` |
| subject-length | Subject <= 72 characters |
| no-trailing-period | Subject doesn't end with `.` |
| body-separator | Blank line between subject and body |
