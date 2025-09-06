#!/bin/bash

#
# Copyright (c) 2025 TRUSTEDGE LABS LLC
# MPL-2.0: https://mozilla.org/MPL/2.0/
# Project: trustedge — Privacy and trust at the edge.
# GitHub: https://github.com/johnzilla/trustedge
#


# TrustEdge Project Board Manager
# Add issues to the GitHub project board automatically
#
# Usage: ./scripts/project/manage-board.sh
# Run from project root directory

set -e

echo "📋 TrustEdge Project Board Manager"
echo "================================="

# Project configuration
PROJECT_NUMBER="2"
PROJECT_OWNER="johnzilla"
REPO="johnzilla/trustedge"

# Check if GitHub CLI is available and authenticated
if ! command -v gh &> /dev/null; then
    echo "❌ Error: GitHub CLI (gh) is not installed"
    exit 1
fi

if ! gh auth status &> /dev/null; then
    echo "❌ Error: Not authenticated with GitHub CLI"
    echo "   Run: gh auth login"
    exit 1
fi

echo "✅ GitHub CLI authenticated"

# Function to add issue to project board
add_issue_to_board() {
    local issue_number=$1
    local issue_title=$2
    
    echo "Adding issue #$issue_number to project board..."
    if gh project item-add $PROJECT_NUMBER --owner $PROJECT_OWNER --url "https://github.com/$REPO/issues/$issue_number" 2>/dev/null; then
        echo "✅ Added #$issue_number: $issue_title"
    else
        echo "⚠️  Issue #$issue_number may already be on the board or doesn't exist"
    fi
}

# Function to list items currently on the project board
list_board_items() {
    echo ""
    echo "📋 Current Project Board Items:"
    echo "------------------------------"
    gh project item-list $PROJECT_NUMBER --owner $PROJECT_OWNER --format json | jq -r '.items[] | select(.type == "ISSUE") | "#\(.content.number): \(.content.title)"' 2>/dev/null || echo "Could not retrieve board items"
}

# Function to add all Phase 3 issues
add_all_phase3_issues() {
    echo ""
    echo "🎯 Adding all Phase 3 issues to project board..."
    echo "----------------------------------------------"
    
    # Get all issues with phase-3 label
    gh issue list --label "phase-3" --json number,title --template '{{range .}}{{.number}}:{{.title}}{{"\n"}}{{end}}' | while IFS=: read -r number title; do
        if [ -n "$number" ] && [ -n "$title" ]; then
            add_issue_to_board "$number" "$title"
        fi
    done
}

# Main menu
echo ""
echo "🔧 Available Actions:"
echo "1. Add all Phase 3 issues to board"
echo "2. List current board items"
echo "3. Add specific issue (interactive)"
echo "4. Exit"
echo ""

read -p "Choose an action (1-4): " choice

case $choice in
    1)
        add_all_phase3_issues
        list_board_items
        ;;
    2)
        list_board_items
        ;;
    3)
        read -p "Enter issue number: " issue_num
        if [[ "$issue_num" =~ ^[0-9]+$ ]]; then
            issue_info=$(gh issue view $issue_num --json title --template '{{.title}}' 2>/dev/null)
            if [ $? -eq 0 ]; then
                add_issue_to_board "$issue_num" "$issue_info"
            else
                echo "❌ Issue #$issue_num not found"
            fi
        else
            echo "❌ Invalid issue number"
        fi
        ;;
    4)
        echo "👋 Goodbye!"
        exit 0
        ;;
    *)
        echo "❌ Invalid choice"
        exit 1
        ;;
esac

echo ""
echo "🔗 View your project board: https://github.com/users/$PROJECT_OWNER/projects/$PROJECT_NUMBER"
echo "✨ Project board management complete!"
