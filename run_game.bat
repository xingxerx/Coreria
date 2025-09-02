@echo off
echo 🌟 Epoch of Elria - Self-Improving Game Engine 🌟
echo ==================================================
echo.

echo 🖥️  Windows environment detected
echo 🎮 Running full graphics mode...
echo.

cargo run --bin epoch_of_elria

if %ERRORLEVEL% neq 0 (
    echo.
    echo ❌ Graphics mode failed
    echo 🎯 Running text demo instead...
    echo.
    cargo run --bin text_demo
)

echo.
echo 🌟 Thanks for playing Epoch of Elria! 🌟
echo 📖 See README_GRAPHICS_SETUP.md for graphics setup instructions
pause
