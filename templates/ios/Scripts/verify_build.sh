#!/bin/bash
# iOS Build Verification Script
# Verifies that the generated iOS project builds successfully

set -euo pipefail

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

PROJECT_DIR="${1:-}"

if [[ -z "$PROJECT_DIR" ]]; then
    echo -e "${RED}[ERROR] Project directory required${NC}"
    echo "Usage: $0 <project_path>"
    exit 1
fi

if [[ ! -d "$PROJECT_DIR" ]]; then
    echo -e "${RED}[ERROR] Directory not found: $PROJECT_DIR${NC}"
    exit 1
fi

echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║            iOS Build Verification                          ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"

cd "$PROJECT_DIR"

# 1. Ensure dependencies are installed (in case they weren't)
echo -e "${BLUE}[INFO] Checking dependencies...${NC}"
if ! tuist install; then
    echo -e "${RED}[ERROR] Dependency installation failed${NC}"
    exit 1
fi

# 2. Build the App
echo -e "${BLUE}[INFO] Building App target...${NC}"
# Use --clean to ensure fresh build, but consider removing for speed if needed
if tuist build App; then
    echo -e "${GREEN}✅ Build verification PASSED${NC}"
    exit 0
else
    echo -e "${RED}❌ Build verification FAILED${NC}"
    exit 1
fi
