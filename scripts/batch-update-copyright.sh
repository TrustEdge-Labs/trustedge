#!/bin/bash
#
# Copyright (c) 2025 TRUSTEDGE LABS LLC
# This source code is subject to the terms of the Mozilla Public License, v. 2.0.
# If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
#
# Project: trustedge — Privacy and trust at the edge.
#
# Script to update copyright in all files with single approval

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

TRACKING_FILE="copyright-update-list.txt"

if [ ! -f "$TRACKING_FILE" ]; then
    echo -e "${RED}❌ Tracking file not found: $TRACKING_FILE${NC}"
    echo -e "${YELLOW}💡 Run ./scripts/find-copyright-files.sh first${NC}"
    exit 1
fi

# Get list of files that haven't been processed yet
FILES_TO_PROCESS=()
while IFS= read -r line; do
    # Skip comments and empty lines
    if [[ "$line" =~ ^#.*$ ]] || [[ -z "$line" ]]; then
        continue
    fi
    
    # Skip files already marked as completed
    if grep -q "# COMPLETED: $line" "$TRACKING_FILE"; then
        continue
    fi
    
    FILES_TO_PROCESS+=("$line")
done < "$TRACKING_FILE"

TOTAL_FILES=${#FILES_TO_PROCESS[@]}

if [ $TOTAL_FILES -eq 0 ]; then
    echo -e "${GREEN}🎉 All files have already been processed!${NC}"
    exit 0
fi

echo -e "${BLUE}📋 Copyright Update Batch Processor${NC}"
echo -e "${BLUE}===================================${NC}"
echo ""
echo -e "${YELLOW}📊 Files to process: $TOTAL_FILES${NC}"
echo ""
echo -e "${CYAN}📄 Preview of files to be updated:${NC}"
for i in "${!FILES_TO_PROCESS[@]}"; do
    if [ $i -lt 10 ]; then
        echo -e "  $(($i + 1)). ${FILES_TO_PROCESS[$i]}"
    elif [ $i -eq 10 ]; then
        echo -e "  ... and $(($TOTAL_FILES - 10)) more files"
        break
    fi
done

echo ""
echo -e "${BLUE}📝 This will replace:${NC}"
echo -e "  ${RED}Copyright (c) 2025 John Turner${NC}"
echo -e "  ${GREEN}Copyright (c) 2025 TRUSTEDGE LABS LLC${NC}"

echo ""
read -p "$(echo -e "${YELLOW}❓ Continue with batch update of all $TOTAL_FILES files? (y/N): ${NC}")" -n 1 -r
echo

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}⏸️  Batch update cancelled by user${NC}"
    exit 0
fi

echo ""
echo -e "${BLUE}🚀 Starting batch copyright update...${NC}"
echo ""

PROCESSED=0
SUCCEEDED=0
FAILED=0

for file in "${FILES_TO_PROCESS[@]}"; do
    PROCESSED=$((PROCESSED + 1))
    
    echo -e "${CYAN}[$PROCESSED/$TOTAL_FILES]${NC} Processing: ${BLUE}$file${NC}"
    
    if [ ! -f "$file" ]; then
        echo -e "  ${RED}❌ File not found${NC}"
        FAILED=$((FAILED + 1))
        continue
    fi
    
    # Check if file contains John Turner copyright
    if ! grep -q "Copyright.*John Turner" "$file"; then
        echo -e "  ${YELLOW}⏭️  No 'John Turner' copyright found${NC}"
        # Mark as completed anyway
        echo "# COMPLETED: $file" >> "$TRACKING_FILE"
        continue
    fi
    
    # Create backup
    cp "$file" "${file}.bak.$$"
    
    # Perform the replacement
    if sed -i 's/Copyright (c) 2025 John Turner/Copyright (c) 2025 TRUSTEDGE LABS LLC/g' "$file" && \
       sed -i 's/Copyright ©\? 2025 John Turner/Copyright © 2025 TRUSTEDGE LABS LLC/g' "$file"; then
        
        # Verify the change
        if grep -q "Copyright.*TRUSTEDGE LABS LLC" "$file"; then
            echo -e "  ${GREEN}✅ Successfully updated${NC}"
            rm "${file}.bak.$$"  # Remove backup on success
            echo "# COMPLETED: $file" >> "$TRACKING_FILE"
            SUCCEEDED=$((SUCCEEDED + 1))
        else
            echo -e "  ${RED}❌ Verification failed${NC}"
            mv "${file}.bak.$$" "$file"  # Restore from backup
            FAILED=$((FAILED + 1))
        fi
    else
        echo -e "  ${RED}❌ Update failed${NC}"
        mv "${file}.bak.$$" "$file"  # Restore from backup
        FAILED=$((FAILED + 1))
    fi
done

echo ""
echo -e "${BLUE}📊 Batch Update Summary${NC}"
echo -e "${BLUE}======================${NC}"
echo -e "${GREEN}✅ Successfully updated: $SUCCEEDED files${NC}"
if [ $FAILED -gt 0 ]; then
    echo -e "${RED}❌ Failed: $FAILED files${NC}"
fi
echo -e "${CYAN}📁 Total processed: $PROCESSED files${NC}"

# Update tracking file with completion summary
echo "" >> "$TRACKING_FILE"
echo "# BATCH UPDATE COMPLETED ON $(date)" >> "$TRACKING_FILE"
echo "# Successfully updated: $SUCCEEDED files" >> "$TRACKING_FILE"
echo "# Failed: $FAILED files" >> "$TRACKING_FILE"

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}🎉 All files updated successfully!${NC}"
else
    echo -e "${YELLOW}⚠️  Some files failed to update. Check the output above for details.${NC}"
fi

echo -e "${BLUE}📋 Progress saved to: $TRACKING_FILE${NC}"
