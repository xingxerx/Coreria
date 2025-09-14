@echo off
echo 🌟 Starting Coreria Everything TM Game Engine
echo 🖥️  Setting up optimal display environment...

REM Force X11 backend for maximum compatibility (Linux/WSL)
set WINIT_UNIX_BACKEND=x11
set GDK_BACKEND=x11
set QT_QPA_PLATFORM=xcb

REM Set display for WSL/Linux environments
if not defined DISPLAY set DISPLAY=:0

echo ✅ Environment configured
echo 🚀 Launching game...

REM Navigate to the game directory and run
cd /d "%~dp0epoch-of-elria"
cargo run --release

echo 🌟 Game session ended. Thanks for playing!
pause
