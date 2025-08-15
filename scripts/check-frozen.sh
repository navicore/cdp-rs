#!/bin/bash

# CDP-RS Frozen Module Checker
# This script ensures frozen modules haven't been modified

set -e

FROZEN_MODULES="cdp-core cdp-pvoc cdp-spectral"
ERRORS=0

echo "Checking frozen modules..."
echo "=========================="

for module in $FROZEN_MODULES; do
    echo -n "Checking $module... "
    
    # Check if module directory exists
    if [ ! -d "$module" ]; then
        echo "✓ (not yet created)"
        continue
    fi
    
    # Check for forbid(unsafe_code) in lib.rs
    if grep -q "#!\[forbid(unsafe_code)\]" "$module/src/lib.rs" 2>/dev/null; then
        echo "✓ (frozen)"
    else
        echo "✗ (missing forbid(unsafe_code))"
        ERRORS=$((ERRORS + 1))
    fi
done

echo ""

if [ $ERRORS -gt 0 ]; then
    echo "ERROR: $ERRORS frozen modules are not properly marked!"
    echo "Add '#![forbid(unsafe_code)]' to the top of lib.rs in frozen modules."
    exit 1
else
    echo "All frozen modules are properly marked."
fi

# Check if running in git repository
if git rev-parse --git-dir > /dev/null 2>&1; then
    echo ""
    echo "Checking for uncommitted changes in frozen modules..."
    
    for module in $FROZEN_MODULES; do
        if [ -d "$module" ]; then
            if git diff --quiet HEAD -- "$module"; then
                echo "  $module: ✓ no changes"
            else
                echo "  $module: ✗ has uncommitted changes!"
                ERRORS=$((ERRORS + 1))
            fi
        fi
    done
    
    if [ $ERRORS -gt 0 ]; then
        echo ""
        echo "ERROR: Frozen modules have uncommitted changes!"
        echo "Frozen modules should not be modified without approval."
        exit 1
    fi
fi

echo ""
echo "Frozen module check complete!"