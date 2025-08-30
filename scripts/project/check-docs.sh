#!/bin/bash

# TrustEdge Documentation Status Checker
# Verifies that all documentation is current and consistent
#
# Usage: ./scripts/project/check-docs.sh
# Run from project root directory

set -e

echo "📚 TrustEdge Documentation Status Check"
echo "======================================"

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to check if file exists and is recent
check_file_status() {
    local file=$1
    local description=$2
    
    if [ -f "$file" ]; then
        local last_modified=$(stat -c %Y "$file" 2>/dev/null || stat -f %m "$file" 2>/dev/null)
        local current_time=$(date +%s)
        local age_days=$(( (current_time - last_modified) / 86400 ))
        
        if [ $age_days -lt 7 ]; then
            echo -e "${GREEN}✅${NC} $description - Recent (${age_days} days old)"
        elif [ $age_days -lt 30 ]; then
            echo -e "${YELLOW}⚠️${NC}  $description - Moderate (${age_days} days old)"
        else
            echo -e "${RED}❌${NC} $description - Old (${age_days} days old)"
        fi
    else
        echo -e "${RED}❌${NC} $description - Missing file: $file"
    fi
}

echo ""
echo "📋 Core Documentation Files:"
echo "----------------------------"
check_file_status "README.md" "Main README"
check_file_status "CONTRIBUTING.md" "Contribution Guidelines"
check_file_status "DEVELOPMENT.md" "Development Guide"
check_file_status "CLI.md" "CLI Reference"
check_file_status "EXAMPLES.md" "Usage Examples"
check_file_status "PROTOCOL.md" "Protocol Specification"
check_file_status "SECURITY.md" "Security Policy"
check_file_status "PHASE3_PROGRESS.md" "Phase 3 Progress"

echo ""
echo "🔧 Project Management Files:"
echo "----------------------------"
check_file_status ".github/ISSUE_TEMPLATE/bug-report.yml" "Bug Report Template"
check_file_status ".github/ISSUE_TEMPLATE/feature-request.yml" "Feature Request Template"
check_file_status ".github/ISSUE_TEMPLATE/documentation.yml" "Documentation Template"
check_file_status ".github/ISSUE_TEMPLATE/security.yml" "Security Template"
check_file_status ".github/pull_request_template.md" "PR Template"
check_file_status "scripts/project/setup-github.sh" "GitHub Setup Script"
check_file_status "scripts/project/manage-board.sh" "Project Board Manager"
check_file_status "scripts/project/check-status.sh" "Status Checker Script"

echo ""
echo "🔗 Cross-Reference Checks:"
echo "-------------------------"

# Check for common documentation consistency issues
echo -n "Checking for broken internal links... "
if grep -r "\[.*\](.*\.md)" *.md | grep -v "http" | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    link=$(echo "$line" | sed 's/.*\[\([^]]*\)\](\([^)]*\)).*/\2/')
    if [ ! -f "$link" ] && [ ! -f "./$link" ]; then
        echo -e "${RED}❌${NC} Broken link in $file: $link"
        return 1
    fi
done; then
    echo -e "${GREEN}✅${NC}"
else
    echo -e "${RED}❌${NC}"
fi

echo -n "Checking for Phase 3 progress consistency... "
if grep -q "Phase 3.*60%" README.md && grep -q "Day 9.*COMPLETED" PHASE3_PROGRESS.md; then
    echo -e "${GREEN}✅${NC}"
else
    echo -e "${YELLOW}⚠️${NC}  Progress indicators may be inconsistent"
fi

echo -n "Checking for project board links... "
if grep -q "github.com/users/johnzilla/projects/2" README.md DEVELOPMENT.md; then
    echo -e "${GREEN}✅${NC}"
else
    echo -e "${RED}❌${NC} Project board links missing"
fi

echo ""
echo "📊 Documentation Metrics:"
echo "------------------------"
echo "Total markdown files: $(find . -name "*.md" | wc -l)"
echo "Total lines of documentation: $(find . -name "*.md" -exec wc -l {} \; | awk '{sum+=$1} END {print sum}')"
echo "GitHub issue templates: $(find .github/ISSUE_TEMPLATE -name "*.yml" | wc -l)"

echo ""
echo "🎯 Quick Actions:"
echo "---------------"
echo "• Update docs: Edit relevant .md files"
echo "• Check issues: ./scripts/project/check-status.sh"
echo "• Setup GitHub: ./scripts/project/setup-github.sh"
echo "• Manage board: ./scripts/project/manage-board.sh"
echo "• Project board: https://github.com/users/johnzilla/projects/2"

echo ""
echo "✨ Documentation check complete!"
