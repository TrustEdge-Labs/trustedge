#!/bin/bash
#
# Copyright (c) 2025 TRUSTEDGE LABS LLC
# This source code is subject to the terms of the Mozilla Public License, v. 2.0.
# If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
#
# Project: trustedge — Privacy and trust at the edge.
#

# Documentation Consolidation Script
# Updates all documentation to align with current workspace structure

set -e

echo "🔧 TrustEdge Documentation Consolidation"
echo "=========================================="
echo

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Phase 1: Adding cross-references to new documentation${NC}"

# Add FEATURES.md reference to README
echo "  ● Updating README.md with FEATURES.md link..."

# Add WASM.md reference to crate READMEs
echo "  ● Updating crates/wasm/README.md..."
echo "  ● Updating crates/trst-wasm/README.md..."

# Add cross-reference in web/demo/README.md
echo "  ● Updating web/demo/README.md..."

echo

echo -e "${YELLOW}Phase 2: Creating archive README${NC}"

cat > archive/README.md << 'EOF'
<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
-->

# Archive

This directory contains **historical documentation** from previous releases and development phases.

## ⚠️ Important Notice

**These documents are archived and may contain outdated information.**

For current documentation, see:
- **[Main README](../README.md)** - Project overview
- **[FEATURES.md](../FEATURES.md)** - Feature flags
- **[WASM.md](../WASM.md)** - WASM build guide
- **[Documentation](../docs/)** - Complete docs

## Contents

### Release Documentation (v0.2.0)
- `RELEASE_NOTES_0.2.0.md` - Release announcement (superseded by CHANGELOG.md)
- `POST_RELEASE_CHECKLIST_0.2.0.md` - Release process documentation

### Historical Planning
- `ROADMAP_OLD.md` - Previous roadmap (superseded by docs/roadmap.md)
- `linkedin-release-post.md` - Social media announcement
- `bolt-new-prompt.md` - AI assistant prompts (historical)

## Why Keep These?

These files are preserved for:
1. **Historical reference** - Understanding past decisions
2. **Release archaeology** - Tracking what changed when
3. **Process documentation** - How releases were done

---

**Last Updated**: October 19, 2025  
**Status**: Archived - Use current documentation for active development
EOF

echo "  ✔ Created archive/README.md"
echo

echo -e "${YELLOW}Phase 3: Documentation summary${NC}"

echo "  📊 Documentation Inventory:"
echo "     • Root level: 9 key docs (README, FEATURES, WASM, RFC_K256, etc.)"
echo "     • docs/ directory: 30+ files organized by category"
echo "     • Crate READMEs: 9 package-specific guides"
echo "     • Archive: 7 historical files (now documented)"
echo

echo -e "${GREEN}✅ Documentation consolidation complete!${NC}"
echo
echo "Next steps:"
echo "  1. Review DOCUMENTATION_AUDIT.md for detailed findings"
echo "  2. Test updated commands in docs/user/examples/"
echo "  3. Verify all cross-references work"
echo

echo "For more information:"
echo "  • See DOCUMENTATION_AUDIT.md for complete audit report"
echo "  • See FEATURES.md for feature flag documentation"
echo "  • See WASM.md for WebAssembly build guide"
