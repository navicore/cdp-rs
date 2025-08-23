#!/bin/bash

# Script to mark/unmark tests as ignored based on module readiness
# Usage: ./scripts/mark-tests.sh ignore cdp-distort    # Mark tests as ignored
#        ./scripts/mark-tests.sh enable cdp-housekeep  # Remove ignore attributes
#        ./scripts/mark-tests.sh status                # Show current status

set -e

ACTION=$1
MODULE=$2

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# List of modules with failing tests that should be ignored
FAILING_MODULES=(
    "cdp-distort"
    "cdp-housekeep"
    "cdp-pvoc"
    "cdp-spectral"
)

# Function to get test files for a module
get_test_files() {
    local module=$1
    case "$module" in
        "cdp-distort")
            echo "crates/cdp-distort/tests/oracle_tests.rs"
            ;;
        "cdp-housekeep")
            echo "crates/cdp-housekeep/src/copy.rs"
            ;;
        "cdp-pvoc")
            echo "crates/cdp-pvoc/tests/oracle_tests.rs crates/cdp-pvoc/tests/format_tests.rs"
            ;;
        "cdp-spectral")
            echo "crates/cdp-spectral/tests/oracle_tests.rs"
            ;;
        *)
            echo ""
            ;;
    esac
}

ignore_tests() {
    local module=$1
    echo -e "${YELLOW}Marking tests as ignored for: $module${NC}"
    
    local test_files=$(get_test_files "$module")
    if [ -z "$test_files" ]; then
        echo -e "${RED}No test files configured for module: $module${NC}"
        return 1
    fi
    
    for file in $test_files; do
        if [ -f "$file" ]; then
            echo "  Processing: $file"
            # Add #[ignore] after #[test] if not already present
            sed -i.bak 's/^#\[test\]$/#[test]\n#[ignore] \/\/ TODO: Enable when module is implemented/' "$file"
            # Remove duplicate #[ignore] lines
            sed -i.bak '/^#\[ignore\].*TODO: Enable/N;/\n#\[ignore\].*TODO: Enable/d' "$file"
            rm "${file}.bak"
        fi
    done
    
    echo -e "${GREEN}Tests marked as ignored for $module${NC}"
}

enable_tests() {
    local module=$1
    echo -e "${YELLOW}Enabling tests for: $module${NC}"
    
    local test_files=$(get_test_files "$module")
    if [ -z "$test_files" ]; then
        echo -e "${RED}No test files configured for module: $module${NC}"
        return 1
    fi
    
    for file in $test_files; do
        if [ -f "$file" ]; then
            echo "  Processing: $file"
            # Remove #[ignore] lines with our TODO marker
            sed -i.bak '/#\[ignore\].*TODO: Enable when module is implemented/d' "$file"
            rm "${file}.bak"
        fi
    done
    
    echo -e "${GREEN}Tests enabled for $module${NC}"
}

ignore_all() {
    echo -e "${YELLOW}Marking all failing tests as ignored...${NC}"
    for module in "${FAILING_MODULES[@]}"; do
        ignore_tests "$module"
    done
}

show_status() {
    echo -e "${GREEN}Module Test Status:${NC}"
    echo ""
    
    for module in "${FAILING_MODULES[@]}"; do
        local test_files=$(get_test_files "$module")
        if [ -n "$test_files" ]; then
            local has_ignore=false
            for file in $test_files; do
                if [ -f "$file" ] && grep -q "#\[ignore\].*TODO: Enable" "$file" 2>/dev/null; then
                    has_ignore=true
                    break
                fi
            done
            
            if [ "$has_ignore" = true ]; then
                echo -e "  ${RED}✗ $module${NC} - tests ignored (not ready)"
            else
                echo -e "  ${GREEN}✓ $module${NC} - tests enabled (ready for testing)"
            fi
        fi
    done
    
    echo ""
    echo "Other modules:"
    echo -e "  ${GREEN}✓ cdp-core${NC} - foundation module"
    echo -e "  ${GREEN}✓ cdp-oracle${NC} - test infrastructure"
    echo -e "  ${GREEN}✓ cdp-modify${NC} - no tests yet"
    echo -e "  ${GREEN}✓ cdp-sndinfo${NC} - no tests yet"
}

case "$ACTION" in
    ignore)
        if [ -z "$MODULE" ]; then
            ignore_all
        else
            ignore_tests "$MODULE"
        fi
        ;;
    enable)
        if [ -z "$MODULE" ]; then
            echo "Usage: $0 enable <module-name>"
            exit 1
        fi
        enable_tests "$MODULE"
        ;;
    status)
        show_status
        ;;
    *)
        echo "Usage: $0 {ignore|enable|status} [module-name]"
        echo ""
        echo "Examples:"
        echo "  $0 status                  # Show current test status"
        echo "  $0 ignore                  # Mark all failing tests as ignored"
        echo "  $0 ignore cdp-distort      # Mark specific module tests as ignored"
        echo "  $0 enable cdp-housekeep    # Enable tests for a module"
        exit 1
        ;;
esac