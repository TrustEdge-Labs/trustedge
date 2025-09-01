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
├── project/          # Project management and GitHub utilities
│   ├── check-status.sh    # Check GitHub issues and project status
│   ├── setup-github.sh    # Setup GitHub milestones, labels, and project
│   ├── manage-board.sh    # Manage project board items and synchronization
│   └── check-docs.sh      # Validate documentation status and consistency
└── testing/          # Testing and validation scripts
    └── test-day9.sh       # Test Day 9 network resilience features
```

## 🚀 Quick Start

All scripts should be run from the project root directory:

```bash
# Check project status
./scripts/project/check-status.sh

# Setup GitHub project management
./scripts/project/setup-github.sh

# Manage project board items
./scripts/project/manage-board.sh

# Validate documentation
./scripts/project/check-docs.sh

# Test network features
./scripts/testing/test-day9.sh
```

## 📋 Script Categories

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
