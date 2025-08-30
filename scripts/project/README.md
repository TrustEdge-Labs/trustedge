<!--
Copyright (c) 2025 John Turner
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/johnzilla/trustedge
-->

# Project Management Scripts

GitHub project management and documentation utilities for TrustEdge.

## Scripts

### 📊 check-status.sh
Monitor GitHub issues and development progress for Phase 3.

**Usage:**
```bash
./scripts/project/check-status.sh
```

**Features:**
- Lists all Phase 3 issues with current status
- Shows assignees and labels
- Provides quick links to project board and milestones
- Displays next actions and current focus

### 🚀 setup-github.sh
Initialize GitHub project management infrastructure.

**Usage:**
```bash
./scripts/project/setup-github.sh
```

**Features:**
- Creates Phase 3 milestones (Day 10-14)
- Sets up project labels and organization
- Provides commands for creating issues
- Links to project board setup

**Requirements:**
- GitHub CLI authenticated (`gh auth login`)
- Repository write access

### � manage-board.sh
Manage GitHub project board items and synchronization.

**Usage:**
```bash
./scripts/project/manage-board.sh
```

**Features:**
- Add issues to project board automatically
- List current project board items
- Interactive issue management
- Bulk operations for labeled issues

**Note:** GitHub project boards require manual addition of issues. This script automates that process.

### �📚 check-docs.sh
Validate documentation status and consistency.

**Usage:**
```bash
./scripts/project/check-docs.sh
```

**Features:**
- Checks file currency (last modified dates)
- Validates internal documentation links
- Verifies project board references
- Reports documentation metrics
- Suggests maintenance actions

## Development

All project scripts should:

1. **Run from project root**: Use relative paths from repository root
2. **Check prerequisites**: Validate required tools (gh, git, etc.)
3. **Provide helpful output**: Clear status messages and next steps
4. **Handle errors gracefully**: Proper error messages and exit codes
5. **Include usage examples**: Help text and common scenarios
