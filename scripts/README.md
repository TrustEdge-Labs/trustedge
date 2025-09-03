<!--
Copyright (c) 2025 John Turner
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# TrustEdge Scripts

Utility scripts for TrustEdge project management, testing, and development workflows.

## 📁 Directory Structure

```
scripts/
├── ci-check.sh            # Pre-commit CI validation script
├── fix-copyright.sh       # Copyright header maintenance
├── pre-commit.sh          # Git pre-commit hooks
├── check_documentation.sh # Documentation validation and checking
├── check_project_status.sh # Project status and health checking
├── setup_github_project.sh # GitHub project setup and configuration
├── project/               # Project management and GitHub utilities
│   ├── check-status.sh    # Check GitHub issues and project status
│   ├── setup-github.sh    # Setup GitHub milestones, labels, and project
│   ├── manage-board.sh    # Manage project board items and synchronization
│   └── check-docs.sh      # Validate documentation status and consistency
└── testing/               # Testing and validation scripts
    └── test-day9.sh       # Test Day 9 network resilience features
```

## 🚀 Quick Start

All scripts should be run from the project root directory:

```bash
# Run pre-commit CI checks (prevents GitHub CI failures)
./scripts/ci-check.sh

# Check documentation status and validation
./scripts/check_documentation.sh

# Check project status and health
./scripts/check_project_status.sh

# Setup GitHub project (initial configuration)
./scripts/setup_github_project.sh

# Advanced project management
./scripts/project/check-status.sh
./scripts/project/setup-github.sh
./scripts/project/manage-board.sh
./scripts/project/check-docs.sh

# Test network features
./scripts/testing/test-day9.sh
```

## 📋 Script Categories

### Core Development
Scripts for daily development workflows:

- **ci-check.sh**: Pre-commit CI validation script that runs the exact same checks as GitHub CI to prevent failures
- **fix-copyright.sh**: Automated copyright header maintenance
- **pre-commit.sh**: Git pre-commit hooks for code quality
- **check_documentation.sh**: Documentation validation and consistency checking
- **check_project_status.sh**: Project health monitoring and status reporting
- **setup_github_project.sh**: Initial GitHub project setup and configuration

### Project Management (`project/`)
Scripts for managing the GitHub project, issues, and documentation:

- **check-status.sh**: Monitor GitHub issues and development progress
- **setup-github.sh**: Initialize GitHub milestones, labels, and project structure
- **manage-board.sh**: Manage project board items and synchronization
- **check-docs.sh**: Validate documentation currency and cross-references

### Testing (`testing/`)
Scripts for testing and validation:

- **test-day9.sh**: Comprehensive testing of Day 9 network resilience features

## 🔧 Requirements

- **GitHub CLI** (`gh`) for project management scripts
- **Bash** shell environment
- **OpenSSL** for cryptographic operations in tests
- **Cargo/Rust** toolchain for building test targets

## 📝 Contributing

When adding new scripts:

1. **Choose appropriate directory** (`project/` vs `testing/`)
2. **Use kebab-case naming** (`new-script.sh`)
3. **Make executable** (`chmod +x`)
4. **Add description** to this README
5. **Include usage examples** in script headers

## 📚 Documentation

For detailed usage and examples, see:

- [DEVELOPMENT.md](../DEVELOPMENT.md) - Development workflows
- [EXAMPLES.md](../EXAMPLES.md) - Usage examples with scripts
- [CONTRIBUTING.md](../CONTRIBUTING.md) - Contribution guidelines
