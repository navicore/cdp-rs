#!/bin/bash

# Script to enable or disable modules in the workspace
# Usage: ./scripts/toggle-module.sh enable cdp-housekeep
#        ./scripts/toggle-module.sh disable cdp-pvoc
#        ./scripts/toggle-module.sh status

set -e

ACTION=$1
MODULE=$2

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

show_status() {
    echo -e "${GREEN}Active modules:${NC}"
    grep -A 20 "^members = \[" Cargo.toml | grep "crates/" | sed 's/.*"crates\//  - /g' | sed 's/".*//' | grep -v "^  - #"
    
    echo -e "\n${RED}Excluded modules:${NC}"
    grep -A 20 "^exclude = \[" Cargo.toml | grep "crates/" | sed 's/.*"crates\//  - /g' | sed 's/".*//'
}

enable_module() {
    if [ -z "$MODULE" ]; then
        echo "Usage: $0 enable <module-name>"
        exit 1
    fi
    
    echo -e "${YELLOW}Enabling module: $MODULE${NC}"
    
    # Check if module exists
    if [ ! -d "crates/$MODULE" ]; then
        echo -e "${RED}Error: Module crates/$MODULE does not exist${NC}"
        exit 1
    fi
    
    # Create a temporary file
    TMP_FILE=$(mktemp)
    
    # Process the Cargo.toml
    python3 - <<EOF
import re

with open('Cargo.toml', 'r') as f:
    content = f.read()

# Remove from exclude list
module_pattern = rf'^\s*"crates/{MODULE}".*$'
content = re.sub(module_pattern, '', content, flags=re.MULTILINE)

# Add to members list (before the closing bracket)
members_pattern = r'(members = \[[^\]]*)'
replacement = rf'\1    "crates/{MODULE}",\n'
content = re.sub(members_pattern, replacement, content)

# Clean up empty lines
content = re.sub(r'\n\s*\n\s*\n', '\n\n', content)

with open('$TMP_FILE', 'w') as f:
    f.write(content)
EOF
    
    # Replace the original file
    mv "$TMP_FILE" Cargo.toml
    
    echo -e "${GREEN}Module $MODULE enabled!${NC}"
    echo "Running cargo check to verify..."
    cargo check --package "$MODULE"
}

disable_module() {
    if [ -z "$MODULE" ]; then
        echo "Usage: $0 disable <module-name>"
        exit 1
    fi
    
    echo -e "${YELLOW}Disabling module: $MODULE${NC}"
    
    # Create a temporary file
    TMP_FILE=$(mktemp)
    
    # Process the Cargo.toml
    python3 - <<EOF
import re

with open('Cargo.toml', 'r') as f:
    content = f.read()

# Remove from members list
module_pattern = rf'^\s*"crates/{MODULE}".*$'
content = re.sub(module_pattern, '', content, flags=re.MULTILINE)

# Add to exclude list (before the closing bracket)
exclude_pattern = r'(exclude = \[[^\]]*)'
replacement = rf'\1    "crates/{MODULE}",\n'
content = re.sub(exclude_pattern, replacement, content)

# Clean up empty lines
content = re.sub(r'\n\s*\n\s*\n', '\n\n', content)

with open('$TMP_FILE', 'w') as f:
    f.write(content)
EOF
    
    # Replace the original file
    mv "$TMP_FILE" Cargo.toml
    
    echo -e "${GREEN}Module $MODULE disabled!${NC}"
}

case "$ACTION" in
    enable)
        enable_module
        ;;
    disable)
        disable_module
        ;;
    status)
        show_status
        ;;
    *)
        echo "Usage: $0 {enable|disable|status} [module-name]"
        echo ""
        echo "Examples:"
        echo "  $0 status                    # Show current module status"
        echo "  $0 enable cdp-housekeep      # Enable a module"
        echo "  $0 disable cdp-pvoc          # Disable a module"
        exit 1
        ;;
esac