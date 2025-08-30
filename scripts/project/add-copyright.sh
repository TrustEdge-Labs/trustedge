#!/bin/bash

# TrustEdge Copyright Header Updater
# Adds consistent copyright headers to all source files
#
# Copyright (c) 2025 John Turner
# MPL-2.0: https://mozilla.org/MPL/2.0/
# Project: trustedge — Privacy and trust at the edge.
# GitHub: https://github.com/johnzilla/trustedge

set -e

echo "📄 TrustEdge Copyright Header Updater"
echo "===================================="

# Color codes for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Copyright information
YEAR="2025"
AUTHOR="John Turner"
LICENSE="MPL-2.0"
LICENSE_URL="https://mozilla.org/MPL/2.0/"
PROJECT="trustedge — Privacy and trust at the edge."
GITHUB_URL="https://github.com/johnzilla/trustedge"

# Function to check if file already has copyright header
has_copyright() {
    local file="$1"
    head -n 15 "$file" | grep -i "copyright.*john turner" >/dev/null 2>&1
}

# Function to add copyright header to Rust files
add_rust_header() {
    local file="$1"
    local temp_file=$(mktemp)
    
    # Check if file starts with #![forbid(unsafe_code)]
    local has_forbid=""
    if head -n 1 "$file" | grep -q "#!\[forbid(unsafe_code)\]"; then
        has_forbid="#![forbid(unsafe_code)]\n\n"
    fi
    
    cat > "$temp_file" << EOF
${has_forbid}//
// Copyright (c) $YEAR $AUTHOR
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at $LICENSE_URL
//
// Project: $PROJECT
// GitHub: $GITHUB_URL
//

EOF
    
    # Skip existing copyright and #![forbid] lines, then append rest of file
    if [ -n "$has_forbid" ]; then
        tail -n +2 "$file" >> "$temp_file"
    else
        cat "$file" >> "$temp_file"
    fi
    
    mv "$temp_file" "$file"
}

# Function to add copyright header to shell scripts
add_shell_header() {
    local file="$1"
    local temp_file=$(mktemp)
    
    # Get the shebang line
    local shebang=$(head -n 1 "$file")
    
    cat > "$temp_file" << EOF
$shebang

#
# Copyright (c) $YEAR $AUTHOR
# $LICENSE: $LICENSE_URL
# Project: $PROJECT
# GitHub: $GITHUB_URL
#

EOF
    
    # Skip shebang line and append rest of file
    tail -n +2 "$file" >> "$temp_file"
    mv "$temp_file" "$file"
}

# Function to add copyright header to markdown files
add_markdown_header() {
    local file="$1"
    local temp_file=$(mktemp)
    
    cat > "$temp_file" << EOF
<!--
Copyright (c) $YEAR $AUTHOR
$LICENSE: $LICENSE_URL
Project: $PROJECT
GitHub: $GITHUB_URL
-->

EOF
    
    cat "$file" >> "$temp_file"
    mv "$temp_file" "$file"
}

# Function to update file with copyright header
update_file() {
    local file="$1"
    local type="$2"
    
    if has_copyright "$file"; then
        echo -e "${YELLOW}⚠️  Skipping $file (already has copyright)${NC}"
        return
    fi
    
    echo -e "${BLUE}📝 Adding copyright to $file${NC}"
    
    case "$type" in
        "rust")
            add_rust_header "$file"
            ;;
        "shell")
            add_shell_header "$file"
            ;;
        "markdown")
            add_markdown_header "$file"
            ;;
    esac
    
    echo -e "${GREEN}✅ Updated $file${NC}"
}

# Process all Rust files
echo ""
echo "🦀 Processing Rust files..."
echo "-------------------------"
find . -name "*.rs" -type f | while read -r file; do
    update_file "$file" "rust"
done

# Process all shell scripts
echo ""
echo "🐚 Processing shell scripts..."
echo "-----------------------------"
find . -name "*.sh" -type f | while read -r file; do
    update_file "$file" "shell"
done

# Process all markdown files
echo ""
echo "📚 Processing markdown files..."
echo "------------------------------"
find . -name "*.md" -type f | while read -r file; do
    update_file "$file" "markdown"
done

echo ""
echo -e "${GREEN}✨ Copyright header update complete!${NC}"
echo ""
echo "📋 Summary:"
echo "- All .rs files: Rust-style copyright headers"
echo "- All .sh files: Shell-style copyright headers"  
echo "- All .md files: HTML comment copyright headers"
echo ""
echo "🔍 To verify headers were added correctly:"
echo "  grep -r \"Copyright.*John Turner\" . --include=\"*.rs\" --include=\"*.sh\" --include=\"*.md\""
