<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: sealedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/sealedge
-->

# Sealedge Copyright Management

This document describes the automated copyright header management system for Sealedge.

## Overview

Sealedge uses automated tools to ensure all source files have proper copyright headers, protecting your intellectual property and ensuring compliance with the MPL-2.0 license.

## Automated Systems

### 🤖 GitHub Actions (Automatic)

**File:** `.github/workflows/copyright-check.yml`

This GitHub Action automatically:
- Runs on every push and pull request
- Checks all source files for copyright headers
- **Automatically adds missing headers and commits them**
- Prevents any code from being merged without proper attribution

**Supported File Types:**
- Rust files (`.rs`)
- Markdown files (`.md`) 
- YAML files (`.yml`, `.yaml`)
- TOML files (`.toml`)

### 🔧 Local Development Tools

#### Make Commands
```bash
# Check and fix copyright headers
make copyright-check

# Install pre-commit hooks
make install-hooks

# Run all quality checks including copyright
make full-check

# Set up complete development environment
make dev-setup
```

#### Manual Scripts
```bash
# Check and add copyright headers
./scripts/fix-copyright.sh

# Install pre-commit hook
cp scripts/pre-commit.sh .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

### 🛡️ Pre-Commit Protection

The pre-commit hook prevents commits of files missing copyright headers:

```bash
# Install the hook
make install-hooks

# Now all commits are automatically checked
git commit -m "my changes"  # Will fail if headers missing
```

## Copyright Header Formats

### Rust Files (`.rs`)
```rust
//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: sealedge — Privacy and trust at the edge.
//
```

### Markdown Files (`.md`)
```markdown
<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: sealedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/sealedge
-->
```

### YAML/TOML Files (`.yml`, `.yaml`, `.toml`)
```yaml
# Copyright (c) 2025 TRUSTEDGE LABS LLC
# MPL-2.0: https://mozilla.org/MPL/2.0/
# Project: sealedge — Privacy and trust at the edge.
```

## Workflow Integration

### For Developers

1. **Setup once:**
   ```bash
   make dev-setup
   ```

2. **Regular development:**
   - Pre-commit hooks automatically check your changes
   - GitHub Actions ensure no files slip through
   - Use `make copyright-check` if you need to fix headers manually

### For Contributors

- The GitHub Action will automatically add copyright headers to any PRs that are missing them
- No action required from contributors
- Headers are added and committed automatically

### For CI/CD

The copyright check is integrated into the main CI pipeline and will:
- ✅ **Pass**: All files have proper headers
- 🔄 **Auto-fix**: Missing headers are added automatically
- ❌ **Fail**: Only if the auto-fix process fails

## Benefits

✅ **Automatic Protection**: Every file that enters the repository gets proper attribution  
✅ **Zero Maintenance**: No manual work required after setup  
✅ **Legal Compliance**: Ensures MPL-2.0 license requirements are met  
✅ **Historical Protection**: Even old versions in git history will have headers added  
✅ **Developer Friendly**: Pre-commit hooks catch issues before they reach GitHub  

## Troubleshooting

### Header Missing Error
```bash
❌ Missing copyright header: src/new_file.rs
```
**Solution:** Run `make copyright-check` to automatically add the header

### Pre-commit Hook Blocking
```bash
❌ Commit blocked: 1 files missing copyright headers
💡 Run 'make fix-copyright' to automatically add headers
```
**Solution:** Run `make copyright-check` then retry the commit

### GitHub Action Failure
If the GitHub Action fails, check the logs. It will show which files need headers and attempt to fix them automatically.

## Security Note

The GitHub Action has permission to commit changes to your repository. This is necessary to automatically add copyright headers. The action only commits copyright header additions with the message "chore: add missing copyright headers [skip ci]".

---

*This copyright management system ensures that your intellectual property is properly protected in every version of every file that gets committed to the repository.*
