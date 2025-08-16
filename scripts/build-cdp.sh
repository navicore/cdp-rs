#!/bin/bash
# Build CDP from source in our workspace
# This ensures reproducible builds without system installation

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
CDP_VERSION="8"
BUILD_DIR="$(pwd)/build/cdp"
INSTALL_DIR="$(pwd)/build/cdp-install"
CDP_GIT_URL="https://github.com/ComposersDesktop/CDP8.git"

# Create build directories
setup_directories() {
    echo "Setting up build directories..."
    mkdir -p "$BUILD_DIR"
    mkdir -p "$INSTALL_DIR"
}

# Clone or update CDP source
get_source() {
    echo -e "${YELLOW}Getting CDP source code...${NC}"
    
    if [ -d "$BUILD_DIR/.git" ]; then
        echo "Updating existing CDP source..."
        cd "$BUILD_DIR"
        # Stash any existing changes before pulling
        if ! git diff --quiet || ! git diff --cached --quiet; then
            echo "Stashing existing changes..."
            git stash push -m "Build script auto-stash"
        fi
        git pull
    else
        echo "Cloning CDP source..."
        git clone "$CDP_GIT_URL" "$BUILD_DIR"
        cd "$BUILD_DIR"
    fi
    
    # Get the latest commit hash for reproducibility
    COMMIT_HASH=$(git rev-parse HEAD)
    echo "CDP commit: $COMMIT_HASH"
    echo "$COMMIT_HASH" > "$INSTALL_DIR/.commit"
}

# Apply patches to fix compatibility issues
apply_patches() {
    echo "Applying compatibility patches..."
    
    cd "$BUILD_DIR"
    
    # Check if patches directory exists
    PATCH_DIR="$(dirname "$0")/patches"
    if [ -d "$PATCH_DIR" ]; then
        for patch in "$PATCH_DIR"/*.patch; do
            if [ -f "$patch" ]; then
                echo "Applying $(basename "$patch")..."
                patch -p1 < "$patch" || {
                    echo -e "${YELLOW}Warning: Patch $(basename "$patch") may have already been applied${NC}"
                }
            fi
        done
    fi
    
    # Additional fixes that are easier to do with sed
    echo "Fixing additional compatibility issues..."
    
    # Fix C++ standard library issues on macOS
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # Remove -stdlib=libc++ which causes issues with newer clang
        find . -name "CMakeLists.txt" -exec sed -i '' 's/-stdlib=libc++//g' {} \;
    fi
    
    # Fix any outdated CMAKE_MINIMUM_REQUIRED that the patch might have missed
    if [[ "$OSTYPE" == "darwin"* ]]; then
        find . -name "CMakeLists.txt" -exec sed -i '' 's/cmake_minimum_required(VERSION 2\.[0-9]\.[0-9])/cmake_minimum_required(VERSION 3.5)/g' {} \;
        find . -name "CMakeLists.txt" -exec sed -i '' 's/cmake_minimum_required(VERSION 2\.[0-9])/cmake_minimum_required(VERSION 3.5)/g' {} \;
    else
        find . -name "CMakeLists.txt" -exec sed -i 's/cmake_minimum_required(VERSION 2\.[0-9]\.[0-9])/cmake_minimum_required(VERSION 3.5)/g' {} \;
        find . -name "CMakeLists.txt" -exec sed -i 's/cmake_minimum_required(VERSION 2\.[0-9])/cmake_minimum_required(VERSION 3.5)/g' {} \;
    fi
    
    # Fix dirent compatibility on macOS
    if [[ "$OSTYPE" == "darwin"* ]]; then
        if [ -f "dev/newsfsys/sfdir.c" ]; then
            echo "Fixing dirent compatibility for macOS..."
            sed -i '' 's/#if defined WIN32 || defined linux/#if defined WIN32 || defined linux || defined __APPLE__/g' dev/newsfsys/sfdir.c
        fi
    fi
    
    # Fix paprogs conditional compilation
    if [ -f "dev/externals/CMakeLists.txt" ]; then
        echo "Fixing paprogs conditional..."
        if [[ "$OSTYPE" == "darwin"* ]]; then
            sed -i '' 's/##if(USE_LOCAL_PORTAUDIO)/if(USE_LOCAL_PORTAUDIO)/g' dev/externals/CMakeLists.txt
            sed -i '' 's/##endif()/endif()/g' dev/externals/CMakeLists.txt
        else
            sed -i 's/##if(USE_LOCAL_PORTAUDIO)/if(USE_LOCAL_PORTAUDIO)/g' dev/externals/CMakeLists.txt
            sed -i 's/##endif()/endif()/g' dev/externals/CMakeLists.txt
        fi
    fi
    
    # Disable programs that require aaio.h (audio I/O library we don't have)
    echo "Disabling programs that require audio I/O libraries..."
    for cmake_file in "dev/new/CMakeLists.txt" "dev/science/CMakeLists.txt"; do
        if [ -f "$cmake_file" ]; then
            if [[ "$OSTYPE" == "darwin"* ]]; then
                # Comment out problematic executables and their related lines
                sed -i '' 's/^add_executable(fracture/#add_executable(fracture/g' "$cmake_file"
                sed -i '' 's/^target_link_libraries(fracture/#target_link_libraries(fracture/g' "$cmake_file"
                sed -i '' 's/^my_install(fracture)/#my_install(fracture)/g' "$cmake_file"
                
                sed -i '' 's/^add_executable(iterfof/#add_executable(iterfof/g' "$cmake_file"
                sed -i '' 's/^target_link_libraries(iterfof/#target_link_libraries(iterfof/g' "$cmake_file"
                sed -i '' 's/^my_install(iterfof)/#my_install(iterfof)/g' "$cmake_file"
                
                sed -i '' 's/^add_executable(newtex/#add_executable(newtex/g' "$cmake_file"
                sed -i '' 's/^target_link_libraries(newtex/#target_link_libraries(newtex/g' "$cmake_file"
                sed -i '' 's/^my_install(newtex)/#my_install(newtex)/g' "$cmake_file"
                
                sed -i '' 's/^add_executable(newsynth/#add_executable(newsynth/g' "$cmake_file"
                sed -i '' 's/^target_link_libraries(newsynth/#target_link_libraries(newsynth/g' "$cmake_file"
                sed -i '' 's/^my_install(newsynth)/#my_install(newsynth)/g' "$cmake_file"
                
                sed -i '' 's/^add_executable(strands/#add_executable(strands/g' "$cmake_file"
                sed -i '' 's/^target_link_libraries(strands/#target_link_libraries(strands/g' "$cmake_file"
                sed -i '' 's/^my_install(strands)/#my_install(strands)/g' "$cmake_file"
            else
                sed -i 's/^add_executable(fracture/#add_executable(fracture/g' "$cmake_file"
                sed -i 's/^target_link_libraries(fracture/#target_link_libraries(fracture/g' "$cmake_file"
                sed -i 's/^my_install(fracture)/#my_install(fracture)/g' "$cmake_file"
                
                sed -i 's/^add_executable(iterfof/#add_executable(iterfof/g' "$cmake_file"
                sed -i 's/^target_link_libraries(iterfof/#target_link_libraries(iterfof/g' "$cmake_file"
                sed -i 's/^my_install(iterfof)/#my_install(iterfof)/g' "$cmake_file"
                
                sed -i 's/^add_executable(newtex/#add_executable(newtex/g' "$cmake_file"
                sed -i 's/^target_link_libraries(newtex/#target_link_libraries(newtex/g' "$cmake_file"
                sed -i 's/^my_install(newtex)/#my_install(newtex)/g' "$cmake_file"
                
                sed -i 's/^add_executable(newsynth/#add_executable(newsynth/g' "$cmake_file"
                sed -i 's/^target_link_libraries(newsynth/#target_link_libraries(newsynth/g' "$cmake_file"
                sed -i 's/^my_install(newsynth)/#my_install(newsynth)/g' "$cmake_file"
                
                sed -i 's/^add_executable(strands/#add_executable(strands/g' "$cmake_file"
                sed -i 's/^target_link_libraries(strands/#target_link_libraries(strands/g' "$cmake_file"
                sed -i 's/^my_install(strands)/#my_install(strands)/g' "$cmake_file"
            fi
        fi
    done
    
    echo -e "${GREEN}✓ Patches applied${NC}"
}

# Check build dependencies
check_dependencies() {
    echo "Checking build dependencies..."
    
    local missing_deps=()
    
    # Check for C compiler
    if ! command -v cc &> /dev/null && ! command -v gcc &> /dev/null && ! command -v clang &> /dev/null; then
        missing_deps+=("C compiler (gcc/clang)")
    fi
    
    # Check for make
    if ! command -v make &> /dev/null; then
        missing_deps+=("make")
    fi
    
    # Check for cmake (CDP uses cmake)
    if ! command -v cmake &> /dev/null; then
        missing_deps+=("cmake")
    fi
    
    if [ ${#missing_deps[@]} -gt 0 ]; then
        echo -e "${RED}Missing dependencies:${NC}"
        printf '%s\n' "${missing_deps[@]}"
        echo ""
        echo "Install them with:"
        
        if [[ "$OSTYPE" == "darwin"* ]]; then
            echo "  brew install cmake"
        elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
            echo "  sudo apt-get install build-essential cmake"
        fi
        
        return 1
    fi
    
    echo -e "${GREEN}✓ All dependencies found${NC}"
    return 0
}

# Build CDP
build_cdp() {
    echo -e "${YELLOW}Building CDP...${NC}"
    
    cd "$BUILD_DIR"
    
    # Create build directory
    mkdir -p build
    cd build
    
    # Configure with cmake
    echo "Configuring build..."
    # Add policy settings to handle old CMake code
    cmake .. \
        -DCMAKE_BUILD_TYPE=Release \
        -DCMAKE_INSTALL_PREFIX="$INSTALL_DIR" \
        -DCMAKE_POLICY_DEFAULT_CMP0025=NEW \
        -DCMAKE_POLICY_DEFAULT_CMP0042=NEW \
        -DCMAKE_OSX_DEPLOYMENT_TARGET=10.15 \
        -DUSE_LOCAL_PORTAUDIO=OFF \
        -DAAIOLIB=""
    
    # Build
    echo "Compiling (this may take a few minutes)..."
    if [[ "$OSTYPE" == "darwin"* ]]; then
        make -j$(sysctl -n hw.ncpu) || {
            echo -e "${YELLOW}Warning: Some programs failed to build (likely audio I/O dependent)${NC}"
            echo -e "${YELLOW}Continuing with programs that built successfully...${NC}"
        }
    else
        make -j$(nproc) || {
            echo -e "${YELLOW}Warning: Some programs failed to build (likely audio I/O dependent)${NC}"
            echo -e "${YELLOW}Continuing with programs that built successfully...${NC}"
        }
    fi
    
    # Install to our local directory
    echo "Installing to $INSTALL_DIR..."
    make install || {
        echo -e "${YELLOW}Note: Install step failed for some programs, copying built binaries...${NC}"
    }
    
    # CDP installs programs in different places, let's consolidate them
    echo "Organizing binaries..."
    mkdir -p "$INSTALL_DIR/bin"
    
    # Copy binaries from NewRelease directory if they exist
    if [ -d "$BUILD_DIR/NewRelease" ]; then
        echo "Copying binaries from NewRelease..."
        cp "$BUILD_DIR/NewRelease"/* "$INSTALL_DIR/bin/" 2>/dev/null || true
    fi
    
    # Find all executables and copy to bin
    find "$INSTALL_DIR" -type f -perm +111 -exec file {} \; 2>/dev/null | \
        grep -E "executable|binary" | \
        cut -d: -f1 | \
        while read exe; do
            if [[ ! "$exe" == *".dylib"* ]] && [[ ! "$exe" == *".so"* ]]; then
                cp "$exe" "$INSTALL_DIR/bin/" 2>/dev/null || true
            fi
        done
    
    # Also check the build directory for executables
    find . -type f -perm +111 -exec file {} \; 2>/dev/null | \
        grep -E "executable|binary" | \
        cut -d: -f1 | \
        while read exe; do
            if [[ ! "$exe" == *".dylib"* ]] && [[ ! "$exe" == *".so"* ]] && [[ ! "$exe" == *".a"* ]]; then
                basename_exe=$(basename "$exe")
                if [ ! -f "$INSTALL_DIR/bin/$basename_exe" ]; then
                    cp "$exe" "$INSTALL_DIR/bin/" 2>/dev/null || true
                fi
            fi
        done
    
    echo -e "${GREEN}✓ CDP built successfully${NC}"
}

# Create environment script
create_env_script() {
    cat > "$INSTALL_DIR/env.sh" << EOF
#!/bin/bash
# CDP environment for cdp-rs
export CDP_PATH="$INSTALL_DIR/bin"
export PATH="\$CDP_PATH:\$PATH"
echo "CDP environment set: \$CDP_PATH"
EOF
    chmod +x "$INSTALL_DIR/env.sh"
}

# Verify build
verify_build() {
    echo ""
    echo "Verifying CDP build..."
    
    # Count binaries
    if [ -d "$INSTALL_DIR/bin" ]; then
        COUNT=$(ls -1 "$INSTALL_DIR/bin" 2>/dev/null | wc -l)
        echo -e "${GREEN}✓ Built $COUNT CDP programs${NC}"
        
        # Check for key programs
        for prog in pvoc blur pitch stretch; do
            if [ -f "$INSTALL_DIR/bin/$prog" ]; then
                echo -e "${GREEN}✓ Found $prog${NC}"
            else
                echo -e "${YELLOW}⚠ Missing $prog${NC}"
            fi
        done
        
        # List first few programs
        echo ""
        echo "Sample programs:"
        ls -1 "$INSTALL_DIR/bin" 2>/dev/null | head -10
    else
        echo -e "${RED}✗ No binaries found${NC}"
        return 1
    fi
    
    return 0
}

# Main
main() {
    echo "======================================"
    echo "Building CDP from source"
    echo "======================================"
    echo ""
    
    setup_directories
    
    if ! check_dependencies; then
        exit 1
    fi
    
    get_source
    apply_patches
    build_cdp
    create_env_script
    
    if verify_build; then
        echo ""
        echo -e "${GREEN}Build complete!${NC}"
        echo ""
        echo "CDP binaries are in: $INSTALL_DIR/bin"
        echo ""
        echo "To use CDP:"
        echo "  source $INSTALL_DIR/env.sh"
        echo ""
        echo "Or set CDP_PATH:"
        echo "  export CDP_PATH=$INSTALL_DIR/bin"
    else
        echo ""
        echo -e "${RED}Build may have issues, please check manually${NC}"
        exit 1
    fi
}

main "$@"