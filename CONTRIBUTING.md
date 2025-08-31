<!--
Copyright (c) 2025 John Turner
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/johnzilla/trustedge
-->

# Contributing to TrustEdge

Thank you for your interest in contributing to TrustEdge! This document provides guidelines for contributing to the project.

## 🤝 Ways to Contribute

- **Bug Reports**: Help us find and fix issues
- **Feature Requests**: Suggest new capabilities
- **Documentation**: Improve guides, examples, and API docs
- **Code**: Implement features, fix bugs, or improve performance
- **Testing**: Write tests, test on different platforms
- **Security**: Report vulnerabilities or suggest security improvements

## 📋 Before You Start

1. **Check existing work**: Search [issues](https://github.com/trustedge/trustedge/issues) and [PRs](https://github.com/trustedge/trustedge/pulls)
2. **Read the docs**: Review [DEVELOPMENT.md](./DEVELOPMENT.md) for technical details
3. **Review coding standards**: See [CODING_STANDARDS.md](./CODING_STANDARDS.md) for style guidelines
4. **Check progress**: See [PHASE3_PROGRESS.md](./PHASE3_PROGRESS.md) for current status
5. **Understand the code**: Review the codebase structure and patterns

## 🐛 Reporting Issues

Use our issue templates for consistent reporting:

- **[Bug Report](./.github/ISSUE_TEMPLATE/bug-report.yml)**: For bugs and errors
- **[Feature Request](./.github/ISSUE_TEMPLATE/feature-request.yml)**: For new features
- **[Documentation](./.github/ISSUE_TEMPLATE/documentation.yml)**: For doc improvements
- **[Security](./.github/ISSUE_TEMPLATE/security.yml)**: For security issues

### Bug Report Best Practices

- **Be specific**: Include exact error messages and steps to reproduce
- **Provide context**: OS, Rust version, TrustEdge version
- **Include logs**: Add relevant command output or error logs
- **Test thoroughly**: Verify the issue exists in the latest version

## ✨ Feature Requests

When requesting features:

- **Explain the use case**: What problem does this solve?
- **Consider scope**: Is this a core feature or an optional enhancement?
- **Think about implementation**: Any technical constraints or suggestions?
- **Check compatibility**: Will this require breaking changes?

## 🔧 Code Contributions

### Development Setup

1. **Install Rust**: Use the latest stable version
2. **Clone the repo**: `git clone https://github.com/trustedge/trustedge.git`
3. **Install dependencies**: `cd trustedge/trustedge-audio && cargo build`
4. **Run tests**: `cargo test` and `cargo clippy`

### Project Board Management

**Important**: GitHub project boards require manual addition of issues.

- **View project board**: [TrustEdge Development](https://github.com/users/johnzilla/projects/2)
- **Add issues to board**: Use `./scripts/project/manage-board.sh`
- **GitHub Issues vs Project Board**: All issues are in `/issues`, but only manually added ones appear on the project board

### Coding Standards

**Follow the comprehensive coding standards**: See [CODING_STANDARDS.md](./CODING_STANDARDS.md) for detailed guidelines.

**Key requirements**:
- **Style**: Follow standard Rust formatting (`cargo fmt`)
- **Linting**: Pass all Clippy checks (`cargo clippy -- -D warnings`)
- **Terminal output**: Use professional UTF-8 symbols (✔, ✖, ⚠, ●, ♪, ■) instead of emojis
- **Testing**: Add tests for new functionality
- **Documentation**: Include doc comments for public APIs
- **Error handling**: Use proper error types and actionable messages
- **Security**: Follow cryptographic best practices

### Code Quality Requirements

See [CODING_STANDARDS.md](./CODING_STANDARDS.md) for detailed requirements. Essential checks:

- [ ] Code compiles without warnings
- [ ] All tests pass (`cargo test`)
- [ ] Clippy passes without errors (`cargo clippy -- -D warnings`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Professional UTF-8 symbols used (no emojis in terminal output)
- [ ] New code includes appropriate tests
- [ ] Public APIs include documentation
- [ ] Error messages are actionable and clear
- [ ] Commit messages are clear and descriptive

### Commit Message Format

```
type(scope): brief description

Longer explanation if needed.

Closes #issue-number
```

**Types**: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`
**Scopes**: `cli`, `server`, `client`, `crypto`, `format`, `docs`, `ci`

**Examples**:
- `feat(client): add connection retry with exponential backoff`
- `fix(crypto): handle edge case in chunk validation`
- `docs(protocol): clarify authentication flow`

## 🔍 Pull Request Process

1. **Create a branch**: `git checkout -b feature/your-feature-name`
2. **Make changes**: Follow coding standards and add tests
3. **Test thoroughly**: Run full test suite
4. **Update docs**: Modify relevant documentation
5. **Create PR**: Use our [PR template](./.github/pull_request_template.md)

### PR Checklist

- [ ] Branch is up to date with main
- [ ] All tests pass locally
- [ ] Code follows project style guidelines
- [ ] Appropriate documentation updated
- [ ] PR description explains changes clearly
- [ ] Related issues linked
- [ ] Security considerations addressed

### PR Review Process

1. **Automated checks**: CI must pass (tests, clippy, formatting)
2. **Code review**: Maintainer review for code quality and design
3. **Testing**: Functional testing of changes
4. **Documentation**: Review of updated docs
5. **Security review**: For changes affecting security or crypto

## 📄 Copyright and Licensing

### Copyright Headers for Contributions

**For external contributors**: When you submit a PR, you have several options for copyright headers:

#### Option 1: Contributor Copyright (Recommended)
```rust
// Copyright (c) 2025 Your Name
// Copyright (c) 2025 John Turner  
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//
```

#### Option 2: Assignment to Project
```rust
// Copyright (c) 2025 John Turner
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//
```

#### Option 3: Collective Copyright
```rust
// Copyright (c) 2025 TrustEdge Contributors
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//
```

### What We Recommend

1. **For substantial contributions** (new files, major features): Use **Option 1** with your name first
2. **For minor changes** (bug fixes, small improvements): Use **Option 2** or let maintainers add appropriate headers
3. **When in doubt**: Use **Option 1** - you retain copyright while licensing under MPL-2.0

### Legal Framework

- **License**: All contributions are licensed under MPL-2.0
- **Copyright retention**: Contributors can retain copyright on their work
- **No CLA required**: We don't require a Contributor License Agreement
- **Clean IP**: Ensure you have the right to contribute your code

### For Maintainers

When accepting external PRs:

1. **Verify copyright headers** are appropriate for the contribution size
2. **Respect contributor copyright** - don't change their copyright line without permission  
3. **For major contributions**: Ensure Option 1 format is used
4. **For collaborative files**: Consider using collective copyright (Option 3)
5. **Document significant contributors** in project acknowledgments

## 🔒 Security Contributions

### Reporting Vulnerabilities

- **Sensitive issues**: Use [private security advisory](https://github.com/trustedge/trustedge/security/advisories/new)
- **General security**: Use [security issue template](./.github/ISSUE_TEMPLATE/security.yml)
- **Follow responsible disclosure**: Don't publish exploits publicly

### Security Review Areas

- Cryptographic implementations
- Key management and storage
- Network protocol security
- Input validation and sanitization
- Authentication and authorization
- Dependency security

## 📚 Documentation Contributions

Good documentation is crucial for user adoption:

- **User guides**: Improve CLI.md, EXAMPLES.md
- **Developer docs**: Enhance DEVELOPMENT.md, code comments
- **Protocol specs**: Update PROTOCOL.md, FORMAT.md
- **Security docs**: Improve THREAT_MODEL.md, SECURITY.md

## 🧪 Testing Contributions

Help improve test coverage:

- **Unit tests**: Test individual functions and modules
- **Integration tests**: Test component interactions
- **End-to-end tests**: Test complete workflows
- **Platform testing**: Test on different operating systems
- **Performance tests**: Benchmark critical paths

## 🎯 Current Priorities

See [PHASE3_PROGRESS.md](./PHASE3_PROGRESS.md) for current development focus:

1. **Server Authentication** (Day 10)
2. **Client Authentication** (Day 11)
3. **Enhanced Security** (Day 12)
4. **Production Deployment** (Day 13)
5. **Final Testing** (Day 14)

## 💬 Communication

- **Issues**: Use GitHub issues for bugs and feature requests
- **Discussions**: Use GitHub Discussions for questions and ideas
- **Email**: For private security reports only

## 📄 License

By contributing to TrustEdge, you agree that your contributions will be licensed under the [Mozilla Public License 2.0 (MPL-2.0)](./LICENSE).

---

Thank you for contributing to TrustEdge! 🚀
