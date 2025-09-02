#!/bin/bash

echo "🌟 Epoch of Elria - Self-Improving Game Engine 🌟"
echo "=================================================="
echo ""

# Check if we're in WSL
if grep -qi microsoft /proc/version 2>/dev/null; then
    echo "🐧 Detected WSL environment"
    echo ""
    
    # Try to set up X11 forwarding
    if [ -z "$DISPLAY" ]; then
        echo "⚠️  No DISPLAY variable set. Setting up X11..."
        export DISPLAY=:0
    fi
    
    echo "🎮 Attempting to run with graphics..."
    echo "💡 If this fails, try the text demo instead!"
    echo ""
    
    # Try to run the main game
    timeout 10s cargo run --bin epoch_of_elria 2>/dev/null
    
    if [ $? -ne 0 ]; then
        echo ""
        echo "❌ Graphics mode failed (expected in WSL without X server)"
        echo "🎯 Running text demo instead..."
        echo ""
        cargo run --bin text_demo
    fi
else
    echo "🖥️  Native Linux environment detected"
    echo "🎮 Running full graphics mode..."
    echo ""
    cargo run --bin epoch_of_elria
fi

echo ""
echo "🌟 Thanks for playing Epoch of Elria! 🌟"
echo "📖 See README_GRAPHICS_SETUP.md for graphics setup instructions"
