#!/bin/bash

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║                 Epoch of Elria - Build Tool                   ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo

function menu() {
    echo "Choose a project to build and run:"
    echo
    echo "[1] Epoch of Elria (Rust Game Engine)"
    echo "[2] The Dream Weaver's Heart (C++ Demos)"
    echo "[3] Windowed Games (C++ Demos)"
    echo "[4] Exit"
    echo
    read -p "Enter your choice: " choice

    case $choice in
        1) rust_game ;;
        2) dream_weavers_heart ;;
        3) windowed_games ;;
        4) exit_script ;;
        *) echo "Invalid choice." ; menu ;;
    esac
}

function rust_game() {
    echo
    echo "Building and running Epoch of Elria (Rust Game Engine)..."
    echo
    cargo run --bin epoch_of_elria
    if [ $? -ne 0 ]; then
        echo
        echo "❌ Graphics mode failed"
        echo "🎯 Running text demo instead..."
        echo
        cargo run --bin text_demo
    fi
    end_script
}

function dream_weavers_heart() {
    echo
    echo "Building and running The Dream Weaver's Heart (C++ Demos)..."
    echo
    cd cpp_src
    g++ -std=c++17 -Wall -Wextra -O2 main_dream_weaver_complete.cpp GameObject3D.cpp -o dream_weaver_complete
    if [ $? -ne 0 ]; then
        echo "❌ Build failed!"
        end_script
    fi
    ./dream_weaver_complete
    end_script
}

function windowed_games() {
    echo
    echo "Building and running Windowed Games (C++ Demos)..."
    echo
    cd cpp_src
    g++ -std=c++17 -O2 windowed_game_engine.cpp -o windowed_game_engine
    if [ $? -ne 0 ]; then
        echo "❌ Build failed!"
        end_script
    fi
    ./windowed_game_engine
    end_script
}

function exit_script() {
    echo
    echo "Goodbye!"
    exit 0
}

function end_script() {
    echo
    echo "Press any key to return to the menu..."
    read -n 1
    menu
}

menu
