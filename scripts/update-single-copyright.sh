#!/bin/bash
#
# Copyright (c) 2025 TRUSTEDGE LABS LLC
# This source code is subject to the terms of the Mozilla Public License, v. 2.0.
# If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
#
# Project: trustedge — Privacy and trust at the edge.
#
# Script to update copyright in individual files with user approval

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

if [ $# -ne 1 ]; then
    echo -e "${RED}❌ Usage: $0 <file_path>${NC}"
    echo -e "${YELLOW}Example: $0 ./README.md${NC}"
    exit 1
fi

FILE="$1"

if [ ! -f "$FILE" ]; then
    echo -e "${RED}❌ File not found: $FILE${NC}"
    exit 1
fi

echo -e "${BLUE}🔍 Analyzing file: $FILE${NC}"

# Check if file contains John Turner copyright
if ! grep -q "Copyright.*John Turner" "$FILE"; then
    echo -e "${GREEN}✅ No 'John Turner' copyright found in $FILE${NC}"
    exit 0
fi

# Show current copyright lines
echo -e "${YELLOW}📄 Current copyright lines in file:${NC}"
grep -n "Copyright.*John Turner" "$FILE" | head -5

echo ""
echo -e "${BLUE}📝 Proposed changes:${NC}"
echo -e "  Replace: ${RED}John Turner${NC}"
echo -e "  With:    ${GREEN}TRUSTEDGE LABS LLC${NC}"

echo ""
read -p "$(echo -e "${YELLOW}❓ Do you approve this change? (y/N): ${NC}")" -n 1 -r
echo

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}⏸️  Change cancelled by user${NC}"
    exit 0
fi

# Create backup
cp "$FILE" "${FILE}.bak"
echo -e "${BLUE}💾 Created backup: ${FILE}.bak${NC}"

# Perform the replacement
sed -i 's/Copyright (c) 2025 John Turner/Copyright (c) 2025 TRUSTEDGE LABS LLC/g' "$FILE"

# Also replace any other variations
sed -i 's/Copyright ©\? 2025 John Turner/Copyright © 2025 TRUSTEDGE LABS LLC/g' "$FILE"

# Verify the change
if grep -q "Copyright.*TRUSTEDGE LABS LLC" "$FILE"; then
    echo -e "${GREEN}✅ Successfully updated copyright in $FILE${NC}"
    
    # Show the updated lines
    echo -e "${BLUE}📄 Updated copyright lines:${NC}"
    grep -n "Copyright.*TRUSTEDGE LABS LLC" "$FILE" | head -5
    
    # Remove backup on success
    rm "${FILE}.bak"
    echo -e "${GREEN}🗑️  Removed backup file${NC}"
else
    echo -e "${RED}❌ Failed to update copyright in $FILE${NC}"
    echo -e "${YELLOW}🔄 Restoring from backup...${NC}"
    mv "${FILE}.bak" "$FILE"
    exit 1
fi

# Mark as completed in tracking file if it exists
TRACKING_FILE="copyright-update-list.txt"
if [ -f "$TRACKING_FILE" ]; then
    # Add a completion marker
    if ! grep -q "# COMPLETED: $FILE" "$TRACKING_FILE"; then
        echo "# COMPLETED: $FILE" >> "$TRACKING_FILE"
        echo -e "${BLUE}📋 Marked as completed in tracking file${NC}"
    fi
fi

echo -e "${GREEN}🎉 Copyright update complete for $FILE${NC}"
