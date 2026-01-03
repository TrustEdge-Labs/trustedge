<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# TrustEdge Documentation Guide

**Complete documentation index for TrustEdge privacy-preserving edge computing.**

---

## 🚀 Quick Start

**New to TrustEdge? Start here:**

| Document | Purpose | Audience |
|----------|---------|----------|
| **[README.md](../README.md)** | Project overview, installation, basic usage | Everyone |
| **[CLI.md](user/cli.md)** | Complete command-line reference | Users |
| **[Examples Index](user/examples/README.md)** | Real-world usage examples | Users |

---

## 📚 User Guides

### Core Usage
| Document | Description | When to Use |
|----------|-------------|-------------|
| **[Examples Index](user/examples/README.md)** | Comprehensive examples and workflows | Learning by example |
| **[CLI.md](user/cli.md)** | Complete CLI reference and options | Command reference |
| **[TROUBLESHOOTING.md](user/troubleshooting.md)** | Error handling and solutions | When things go wrong |

### Authentication & Security
| Document | Description | When to Use |
|----------|-------------|-------------|
| **[AUTHENTICATION_GUIDE.md](user/authentication.md)** | Complete authentication setup guide | Setting up secure networks |
| **[SECURITY.md](../SECURITY.md)** | Security model and considerations | Security planning |
| **[THREAT_MODEL.md](technical/threat-model.md)** | Threat analysis and mitigations | Security assessment |

---

## 🏗️ Architecture & Technical Reference

### System Architecture
| Document | Description | Audience |
|----------|-------------|----------|
| **[UNIVERSAL_BACKEND.md](technical/universal-backend.md)** | Universal Backend system design | Developers |
| **[FORMAT.md](technical/format.md)** | Binary format specification | Developers |
| **[PROTOCOL.md](technical/protocol.md)** | Network protocol specification | Developers |
| **[WASM.md](../WASM.md)** | WebAssembly build and deployment guide | Developers |
| **[manifest_cam_video.md](manifest_cam_video.md)** | cam.video manifest specification | Developers |

### Hardware
| Document | Description | Audience |
|----------|-------------|----------|
| **[SECURE_NODE_MVP.md](hardware/SECURE_NODE_MVP.md)** | small ESP32-based reference board | Developers & Builders |

### Testing & Quality
| Document | Description | Audience |
|----------|-------------|----------|
| **[TESTING.md](developer/testing.md)** | Test procedures and validation | Developers/QA |
| **[TESTING_PATTERNS.md](developer/testing-patterns.md)** | Advanced testing patterns and best practices | Developers |
| **[WASM_TESTING.md](developer/wasm-testing.md)** | WebAssembly testing in browser environments | Developers |
| **[CODING_STANDARDS.md](developer/coding-standards.md)** | Code style and standards | Contributors |

---

## 🛠️ Development & Contributing

### Project Development
| Document | Description | Audience |
|----------|-------------|----------|
| **[ROADMAP.md](roadmap.md)** | Project roadmap and milestones | Stakeholders |
| **[DEVELOPMENT.md](developer/development.md)** | Development setup and workflows | Contributors |
| **[CONTRIBUTING.md](../CONTRIBUTING.md)** | Contribution guidelines | Contributors |

### Legal & Governance
| Document | Description | Audience |
|----------|-------------|----------|
| **[DCO.md](legal/dco.md)** | Developer Certificate of Origin | Contributors |
| **[CLA.md](legal/cla.md)** | Contributor License Agreement | Contributors |
| **[COPYRIGHT.md](legal/copyright.md)** | Copyright and licensing | Legal/Contributors |
| **[LICENSING.md](legal/licensing.md)** | Dual licensing strategy (MPL-2.0 + Commercial) | Legal/Business |
| **[ENTERPRISE.md](legal/enterprise.md)** | Enterprise licensing and support options | Business/Enterprise |

---

## 📋 Document Categories

### By User Type

#### 👤 **End Users**
1. [README.md](../README.md) - Start here
2. [CLI.md](user/cli.md) - Command reference
3. [Examples Index](user/examples/README.md) - Usage examples
4. [TROUBLESHOOTING.md](user/troubleshooting.md) - Problem solving
5. [AUTHENTICATION_GUIDE.md](user/authentication.md) - Security setup

#### 👨‍💻 **Developers** 
1. [UNIVERSAL_BACKEND.md](technical/universal-backend.md) - Architecture overview
2. [FORMAT.md](technical/format.md) - Binary format
3. [PROTOCOL.md](technical/protocol.md) - Network protocol
4. [TESTING.md](developer/testing.md) - Test procedures
5. [DEVELOPMENT.md](developer/development.md) - Dev environment
6. [CLAUDE.md](developer/claude.md) - AI assistant coding guidelines

#### 🤝 **Contributors**
1. [CONTRIBUTING.md](../CONTRIBUTING.md) - How to contribute
2. [CODING_STANDARDS.md](developer/coding-standards.md) - Code standards
3. [DCO.md](legal/dco.md) - Legal requirements
4. [ROADMAP.md](roadmap.md) - Project direction

#### 🔒 **Security Researchers**
1. [SECURITY.md](../SECURITY.md) - Security model
2. [THREAT_MODEL.md](technical/threat-model.md) - Threat analysis
3. [AUTHENTICATION_GUIDE.md](user/authentication.md) - Auth system

### By Topic

#### 🎯 **Getting Started**
- [README.md](../README.md) → [CLI.md](user/cli.md) → [Examples Index](user/examples/README.md)

#### 🔐 **Security Implementation**  
- [AUTHENTICATION_GUIDE.md](user/authentication.md) → [SECURITY.md](../SECURITY.md) → [THREAT_MODEL.md](technical/threat-model.md)

#### 🏗️ **System Architecture**
- [UNIVERSAL_BACKEND.md](technical/universal-backend.md) → [FORMAT.md](technical/format.md) → [PROTOCOL.md](technical/protocol.md)

#### 🧪 **Development & Testing**
- [DEVELOPMENT.md](developer/development.md) → [TESTING.md](developer/testing.md) → [CODING_STANDARDS.md](developer/coding-standards.md)

---

## 🎯 Recommended Reading Paths

### **Path 1: New User Journey**
```
README.md → CLI.md → EXAMPLES.md → TROUBLESHOOTING.md
```
*Complete user onboarding from installation to advanced usage*

### **Path 2: Security Setup**
```
README.md → AUTHENTICATION_GUIDE.md → SECURITY.md → EXAMPLES.md
```
*Secure deployment with authentication and network security*

### **Path 3: Developer Onboarding**
```
README.md → UNIVERSAL_BACKEND.md → DEVELOPMENT.md → TESTING.md → CONTRIBUTING.md
```
*Complete developer setup from architecture to contribution*

### **Path 4: Architecture Deep Dive**
```
UNIVERSAL_BACKEND.md → FORMAT.md → PROTOCOL.md → TESTING.md
```
*Technical understanding of TrustEdge system architecture*

### **Path 5: Security Analysis**
```
SECURITY.md → THREAT_MODEL.md → AUTHENTICATION_GUIDE.md → TESTING.md
```
*Comprehensive security assessment and implementation*

---

## 📖 Documentation Metrics

| Category | Documents | Total Lines | Purpose |
|----------|-----------|-------------|---------|
| **User Guides** | 5 docs | ~4,200 lines | User onboarding and usage |
| **Technical Reference** | 4 docs | ~2,800 lines | Architecture and protocols |
| **Development** | 4 docs | ~2,100 lines | Contributing and development |
| **Security** | 3 docs | ~1,900 lines | Security and threat analysis |
| **Project Meta** | 3 docs | ~800 lines | Legal and governance |

**Total: 19 documents, ~11,800 lines of comprehensive documentation**

---

## 🔍 Quick Reference

### Most Common Tasks

| Task | Primary Document | Supporting Documents |
|------|-----------------|---------------------|
| **Install TrustEdge** | [README.md](../README.md) | [TROUBLESHOOTING.md](user/troubleshooting.md) |
| **Learn CLI options** | [CLI.md](user/cli.md) | [Examples Index](user/examples/README.md) |
| **Set up authentication** | [AUTHENTICATION_GUIDE.md](user/authentication.md) | [SECURITY.md](../SECURITY.md) |
| **Understand architecture** | [UNIVERSAL_BACKEND.md](technical/universal-backend.md) | [FORMAT.md](technical/format.md), [PROTOCOL.md](technical/protocol.md) |
| **Contribute code** | [CONTRIBUTING.md](../CONTRIBUTING.md) | [DEVELOPMENT.md](developer/development.md), [CODING_STANDARDS.md](developer/coding-standards.md) |
| **Run tests** | [TESTING.md](developer/testing.md) | [DEVELOPMENT.md](developer/development.md) |
| **Report security issue** | [SECURITY.md](../SECURITY.md) | [THREAT_MODEL.md](technical/threat-model.md) |

### Emergency Situations

| Problem | Solution Document | Quick Action |
|---------|------------------|--------------|
| **Build fails** | [TROUBLESHOOTING.md](user/troubleshooting.md) | Check dependencies |
| **Authentication error** | [AUTHENTICATION_GUIDE.md](user/authentication.md) | Verify certificates |
| **Network connection issues** | [TROUBLESHOOTING.md](user/troubleshooting.md) | Check server status |
| **Test failures** | [TESTING.md](developer/testing.md) | Run `cargo test --verbose` |
| **Security concern** | [SECURITY.md](../SECURITY.md) | Follow responsible disclosure |

---

## 💡 Tips for Documentation Navigation

### **For New Users:**
1. **Always start with [README.md](../README.md)** for project overview
2. **Use [Examples Index](user/examples/README.md)** to learn by doing
3. **Keep [TROUBLESHOOTING.md](user/troubleshooting.md)** handy for issues

### **For Developers:**
1. **Begin with [UNIVERSAL_BACKEND.md](technical/universal-backend.md)** for architecture
2. **Review [TESTING.md](developer/testing.md)** before making changes
3. **Follow [CODING_STANDARDS.md](developer/coding-standards.md)** for consistency

### **For Contributors:**
1. **Read [CONTRIBUTING.md](../CONTRIBUTING.md)** first
2. **Understand [DCO.md](legal/dco.md)** for legal compliance
3. **Check [ROADMAP.md](roadmap.md)** for project direction

---

*This documentation index is maintained as part of the TrustEdge project. For updates or suggestions, see [CONTRIBUTING.md](../CONTRIBUTING.md).*

---

**📖 Links:**
- **[TrustEdge Home](https://github.com/TrustEdge-Labs/trustedge)** - Main repository
- **[TrustEdge Labs](https://github.com/TrustEdge-Labs)** - Organization profile
- **[Documentation](https://github.com/TrustEdge-Labs/trustedge/tree/main/docs)** - Complete docs
- **[Issues](https://github.com/TrustEdge-Labs/trustedge/issues)** - Bug reports & features

**⚖️ Legal:**
- **Copyright**: © 2025 TrustEdge Labs LLC
- **License**: Mozilla Public License 2.0 ([MPL-2.0](https://mozilla.org/MPL/2.0/))
- **Commercial**: [Enterprise licensing available](mailto:enterprise@trustedgelabs.com)
