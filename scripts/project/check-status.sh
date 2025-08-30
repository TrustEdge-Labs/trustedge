#!/bin/bash

# TrustEdge Project Status Checker
# Quick status overview of Phase 3 development
# 
# Usage: ./scripts/project/check-status.sh
# Run from project root directory

set -e

echo "🚀 TrustEdge Phase 3 Development Status"
echo "======================================="

# Check if GitHub CLI is available
if ! command -v gh &> /dev/null; then
    echo "❌ Error: GitHub CLI (gh) is not installed"
    exit 1
fi

echo ""
echo "📊 Phase 3 Issues Status:"
echo "------------------------"

# Function to get issue status
get_issue_status() {
    local issue_num=$1
    local title=$2
    
    # Simple approach using gh issue view
    if gh issue view $issue_num >/dev/null 2>&1; then
        echo "📋 #$issue_num: $title"
        gh issue view $issue_num --json state,assignees --template '{{if eq .state "closed"}}   Status: ✅ COMPLETED{{else}}{{if .assignees}}   Status: 🔄 In Progress ({{range .assignees}}{{.login}} {{end}}){{else}}   Status: 📋 Open{{end}}{{end}}'
        echo ""
    else
        echo "❓ #$issue_num: $title (Not found)"
        echo ""
    fi
}

# Check each Phase 3 issue
get_issue_status "16" "Phase 3 Progress Tracker (Epic)"
get_issue_status "11" "Day 10: Server Authentication"
get_issue_status "12" "Day 11: Client Authentication"
get_issue_status "13" "Day 12: Enhanced Security"
get_issue_status "14" "Day 13: Production Deployment"
get_issue_status "15" "Day 14: Final Testing & Documentation"

echo "🔗 Quick Links:"
echo "- Project Board: https://github.com/users/johnzilla/projects/2"
echo "- Repository Issues: https://github.com/johnzilla/trustedge/issues"
echo "- Milestones: https://github.com/johnzilla/trustedge/milestones"

echo ""
echo "📋 Next Actions:"
echo "1. Visit your project board to organize issues into columns"
echo "2. Assign yourself to the next issue you want to work on"
echo "3. Update issue status as you make progress"
echo "4. Use 'gh issue list --milestone \"Day 10: Server Authentication\"' for detailed views"

echo ""
echo "🎯 Current Focus: Day 10 Server Authentication Implementation"
echo "   Ready to begin after completing Day 9 network resilience features"
