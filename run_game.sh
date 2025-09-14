#!/bin/bash

echo "🌟 Starting Coreria Everything TM Game Engine"
echo "🖥️  Setting up optimal display environment..."

# Force X11 backend for maximum compatibility
export WINIT_UNIX_BACKEND=x11
export GDK_BACKEND=x11
export QT_QPA_PLATFORM=xcb

# Disable Wayland to prevent conflicts
unset WAYLAND_DISPLAY

# Set display if not already set
if [ -z "$DISPLAY" ]; then
    export DISPLAY=:0
fi

echo "✅ Environment configured for X11"
echo "🚀 Launching game..."

# Navigate to the game directory and run
cd "$(dirname "$0")/epoch-of-elria"
cargo run --release

echo "🌟 Game session ended. Thanks for playing!"
