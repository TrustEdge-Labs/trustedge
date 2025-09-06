#!/bin/bash
#
# Copyright (c) 2025 TRUSTEDGE LABS LLC
# This source code is subject to the terms of the Mozilla Public License, v. 2.0.
# If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
#
# Project: trustedge — Privacy and trust at the edge.
#
# Script to find all files that need copyright updates from "John Turner" to "TRUSTEDGE LABS LLC"

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔍 Finding all files with 'John Turner' copyright that need updating to 'TRUSTEDGE LABS LLC'...${NC}"

# Create a log file for tracking
LOG_FILE="copyright-update-list.txt"
echo "# Copyright Update Tracking List" > "$LOG_FILE"
echo "# Generated on $(date)" >> "$LOG_FILE"
echo "# Files needing copyright update from 'John Turner' to 'TRUSTEDGE LABS LLC'" >> "$LOG_FILE"
echo "" >> "$LOG_FILE"

TOTAL_FILES=0

# Function to check and log files with John Turner copyright
check_and_log_file() {
    local file="$1"
    if [ -f "$file" ] && grep -l "Copyright.*John Turner" "$file" >/dev/null 2>&1; then
        echo -e "${YELLOW}📄 Found: $file${NC}"
        echo "$file" >> "$LOG_FILE"
        TOTAL_FILES=$((TOTAL_FILES + 1))
    fi
}

echo -e "${BLUE}Searching in root directory...${NC}"
find . -maxdepth 1 -type f \( -name "*.md" -o -name "*.yml" -o -name "*.yaml" -o -name "Makefile" -o -name "*.sh" \) -not -path "./.git/*" | while read -r file; do
    check_and_log_file "$file"
done

echo -e "${BLUE}Searching in .github directory...${NC}"
find .github -type f \( -name "*.md" -o -name "*.yml" -o -name "*.yaml" \) 2>/dev/null | while read -r file; do
    check_and_log_file "$file"
done

echo -e "${BLUE}Searching in docs directory...${NC}"
find docs -type f -name "*.md" 2>/dev/null | while read -r file; do
    check_and_log_file "$file"
done

echo -e "${BLUE}Searching in examples directory...${NC}"
find examples -type f \( -name "*.rs" -o -name "*.md" \) 2>/dev/null | while read -r file; do
    check_and_log_file "$file"
done

echo -e "${BLUE}Searching in scripts directory...${NC}"
find scripts -type f \( -name "*.sh" -o -name "*.md" \) 2>/dev/null | while read -r file; do
    check_and_log_file "$file"
done

echo -e "${BLUE}Searching in trustedge-audio directory...${NC}"
find trustedge-audio -type f \( -name "*.rs" -o -name "*.md" -o -name "*.toml" -o -name "*.sh" \) -not -path "*/target/*" 2>/dev/null | while read -r file; do
    check_and_log_file "$file"
done

# Count total files found
TOTAL_COUNT=$(grep -v "^#" "$LOG_FILE" | grep -v "^$" | wc -l)

echo "" >> "$LOG_FILE"
echo "# Total files found: $TOTAL_COUNT" >> "$LOG_FILE"
echo "# Status: PENDING - Ready for manual review and approval" >> "$LOG_FILE"

echo -e "${GREEN}✅ Search complete!${NC}"
echo -e "${YELLOW}📊 Total files found: $TOTAL_COUNT${NC}"
echo -e "${BLUE}📝 Results saved to: $LOG_FILE${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo -e "  1. Review the list in $LOG_FILE"
echo -e "  2. Use the individual file update process for each file"
echo -e "  3. Mark files as completed in the log file"
