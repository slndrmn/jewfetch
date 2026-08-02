#!/usr/bin/env bash
set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "Select Installation Option"
echo "1 - Install by building"
echo "2 - Uninstall jewfetch"
echo -n "> "
read -r main

install_rust() {
    echo -e "${YELLOW}Rust toolchain not found.${NC}"
    echo -n "Install rustup now? [Y/n] "
    read -r rust_choice

    if [[ "$rust_choice" =~ ^[Nn]$ ]]; then
        echo -e "${RED}Aborting: Rust is required to build jewfetch.${NC}"
        exit 1
    fi

    echo "Installing rustup..."
    if command -v curl &> /dev/null; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    elif command -v wget &> /dev/null; then
        wget -qO- https://sh.rustup.rs | sh -s -- -y
    else
        echo -e "${RED}Error: curl or wget is required to install Rust.${NC}"
        exit 1
    fi

    # Source cargo env for this session
    source "$HOME/.cargo/env"
}

if [ "$main" == "1" ]; then
    # Check that cargo available on system
    if ! command -v cargo &> /dev/null; then
        install_rust
    fi

    # Double check if still theres no cargo
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}Error: cargo still not found after rustup install.${NC}"
        echo "Try restarting your terminal or running: source ~/.cargo/env"
        exit 1
    fi

    echo "Building jewfetch..."
    cargo build --release

    echo "Installing binary..."
    mkdir -p ~/.local/bin
    cp target/release/jewfetch ~/.local/bin/

    echo "Installing config files..."
    mkdir -p ~/.config/jewfetch
    cp -r src/files/* ~/.config/jewfetch/

    echo -e "${GREEN}Installation complete!${NC}"
    echo "Binary: ~/.local/bin/jewfetch"
    echo "Config: ~/.config/jewfetch"
    echo ""
    echo "Make sure ~/.local/bin is in your PATH."
    echo "Run 'jewfetch' to test."

    # Run if in path if not then give hint
    if command -v jewfetch &> /dev/null; then
        jewfetch
    else
        echo ""
        echo "jewfetch not in PATH yet. Run: ~/.local/bin/jewfetch"
    fi

elif [ "$main" == "2" ]; then
    echo "Uninstalling..."
    rm -f ~/.local/bin/jewfetch
    rm -rf ~/.config/jewfetch
    echo -e "${GREEN}Uninstalled.${NC}"

else
    echo -e "${RED}Error: invalid option '$main'. Choose 1 or 2.${NC}"
    exit 1
fi
