#!/bin/bash
#
# Copyright (c) 2025 TRUSTEDGE LABS LLC
# This source code is subject to the terms of the Mozilla Public License, v. 2.0.
# If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
#
# Project: trustedge — Privacy and trust at the edge.
#
# Script to find all "trustedge-audio" references that need updating to "trustedge-core"

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔍 Finding all 'trustedge-audio' references to update to 'trustedge-core'...${NC}"

# Create a log file for tracking
LOG_FILE="trustedge-rename-analysis.txt"
echo "# TrustEdge Audio -> Core Rename Analysis" > "$LOG_FILE"
echo "# Generated on $(date)" >> "$LOG_FILE"
echo "# Files needing 'trustedge-audio' -> 'trustedge-core' updates" >> "$LOG_FILE"
echo "" >> "$LOG_FILE"

echo -e "${CYAN}📊 Analysis Categories:${NC}"
echo "  1. Directory name: trustedge-audio/ -> trustedge-core/"
echo "  2. Cargo.toml package names and dependencies"  
echo "  3. Rust import statements (use trustedge_audio)"
echo "  4. Documentation references"
echo "  5. Script paths and commands"
echo "  6. CI/CD pipeline references"
echo ""

# Function to search and log matches
search_and_log() {
    local pattern="$1"
    local description="$2"
    local files_found=0
    
    echo -e "${YELLOW}🔍 Searching for: $description${NC}"
    echo "## $description" >> "$LOG_FILE"
    
    # Search for the pattern, excluding .git and target directories
    while IFS= read -r match; do
        if [ -n "$match" ]; then
            echo -e "  ${CYAN}📄 $match${NC}"
            echo "$match" >> "$LOG_FILE"
            files_found=$((files_found + 1))
        fi
    done < <(grep -r "$pattern" . --exclude-dir=.git --exclude-dir=target --exclude="$LOG_FILE" 2>/dev/null || true)
    
    echo "# Found $files_found matches" >> "$LOG_FILE"
    echo "" >> "$LOG_FILE"
    echo -e "${GREEN}  ✅ Found $files_found matches${NC}"
    echo ""
    
    return $files_found
}

total_matches=0

# 1. Search for "trustedge-audio" (with hyphen)
search_and_log "trustedge-audio" "Package name with hyphen (trustedge-audio)"
total_matches=$((total_matches + $?))

# 2. Search for "trustedge_audio" (with underscore) 
search_and_log "trustedge_audio" "Rust module name with underscore (trustedge_audio)"
total_matches=$((total_matches + $?))

# 3. Search for directory references
search_and_log "trustedge-audio/" "Directory path references"
total_matches=$((total_matches + $?))

# 4. Search for specific patterns that might be missed
search_and_log "audio.*trustedge" "Reverse pattern (audio + trustedge)"
total_matches=$((total_matches + $?))

# 5. Search for cargo references
search_and_log "name.*trustedge.*audio" "Cargo package name patterns"
total_matches=$((total_matches + $?))

echo "## Summary" >> "$LOG_FILE"
echo "# Total patterns found: $total_matches" >> "$LOG_FILE"
echo "# Status: ANALYSIS COMPLETE - Ready for systematic updates" >> "$LOG_FILE"
echo "" >> "$LOG_FILE"

echo -e "${BLUE}📋 Rename Plan:${NC}"
echo -e "${BLUE}==============${NC}"
echo -e "${YELLOW}Phase 1: Prepare${NC}"
echo "  • Create backup of current state"
echo "  • Update Cargo.toml files"
echo ""
echo -e "${YELLOW}Phase 2: Directory Rename${NC}" 
echo "  • Rename trustedge-audio/ -> trustedge-core/"
echo "  • Update all path references"
echo ""
echo -e "${YELLOW}Phase 3: Code Updates${NC}"
echo "  • Update Rust import statements"
echo "  • Update library names and crate references"
echo ""
echo -e "${YELLOW}Phase 4: Documentation${NC}"
echo "  • Update README and documentation files"
echo "  • Update examples and demos"
echo ""
echo -e "${YELLOW}Phase 5: Infrastructure${NC}"
echo "  • Update CI/CD scripts"
echo "  • Update build and test scripts"
echo ""

echo -e "${GREEN}✅ Analysis complete!${NC}"
echo -e "${YELLOW}📊 Total matches found: $total_matches${NC}"
echo -e "${BLUE}📝 Full analysis saved to: $LOG_FILE${NC}"
echo ""
echo -e "${CYAN}💡 Next: Review the analysis file and proceed with Phase 1${NC}"
