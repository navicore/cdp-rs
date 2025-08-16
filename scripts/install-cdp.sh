#!/bin/bash
# CDP Binary Installation Script
# Automatically downloads and installs CDP binaries for testing

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
CDP_VERSION="8.0"
CDP_DIR="${CDP_DIR:-$(pwd)/cdp-bin}"
CACHE_DIR="${HOME}/.cache/cdp-rs"

# Platform detection
detect_platform() {
    case "$(uname -s)" in
        Darwin*)
            PLATFORM="mac"
            CDP_URL="https://unstablesound.net/downloads/CDP8/CDPRel8Lite_Mac.zip"
            CDP_ARCHIVE="CDPRel8Lite_Mac.zip"
            ;;
        Linux*)
            PLATFORM="linux"
            # For Linux, we'll need to build from source
            CDP_URL="https://github.com/ComposersDesktop/CDP8/archive/refs/heads/master.zip"
            CDP_ARCHIVE="CDP8-master.zip"
            ;;
        MINGW*|MSYS*|CYGWIN*)
            PLATFORM="windows"
            CDP_URL="https://unstablesound.net/downloads/CDP8/CDPRel8Lite_PC.zip"
            CDP_ARCHIVE="CDPRel8Lite_PC.zip"
            ;;
        *)
            echo -e "${RED}Error: Unsupported platform$(uname -s)${NC}"
            exit 1
            ;;
    esac
}

# Check if CDP is already installed
check_existing() {
    if [ -d "$CDP_DIR" ] && [ -f "$CDP_DIR/.version" ]; then
        INSTALLED_VERSION=$(cat "$CDP_DIR/.version")
        if [ "$INSTALLED_VERSION" = "$CDP_VERSION" ]; then
            echo -e "${GREEN}CDP $CDP_VERSION already installed at $CDP_DIR${NC}"
            echo "Run 'make clean-cdp' to force reinstallation"
            exit 0
        fi
    fi
}

# Create directories
setup_directories() {
    echo "Setting up directories..."
    mkdir -p "$CDP_DIR"
    mkdir -p "$CACHE_DIR"
}

# Download CDP binaries
download_cdp() {
    echo -e "${YELLOW}Downloading CDP binaries for $PLATFORM...${NC}"
    
    # Check cache first
    if [ -f "$CACHE_DIR/$CDP_ARCHIVE" ]; then
        echo "Using cached download from $CACHE_DIR/$CDP_ARCHIVE"
    else
        echo "Downloading from $CDP_URL..."
        if command -v curl &> /dev/null; then
            curl -L -o "$CACHE_DIR/$CDP_ARCHIVE" "$CDP_URL"
        elif command -v wget &> /dev/null; then
            wget -O "$CACHE_DIR/$CDP_ARCHIVE" "$CDP_URL"
        else
            echo -e "${RED}Error: Neither curl nor wget found. Please install one.${NC}"
            exit 1
        fi
    fi
}

# Extract CDP binaries
extract_cdp() {
    echo "Extracting CDP binaries..."
    
    cd "$CDP_DIR"
    
    if [[ "$CDP_ARCHIVE" == *.zip ]]; then
        unzip -q "$CACHE_DIR/$CDP_ARCHIVE"
    elif [[ "$CDP_ARCHIVE" == *.tar.gz ]]; then
        tar -xzf "$CACHE_DIR/$CDP_ARCHIVE"
    else
        echo -e "${RED}Error: Unknown archive format${NC}"
        exit 1
    fi
    
    # Find the actual binary directory
    if [ "$PLATFORM" = "mac" ]; then
        # Check if we got a DMG file
        if [ -f "CDPRelease8-Lite.dmg" ]; then
            echo "Found DMG file, mounting..."
            
            # Mount the DMG
            hdiutil attach -quiet -nobrowse CDPRelease8-Lite.dmg
            
            # The mount point is typically /Volumes/CDPRelease8-Lite
            MOUNT_POINT="/Volumes/CDPRelease8-Lite"
            
            # Wait a moment for mounting
            sleep 1
            
            echo "DMG mounted at: $MOUNT_POINT"
            
            # Copy the CDP folder
            if [ -d "$MOUNT_POINT/cdp" ]; then
                echo "Copying CDP binaries..."
                cp -r "$MOUNT_POINT/cdp" .
            elif [ -d "$MOUNT_POINT/CDP" ]; then
                cp -r "$MOUNT_POINT/CDP" ./cdp
            fi
            
            # Unmount the DMG
            hdiutil detach -quiet "$MOUNT_POINT" || true
            
            # Remove the DMG file
            rm -f CDPRelease8-Lite.dmg
        fi
        
        # Mac binaries are usually in cdp/_cdp/_cdprogs
        if [ -d "cdp/_cdp/_cdprogs" ]; then
            echo "Found CDP binaries in cdp/_cdp/_cdprogs"
            CDP_BIN_DIR="$CDP_DIR/cdp/_cdp/_cdprogs"
        elif [ -d "cdp/bin" ]; then
            echo "Found CDP binaries in cdp/bin"
            CDP_BIN_DIR="$CDP_DIR/cdp/bin"
        elif [ -d "bin" ]; then
            echo "Found CDP binaries in bin"
            CDP_BIN_DIR="$CDP_DIR/bin"
        else
            echo -e "${YELLOW}Warning: Could not locate CDP binaries${NC}"
            echo "Please check directory structure:"
            find . -type d -name "*bin*" -o -name "*cdp*" 2>/dev/null | head -10
        fi
    elif [ "$PLATFORM" = "windows" ]; then
        # Windows binaries structure
        if [ -d "CDPR8_PC" ]; then
            mv CDPR8_PC/* . 2>/dev/null || true
            rmdir CDPR8_PC 2>/dev/null || true
        fi
        if [ -d "cdp/_cdp/_cdprogs" ]; then
            CDP_BIN_DIR="$CDP_DIR/cdp/_cdp/_cdprogs"
        elif [ -d "cdp/bin" ]; then
            CDP_BIN_DIR="$CDP_DIR/cdp/bin"
        elif [ -d "bin" ]; then
            CDP_BIN_DIR="$CDP_DIR/bin"
        fi
    fi
    
    # Mark installation version
    echo "$CDP_VERSION" > "$CDP_DIR/.version"
}

# Build from source for Linux
build_linux() {
    echo -e "${YELLOW}Building CDP from source for Linux...${NC}"
    
    cd "$CDP_DIR"
    
    # Extract source
    unzip -q "$CACHE_DIR/$CDP_ARCHIVE"
    cd CDP8-master || cd CDP8
    
    # Check for build dependencies
    if ! command -v cmake &> /dev/null; then
        echo -e "${RED}Error: cmake not found. Please install: sudo apt-get install cmake build-essential${NC}"
        exit 1
    fi
    
    # Build CDP
    echo "Building CDP (this may take a few minutes)..."
    mkdir -p build
    cd build
    cmake ..
    make -j$(nproc)
    
    # Copy binaries to expected location
    mkdir -p "$CDP_DIR/bin"
    find . -type f -executable -exec cp {} "$CDP_DIR/bin/" \; 2>/dev/null || true
    
    CDP_BIN_DIR="$CDP_DIR/bin"
    
    # Mark installation
    echo "$CDP_VERSION" > "$CDP_DIR/.version"
}

# Set up environment
setup_environment() {
    echo ""
    echo -e "${GREEN}CDP binaries installed successfully!${NC}"
    echo ""
    echo "Binary location: $CDP_BIN_DIR"
    echo ""
    echo "To use CDP binaries, either:"
    echo "  1. Export CDP_PATH environment variable:"
    echo "     export CDP_PATH=$CDP_BIN_DIR"
    echo ""
    echo "  2. Or the Makefile will automatically use:"
    echo "     $CDP_DIR/bin or $CDP_DIR/cdp/bin"
    echo ""
    
    # Create a script to set environment
    cat > "$CDP_DIR/env.sh" << EOF
#!/bin/bash
# Source this file to set up CDP environment
export CDP_PATH="$CDP_BIN_DIR"
export PATH="\$CDP_PATH:\$PATH"
echo "CDP environment configured: \$CDP_PATH"
EOF
    chmod +x "$CDP_DIR/env.sh"
    
    echo "You can also source the environment file:"
    echo "  source $CDP_DIR/env.sh"
}

# Verify installation
verify_installation() {
    echo ""
    echo "Verifying installation..."
    
    # Try to find a common CDP program
    if [ -f "$CDP_BIN_DIR/pvoc" ] || [ -f "$CDP_BIN_DIR/pvoc.exe" ]; then
        echo -e "${GREEN}✓ Found pvoc binary${NC}"
        
        # Count total binaries
        if [ "$PLATFORM" = "windows" ]; then
            COUNT=$(ls -1 "$CDP_BIN_DIR"/*.exe 2>/dev/null | wc -l)
        else
            COUNT=$(find "$CDP_BIN_DIR" -type f -executable 2>/dev/null | wc -l)
        fi
        echo -e "${GREEN}✓ Found $COUNT CDP programs${NC}"
        
        return 0
    else
        echo -e "${YELLOW}Warning: Could not verify pvoc binary${NC}"
        echo "CDP binaries may be in: $CDP_BIN_DIR"
        echo "Please verify manually with: ls -la $CDP_BIN_DIR"
        return 1
    fi
}

# Main installation flow
main() {
    echo "======================================"
    echo "CDP Binary Installation for cdp-rs"
    echo "======================================"
    echo ""
    
    detect_platform
    echo "Detected platform: $PLATFORM"
    
    check_existing
    setup_directories
    download_cdp
    
    if [ "$PLATFORM" = "linux" ]; then
        build_linux
    else
        extract_cdp
    fi
    
    setup_environment
    verify_installation
    
    echo ""
    echo -e "${GREEN}Installation complete!${NC}"
    echo "Run 'make test-cdp' to verify CDP binaries work correctly"
}

# Run main function
main "$@"