#!/bin/bash

# TrustEdge GitHub Project Management Setup
# This script helps organize issues and milestones for Phase 3 development

set -e

echo "🚀 TrustEdge GitHub Project Management Setup"
echo "==========================================="

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo "❌ Error: Not in a git repository"
    exit 1
fi

# Check if GitHub CLI is available
if ! command -v gh &> /dev/null; then
    echo "❌ Error: GitHub CLI (gh) is not installed"
    echo "   Install from: https://cli.github.com/"
    exit 1
fi

# Check if user is authenticated
if ! gh auth status &> /dev/null; then
    echo "❌ Error: Not authenticated with GitHub CLI"
    echo "   Run: gh auth login"
    exit 1
fi

echo "✅ Environment checks passed"
echo ""

# Get repository information
REPO=$(gh repo view --json nameWithOwner -q .nameWithOwner)
echo "📂 Repository: $REPO"

# Create milestones for Phase 3
echo ""
echo "📋 Creating Phase 3 milestones..."

# Phase 3 main milestone
gh api repos/$REPO/milestones -X POST -f title="Phase 3: Network Operations" \
    -f description="Production-ready network layer with authentication and security" \
    -f due_on="2025-02-28T23:59:59Z" || echo "   Milestone may already exist"

# Individual day milestones
gh api repos/$REPO/milestones -X POST -f title="Day 10: Server Authentication" \
    -f description="Implement server certificate validation and mutual TLS" \
    -f due_on="2025-02-07T23:59:59Z" || echo "   Milestone may already exist"

gh api repos/$REPO/milestones -X POST -f title="Day 11: Client Authentication" \
    -f description="Implement client authentication and authorization" \
    -f due_on="2025-02-14T23:59:59Z" || echo "   Milestone may already exist"

gh api repos/$REPO/milestones -X POST -f title="Day 12: Enhanced Security" \
    -f description="Perfect Forward Secrecy and advanced security features" \
    -f due_on="2025-02-21T23:59:59Z" || echo "   Milestone may already exist"

gh api repos/$REPO/milestones -X POST -f title="Day 13: Production Deployment" \
    -f description="Docker, monitoring, and production deployment features" \
    -f due_on="2025-02-28T23:59:59Z" || echo "   Milestone may already exist"

echo "✅ Milestones created"

# Create labels for organization
echo ""
echo "🏷️  Creating project labels..."

# Phase labels
gh label create "phase-3" --description "Phase 3: Network Operations" --color "0E8A16" || true
gh label create "day-10" --description "Day 10: Server Authentication" --color "1D76DB" || true
gh label create "day-11" --description "Day 11: Client Authentication" --color "1D76DB" || true
gh label create "day-12" --description "Day 12: Enhanced Security" --color "1D76DB" || true
gh label create "day-13" --description "Day 13: Production Deployment" --color "1D76DB" || true

# Component labels
gh label create "authentication" --description "Authentication and authorization" --color "D93F0B" || true
gh label create "networking" --description "Network protocols and connections" --color "FBCA04" || true
gh label create "crypto" --description "Cryptographic operations" --color "5319E7" || true
gh label create "deployment" --description "Production deployment and operations" --color "0052CC" || true

echo "✅ Labels created"

# Suggest next steps
echo ""
echo "📝 Next Steps:"
echo "1. Create issues for Day 10 server authentication:"
echo "   gh issue create --title 'Day 10: Server Authentication Implementation' --body-file ISSUE_DAY10_SERVER_AUTH.md --milestone 'Day 10: Server Authentication' --label 'enhancement,security,phase-3,day-10,authentication'"
echo ""
echo "2. Create issues for Day 11 client authentication:"
echo "   gh issue create --title 'Day 11: Client Authentication Implementation' --body-file ISSUE_DAY11_CLIENT_AUTH.md --milestone 'Day 11: Client Authentication' --label 'enhancement,security,phase-3,day-11,authentication'"
echo ""
echo "3. Create issues for remaining Phase 3 work"
echo ""
echo "4. Set up GitHub project board:"
echo "   Visit: https://github.com/$REPO/projects"
echo ""
echo "5. Review and update PHASE3_PROGRESS.md with issue links"

echo ""
echo "🎉 GitHub project management setup complete!"
echo "   Repository: https://github.com/$REPO"
echo "   Milestones: https://github.com/$REPO/milestones"
echo "   Labels: https://github.com/$REPO/labels"
