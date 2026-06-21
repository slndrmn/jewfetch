echo "Select Installation Option"
echo "1- Install by building"
echo "2- Install by pre-build binary image"
echo "3- Uninstall jewfetch"
echo -n "> "
read main

if [ "$main" == "1" ]; then
    cargo build --release
    cp target/release/jewfetch ~/.local/bin
    mkdir -p ~/.config/jewfetch
    cd src/files && cp -r * ~/.config/jewfetch
    jewfetch
elif [ "$main" == 2 ]; then
    cp jewfetch ~/.local/bin
    mkdir -p ~/.config/jewfetch
    cd src/files && cp -r * ~/.config/jewfetch
    jewfetch
elif [ "$main" == 3 ]; then
    rm ~/.local/bin/jewfetch
    rm -rf ~/.config/jewfetch
else
    echo "uCwuMCXVYMc"
fi
