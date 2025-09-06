#!/bin/bash
#
# Copyright (c) 2025 TRUSTEDGE LABS LLC
# This source code is subject to the terms of the Mozilla Public License, v. 2.0.
# If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
#
# Project: trustedge — Privacy and trust at the edge.
#
# Script to update import statements from trustedge_audio to trustedge_core

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔄 Updating Rust import statements from trustedge_audio to trustedge_core...${NC}"

# Update all Rust files in trustedge-core
find trustedge-core -name "*.rs" -type f | while read -r file; do
    if grep -q "trustedge_audio" "$file"; then
        echo -e "${YELLOW}📝 Updating imports in: $file${NC}"
        
        # Create backup
        cp "$file" "${file}.bak"
        
        # Replace trustedge_audio with trustedge_core
        sed -i 's/trustedge_audio/trustedge_core/g' "$file"
        
        # Verify the change
        if grep -q "trustedge_core" "$file" && ! grep -q "trustedge_audio" "$file"; then
            echo -e "${GREEN}  ✅ Successfully updated${NC}"
            rm "${file}.bak"  # Remove backup on success
        else
            echo -e "${RED}  ❌ Update failed, restoring backup${NC}"
            mv "${file}.bak" "$file"
        fi
    fi
done

echo -e "${GREEN}✅ Rust import updates complete!${NC}"
