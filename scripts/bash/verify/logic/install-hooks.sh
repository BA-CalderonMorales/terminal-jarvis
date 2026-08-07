#!/usr/bin/env sh
set -eu

# Installs the pre-commit hook into .git/hooks/. Safe to run repeatedly:
# the hook replaces itself with the current scripts/bash/verify checkout.

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../../../.." && pwd)
hook="$root/.git/hooks/pre-commit"
gate="$here/pre-commit.sh"

mkdir -p "$(dirname "$hook")"
cat > "$hook" <<EOF
#!/usr/bin/env sh
set -eu
exec "$gate"
EOF
chmod +x "$hook"
echo "pre-commit hook installed: $hook"
