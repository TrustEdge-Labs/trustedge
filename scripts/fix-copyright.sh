#!/bin/bash
#
# Copyright (c) 2025 John Turner
# This source code is subject to the terms of the Mozilla Public License, v. 2.0.
# If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
#
# Project: trustedge — Privacy and trust at the edge.
#
# Script to automatically add copyright headers to all source files

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}🔧 Adding copyright headers to all source files...${NC}"

FIXED=0

# Function to add copyright header if missing
add_copyright_header() {
    local file="$1"
    local header_type="$2"
    
    if ! head -10 "$file" | grep -q "Copyright (c) 2025 John Turner"; then
        echo -e "${YELLOW}📝 Adding copyright header to: $file${NC}"
        
        case "$header_type" in
            "rust")
                HEADER=$'//\n// Copyright (c) 2025 John Turner\n// This source code is subject to the terms of the Mozilla Public License, v. 2.0.\n// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/\n//\n// Project: trustedge — Privacy and trust at the edge.\n//\n\n'
                ;;
            "markdown")
                HEADER=$'<!--\nCopyright (c) 2025 John Turner\nMPL-2.0: https://mozilla.org/MPL/2.0/\nProject: trustedge — Privacy and trust at the edge.\nGitHub: https://github.com/johnzilla/trustedge\n-->\n\n'
                ;;
            "yaml")
                HEADER=$'# Copyright (c) 2025 John Turner\n# MPL-2.0: https://mozilla.org/MPL/2.0/\n# Project: trustedge — Privacy and trust at the edge.\n\n'
                ;;
        esac
        
        # Create temporary file with header + original content
        echo -e "$HEADER" > "${file}.tmp"
        cat "$file" >> "${file}.tmp"
        mv "${file}.tmp" "$file"
        
        FIXED=$((FIXED + 1))
    fi
}

# Process Rust files
echo -e "${YELLOW}📁 Processing Rust files...${NC}"
find . -name "*.rs" -not -path "./target/*" -not -path "./.git/*" | while read -r file; do
    add_copyright_header "$file" "rust"
done

# Process Markdown files (excluding auto-generated ones)
echo -e "${YELLOW}📁 Processing Markdown files...${NC}"
find . -name "*.md" -not -path "./.git/*" -not -name "CHANGELOG.md" | while read -r file; do
    add_copyright_header "$file" "markdown"
done

# Process YAML files
echo -e "${YELLOW}📁 Processing YAML/YML files...${NC}"
find . -name "*.yml" -o -name "*.yaml" -not -path "./.git/*" | while read -r file; do
    add_copyright_header "$file" "yaml"
done

# Process TOML files  
echo -e "${YELLOW}📁 Processing TOML files...${NC}"
find . -name "*.toml" -not -path "./.git/*" | while read -r file; do
    add_copyright_header "$file" "yaml"
done

if [ $FIXED -gt 0 ]; then
    echo -e "${GREEN}✅ Added copyright headers to $FIXED files${NC}"
else
    echo -e "${GREEN}✅ All files already have copyright headers${NC}"
fi

echo -e "${GREEN}🎉 Copyright header check complete!${NC}"
