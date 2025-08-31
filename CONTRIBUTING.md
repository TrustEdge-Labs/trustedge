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
4. **Check progress**: See [GitHub Issues](https://github.com/johnzilla/trustedge/issues) and [Issue #16](https://github.com/johnzilla/trustedge/issues/16) for current status
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

- **License**: All contributions are licensed under MPL-2.0 for open-source use
- **CLA Required**: Contributors must sign our CLA before code can be merged
- **Copyright retention**: Contributors retain copyright on their original work
- **Dual licensing**: CLA enables commercial licensing to sustain the project
- **Clean IP**: Ensure you have the right to contribute your code

### For Maintainers

When accepting external PRs:

1. **Verify CLA is signed** via the CLA Assistant bot status
2. **Verify copyright headers** are appropriate for the contribution size
3. **Respect contributor copyright** - don't change their copyright line without permission  
4. **For major contributions**: Ensure Option 1 format is used
5. **For collaborative files**: Consider using collective copyright (Option 3)

---

## Contributor License Agreement (CLA)

### Why We Require a CLA

All code contributions to TrustEdge require signing our Contributor License Agreement (CLA). We want to be transparent about why this is necessary and what it means for you as a contributor.

**TL;DR**: You keep the copyright to your work, but you give the project permission to use it in flexible ways that help sustain the project long-term.

### What the CLA Means

#### 🏠 **You Retain Copyright**
The CLA is **not** a copyright assignment. You will always own the copyright to your original contributions. Your name stays on your code, and you can use your contributions however you wish.

#### 📜 **Project Gets Usage Rights**
The CLA grants TrustEdge (John Turner) a broad, perpetual, and irrevocable license to:
- Use your contribution in the open-source version
- Modify and improve your contribution
- Sublicense your contribution under different terms when necessary

#### 💼 **Why This Matters: Dual-Licensing Strategy**
TrustEdge uses a **dual-licensing model** to ensure project sustainability:

- **Open-Source License (MPL-2.0)**: Free for everyone to use, modify, and distribute
- **Commercial License**: Available for organizations that need different licensing terms

The revenue from commercial licensing helps fund:
- ✅ **Ongoing development** and maintenance
- ✅ **Security audits** and professional reviews  
- ✅ **Infrastructure costs** (CI/CD, hosting, etc.)
- ✅ **Long-term project viability** and feature development

Without the CLA, we couldn't offer commercial licenses, which would limit our ability to sustain and grow the project professionally.

### 🤖 How It Works

1. **Automated Process**: When you submit your first pull request, the CLA Assistant bot will automatically appear
2. **One-Time Signing**: You only need to sign once - it covers all future contributions
3. **Digital Signature**: Quick online process, no printing or mailing required
4. **Immediate Access**: Once signed, your PR can be reviewed and merged

### 📋 CLA Document

The full CLA document is available here: **[TrustEdge CLA](https://gist.github.com/johnzilla/0624a138cdabd17e9f5ae44607958922)** 

### Questions About the CLA?

We believe in transparency. If you have questions about:
- What rights you're granting
- How commercial licensing works  
- Why the CLA is structured this way
- Your copyright protections

Please [open a discussion](https://github.com/johnzilla/trustedge/discussions) or reach out via [email](mailto:john@trustedge.dev). We're happy to explain our approach and reasoning.

---

## 📄 Copyright and Licensing

### Copyright Headers for Contributions

**For external contributors**: After signing the CLA, you have several options for copyright headers:
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

**Current Focus**: Phase 3 - Network Operations & Authentication

**High Priority Issues**:
1. **[Day 10: Server Authentication](https://github.com/johnzilla/trustedge/issues/11)** - Implement server certificate validation and mutual TLS
2. **[Day 11: Client Authentication](https://github.com/johnzilla/trustedge/issues/12)** - Client certificate and token-based authentication  
3. **[Cross-platform Audio Capture](https://github.com/johnzilla/trustedge/issues/5)** - Integrate cpal for live audio streaming
4. **[Day 12: Enhanced Security](https://github.com/johnzilla/trustedge/issues/13)** - Perfect Forward Secrecy and additional algorithms

**Good First Issues**:
- **[Community Engagement](https://github.com/johnzilla/trustedge/issues/8)** - Beta testing program setup
- **[Example Configurations](https://github.com/johnzilla/trustedge/issues/9)** - Add deployment scenario examples

**Track Progress**: [Phase 3 Progress Tracker](https://github.com/johnzilla/trustedge/issues/16)

## 💬 Communication

- **Issues**: Use GitHub issues for bugs and feature requests
- **Discussions**: Use GitHub Discussions for questions and ideas
- **Email**: For private security reports only

## 📄 License

By contributing to TrustEdge, you agree that your contributions will be licensed under the [Mozilla Public License 2.0 (MPL-2.0)](./LICENSE).

---

Thank you for contributing to TrustEdge! 🚀
