#!/bin/bash
#
# Copyright (c) 2025 TRUSTEDGE LABS LLC
# This source code is subject to the terms of the Mozilla Public License, v. 2.0.
# If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
#
# Project: trustedge — Privacy and trust at the edge.
#
# Pre-commit hook to ensure copyright headers are present
# Install: cp scripts/pre-commit.sh .git/hooks/pre-commit && chmod +x .git/hooks/pre-commit

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}● Checking copyright headers before commit...${NC}"

MISSING_FILES=()

# Function to check copyright header
check_copyright() {
    local file="$1"
    if ! head -10 "$file" | grep -q "Copyright (c) 2025 TRUSTEDGE LABS LLC"; then
        MISSING_FILES+=("$file")
        return 1
    fi
    return 0
}

# Check staged files only
MISSING_COUNT=0
while IFS= read -r file; do
    if [[ -f "$file" ]]; then
        case "$file" in
            *.rs|*.md|*.yml|*.yaml|*.toml)
                if ! check_copyright "$file"; then
                    echo -e "${RED}✖ Missing copyright header: $file${NC}"
                    MISSING_COUNT=$((MISSING_COUNT + 1))
                fi
                ;;
        esac
    fi
done < <(git diff --cached --name-only --diff-filter=ACM)

# If any files are missing headers, block the commit
if [ $MISSING_COUNT -gt 0 ]; then
    echo -e "${RED}✖ Commit blocked: $MISSING_COUNT files missing copyright headers${NC}"
    echo -e "${YELLOW}● Run 'make fix-copyright' to automatically add headers${NC}"
    exit 1
fi

echo -e "${GREEN}✔ All files have proper copyright headers${NC}"
exit 0
