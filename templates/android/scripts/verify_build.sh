#!/bin/bash
# Android Build Verification Script
# Verifies that the generated Android project builds successfully

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
echo -e "${BLUE}║          Android Build Verification                        ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"

cd "$PROJECT_DIR"

# 1. Ensure gradlew is executable
chmod +x gradlew

# 2. Build the App
echo -e "${BLUE}[INFO] Building Debug variant...${NC}"

# Using assembleDebug to verify compilation without full release build overhead
if ./gradlew assembleDebug; then
    echo -e "${GREEN}✅ Build verification PASSED${NC}"
    exit 0
else
    echo -e "${RED}❌ Build verification FAILED${NC}"
    exit 1
fi
