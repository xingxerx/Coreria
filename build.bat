@echo off
echo ╔══════════════════════════════════════════════════════════════╗
echo ║                 Epoch of Elria - Build Tool                   ║
echo ╚══════════════════════════════════════════════════════════════╝
echo.

:menu
echo Choose a project to build and run:
echo.
echo [1] Epoch of Elria (Rust Game Engine)
echo [2] The Dream Weaver's Heart (C++ Demos)
echo [3] Windowed Games (C++ Demos)
echo [4] Exit
echo.
set /p choice="Enter your choice: "

if "%choice%"=="1" goto rust_game
if "%choice%"=="2" goto dream_weavers_heart
if "%choice%"=="3" goto windowed_games
if "%choice%"=="4" goto exit

echo Invalid choice.
goto menu

:rust_game
echo.
echo Building and running Epoch of Elria (Rust Game Engine)...
echo.
cargo run --bin epoch_of_elria
if %ERRORLEVEL% neq 0 (
    echo.
    echo ❌ Graphics mode failed
    echo 🎯 Running text demo instead...
    echo.
    cargo run --bin text_demo
)
goto end

:dream_weavers_heart
echo.
echo Building and running The Dream Weaver's Heart (C++ Demos)...
echo.
cd /d "%~dp0cpp_src"
g++ -std=c++17 -Wall -Wextra -O2 main_dream_weaver_complete.cpp GameObject3D.cpp -o dream_weaver_complete.exe
if %errorlevel% neq 0 (
    echo ❌ Build failed!
    goto end
)
dream_weaver_complete.exe
goto end

:windowed_games
echo.
echo Building and running Windowed Games (C++ Demos)...
echo.
cd /d "%~dp0cpp_src"
g++ -std=c++17 -O2 windowed_game_engine.cpp -o windowed_game_engine.exe -lgdi32 -luser32
if %errorlevel% neq 0 (
    echo ❌ Build failed!
    goto end
)
start windowed_game_engine.exe
goto end

:exit
echo.
echo Goodbye!
exit /b 0

:end
echo.
echo Press any key to return to the menu...
pause >nul
goto menu
