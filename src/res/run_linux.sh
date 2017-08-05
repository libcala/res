# Step 1: Install

## Install Binary
mkdir -p ~/.local/bin/
cp $1 ~/.local/bin/`basename $1`

## Install Icon & Symbol
mkdir -p ~/.local/share/icons/hicolor/scalable/
mkdir -p ~/.local/share/icons/symbolic/scalable/

cp res/icon.png ~/.local/share/icons/hicolor/scalable/`basename $1`.png
cp res/symbol.png ~/.local/share/icons/symbolic/scalable/`basename $1`.png

# Step 2: Run
`basename $1`
