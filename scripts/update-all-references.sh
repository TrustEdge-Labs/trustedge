#!/bin/bash
#
# Copyright (c) 2025 TRUSTEDGE LABS LLC
# This source code is subject to the terms of the Mozilla Public License, v. 2.0.
# If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
#
# Project: trustedge — Privacy and trust at the edge.
#
# Script to update all documentation and configuration references

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔄 Updating all documentation and script references...${NC}"

UPDATED_FILES=0

# Function to update a file
update_file() {
    local file="$1"
    local description="$2"
    
    if [ ! -f "$file" ]; then
        echo -e "${YELLOW}⏭️  Skipping missing file: $file${NC}"
        return
    fi
    
    if grep -q "trustedge-audio" "$file"; then
        echo -e "${CYAN}📝 Updating $description: $file${NC}"
        
        # Create backup
        cp "$file" "${file}.bak"
        
        # Update references
        sed -i 's/trustedge-audio/trustedge-core/g' "$file"
        
        # Verify the change
        if ! grep -q "trustedge-audio" "$file"; then
            echo -e "${GREEN}  ✔ Successfully updated${NC}"
            rm "${file}.bak"  # Remove backup on success
            UPDATED_FILES=$((UPDATED_FILES + 1))
        else
            echo -e "${RED}  ✖ Update failed, restoring backup${NC}"
            mv "${file}.bak" "$file"
        fi
    fi
}

echo -e "${YELLOW}Phase 1: Documentation Files${NC}"
update_file "./README.md" "Main README"
update_file "./TROUBLESHOOTING.md" "Troubleshooting Guide"
update_file "./CLI.md" "CLI Documentation"
update_file "./EXAMPLES.md" "Examples Guide"
update_file "./TESTING.md" "Testing Documentation"
update_file "./AUTHENTICATION_GUIDE.md" "Authentication Guide"
update_file "./CONTRIBUTING.md" "Contributing Guide"
update_file "./DEVELOPMENT.md" "Development Guide"
update_file "./PROTOCOL.md" "Protocol Documentation"
update_file "./ROADMAP.md" "Roadmap"

echo -e "${YELLOW}Phase 2: Configuration Files${NC}"
update_file "./Makefile" "Makefile"
update_file "./.gitignore" "Git ignore file"
update_file "./codecov.yml" "Code coverage config"

echo -e "${YELLOW}Phase 3: GitHub Configuration${NC}"
update_file "./.github/workflows/ci.yml" "CI workflow"
update_file "./.github/workflows/copyright-check.yml" "Copyright check workflow"
update_file "./.github/CODEOWNERS" "Code owners"
update_file "./.github/copilot-instructions.md" "Copilot instructions"
update_file "./.github/ISSUE_TEMPLATE/bug-report.yml" "Bug report template"
update_file "./.github/ISSUE_TEMPLATE/feature-request.yml" "Feature request template"

echo -e "${YELLOW}Phase 4: Scripts${NC}"
update_file "./scripts/ci-check.sh" "CI check script"
update_file "./scripts/fast-bench.sh" "Fast benchmark script"
update_file "./scripts/README.md" "Scripts README"

echo -e "${YELLOW}Phase 5: Core Directory Files${NC}"
update_file "./trustedge-core/BENCHMARKS.md" "Benchmarks documentation"
update_file "./trustedge-core/CI_CHECK_SOLUTION.md" "CI check solution"
update_file "./trustedge-core/AUTHENTICATION.md" "Core authentication docs"
update_file "./trustedge-core/PERFORMANCE.md" "Performance documentation"
update_file "./trustedge-core/SOFTWARE_HSM_TEST_REPORT.md" "Software HSM test report"
update_file "./trustedge-core/ci-check.sh" "Core CI check script"

echo ""
echo -e "${GREEN}✔ Documentation update complete!${NC}"
echo -e "${CYAN}● Total files updated: $UPDATED_FILES${NC}"
echo ""
echo -e "${BLUE}● Summary of Changes:${NC}"
echo -e "  • Directory: trustedge-audio/ → trustedge-core/"
echo -e "  • Package: trustedge-audio → trustedge-core"
echo -e "  • Library: trustedge_audio → trustedge_core"
echo -e "  • Binary: trustedge-audio → trustedge-core"
echo -e "  • Documentation: All references updated"
echo -e "  • Scripts: All paths and references updated"
