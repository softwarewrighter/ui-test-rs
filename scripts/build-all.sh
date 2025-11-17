#!/usr/bin/env bash
#
# build-all.sh - Build ui-test-rs with all metadata
#
# This script ensures reproducible builds with proper metadata capture.
# Always use this script instead of direct cargo commands.
#

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Change to project root
cd "$(dirname "$0")/.."

info "Building ui-test-rs..."

# Determine build profile (default to release)
PROFILE="${1:-release}"

if [ "$PROFILE" = "dev" ] || [ "$PROFILE" = "debug" ]; then
    info "Building in development mode..."
    cargo build
    BINARY_PATH="./target/debug/ui-test-rs"
elif [ "$PROFILE" = "release" ]; then
    info "Building in release mode..."
    cargo build --release
    BINARY_PATH="./target/release/ui-test-rs"
else
    error "Unknown profile: $PROFILE"
    echo "Usage: $0 [dev|release]"
    exit 1
fi

# Verify the binary was created
if [ ! -f "$BINARY_PATH" ]; then
    error "Build failed - binary not found at $BINARY_PATH"
    exit 1
fi

info "Build successful!"
info "Binary location: $BINARY_PATH"

# Show version information
info "Version information:"
"$BINARY_PATH" --version

exit 0
