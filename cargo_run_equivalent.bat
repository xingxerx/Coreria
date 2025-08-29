@echo off
echo ╔══════════════════════════════════════════════════════════════╗
echo ║           CARGO RUN EQUIVALENT - COMPLETE BUILD             ║
echo ║              Epoch of Elria Game Engine                     ║
echo ╚══════════════════════════════════════════════════════════════╝
echo.

echo 🚀 Simulating 'cargo run' - Building complete game demo...
echo.

REM Check for compiler
where g++ >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Error: g++ compiler not found!
    echo Please install MinGW-w64 or Visual Studio
    pause
    exit /b 1
)

echo ✓ Compiler found: g++
echo.

echo 📦 COMPILING COMPONENTS...
echo ═══════════════════════════════════════════════════════════════

REM Build the main Dream Weaver game
echo [1/4] Building Dream Weaver's Heart (main game)...
cd /d "%~dp0src\dream_weavers_heart"
g++ -std=c++17 -Wall -O2 main_dream_weaver_complete.cpp GameObject3D.cpp -o dream_weaver_complete.exe
if %errorlevel% neq 0 (
    echo ❌ Failed to build main game
    pause
    exit /b 1
)
echo ✓ Dream Weaver's Heart compiled successfully

REM Build RPG characters demo
echo [2/4] Building RPG Characters system...
cd /d "%~dp0src\epoch_of_elria"
g++ -std=c++17 -Wall -O2 main_rpg_characters.cpp ../dream_weavers_heart/GameObject3D.cpp -I../dream_weavers_heart -o rpg_characters.exe
if %errorlevel% neq 0 (
    echo ⚠️  RPG Characters build failed, continuing...
) else (
    echo ✓ RPG Characters compiled successfully
)

REM Build 3D world demo
echo [3/4] Building 3D Open World...
g++ -std=c++17 -Wall -O2 main_3d_openworld.cpp ../dream_weavers_heart/GameObject3D.cpp -I../dream_weavers_heart -o game_3d_openworld.exe
if %errorlevel% neq 0 (
    echo ⚠️  3D Open World build failed, continuing...
) else (
    echo ✓ 3D Open World compiled successfully
)

REM Build windowed sandbox game
echo [4/4] Building Windowed Sandbox...
g++ -std=c++17 -Wall -O2 windowed_sandbox_game.cpp -lgdi32 -luser32 -o windowed_sandbox_game.exe
if %errorlevel% neq 0 (
    echo ⚠️  Windowed Sandbox build failed, continuing...
) else (
    echo ✓ Windowed Sandbox compiled successfully
)

echo.
echo 🎯 BUILD COMPLETE!
echo ═══════════════════════════════════════════════════════════════

cd /d "%~dp0"

REM Check what was built successfully
set /a built_count=0
if exist "src\dream_weavers_heart\dream_weaver_complete.exe" (
    echo ✅ dream_weaver_complete.exe - Complete Dream Weaver experience
    set /a built_count+=1
)
if exist "src\epoch_of_elria\rpg_characters.exe" (
    echo ✅ rpg_characters.exe - RPG character system demo
    set /a built_count+=1
)
if exist "src\epoch_of_elria\game_3d_openworld.exe" (
    echo ✅ game_3d_openworld.exe - 3D world exploration
    set /a built_count+=1
)
if exist "src\epoch_of_elria\windowed_sandbox_game.exe" (
    echo ✅ windowed_sandbox_game.exe - Interactive windowed game
    set /a built_count+=1
)

echo.
echo 📊 SUMMARY: %built_count% components built successfully
echo.

if %built_count% equ 0 (
    echo ❌ No components built successfully!
    echo Check the error messages above for details.
    pause
    exit /b 1
)

echo 🎮 RUNNING COMPLETE DEMO...
echo ═══════════════════════════════════════════════════════════════
echo.
echo Choose which component to run:
echo.
if exist "src\dream_weavers_heart\dream_weaver_complete.exe" (
    echo 1. 🌟 Dream Weaver's Heart ^(Complete Experience^) - RECOMMENDED
)
if exist "src\epoch_of_elria\rpg_characters.exe" (
    echo 2. 👥 RPG Characters Demo
)
if exist "src\epoch_of_elria\game_3d_openworld.exe" (
    echo 3. 🌍 3D Open World Exploration
)
if exist "src\epoch_of_elria\windowed_sandbox_game.exe" (
    echo 4. 🎨 Windowed Sandbox Game
)
echo 5. 🔄 Run All Components Sequentially
echo 6. ❌ Exit
echo.
set /p choice="Enter your choice (1-6): "

if "%choice%"=="1" (
    if exist "src\dream_weavers_heart\dream_weaver_complete.exe" (
        echo.
        echo 🌟 Starting The Dream Weaver's Heart...
        echo ═══════════════════════════════════════════════════════════════
        echo This is the complete narrative experience!
        echo.
        cd /d "%~dp0src\dream_weavers_heart"
        dream_weaver_complete.exe
        echo.
        echo 🌟 Dream Weaver's Heart session completed!
    ) else (
        echo ❌ Dream Weaver's Heart not available
    )
)

if "%choice%"=="2" (
    if exist "src\epoch_of_elria\rpg_characters.exe" (
        echo.
        echo 👥 Starting RPG Characters Demo...
        echo ═══════════════════════════════════════════════════════════════
        cd /d "%~dp0src\epoch_of_elria"
        rpg_characters.exe
        echo.
        echo 👥 RPG Characters demo completed!
    ) else (
        echo ❌ RPG Characters demo not available
    )
)

if "%choice%"=="3" (
    if exist "src\epoch_of_elria\game_3d_openworld.exe" (
        echo.
        echo 🌍 Starting 3D Open World Exploration...
        echo ═══════════════════════════════════════════════════════════════
        cd /d "%~dp0src\epoch_of_elria"
        game_3d_openworld.exe
        echo.
        echo 🌍 3D Open World session completed!
    ) else (
        echo ❌ 3D Open World not available
    )
)

if "%choice%"=="4" (
    if exist "src\epoch_of_elria\windowed_sandbox_game.exe" (
        echo.
        echo 🎨 Starting Windowed Sandbox Game...
        echo ═══════════════════════════════════════════════════════════════
        cd /d "%~dp0src\epoch_of_elria"
        start windowed_sandbox_game.exe
        echo.
        echo 🎨 Windowed Sandbox Game launched in separate window!
    ) else (
        echo ❌ Windowed Sandbox Game not available
    )
)

if "%choice%"=="5" (
    echo.
    echo 🔄 Running All Components Sequentially...
    echo ═══════════════════════════════════════════════════════════════
    
    if exist "src\dream_weavers_heart\dream_weaver_complete.exe" (
        echo.
        echo 🌟 [1/4] Dream Weaver's Heart
        echo Press any key to start, or Ctrl+C to skip...
        pause >nul
        cd /d "%~dp0src\dream_weavers_heart"
        dream_weaver_complete.exe
    )
    
    if exist "src\epoch_of_elria\rpg_characters.exe" (
        echo.
        echo 👥 [2/4] RPG Characters Demo
        echo Press any key to start, or Ctrl+C to skip...
        pause >nul
        cd /d "%~dp0src\epoch_of_elria"
        rpg_characters.exe
    )
    
    if exist "src\epoch_of_elria\game_3d_openworld.exe" (
        echo.
        echo 🌍 [3/4] 3D Open World
        echo Press any key to start, or Ctrl+C to skip...
        pause >nul
        cd /d "%~dp0src\epoch_of_elria"
        game_3d_openworld.exe
    )
    
    if exist "src\epoch_of_elria\windowed_sandbox_game.exe" (
        echo.
        echo 🎨 [4/4] Windowed Sandbox ^(will open in separate window^)
        echo Press any key to start, or Ctrl+C to skip...
        pause >nul
        cd /d "%~dp0src\epoch_of_elria"
        start windowed_sandbox_game.exe
        echo Windowed game launched! Check your taskbar.
    )
    
    echo.
    echo 🔄 All available components have been run!
)

if "%choice%"=="6" (
    echo.
    echo 👋 Thanks for using Epoch of Elria Game Engine!
    echo.
    echo 📈 Build Summary:
    echo   - Components built: %built_count%
    echo   - Main game: Dream Weaver's Heart
    echo   - Engine: C++ with custom game objects
    echo   - Platform: Windows with MinGW
    echo.
    echo 🚀 To run again, execute: cargo_run_equivalent.bat
    echo.
    exit /b 0
)

echo.
echo 🎮 Game session completed!
echo.
echo 🔄 Run again? (Y/N)
set /p restart="Enter choice: "
if /i "%restart%"=="Y" (
    echo.
    goto :eof
    REM This would restart the script, but we'll exit for now
)

echo.
echo 👋 Thanks for playing Epoch of Elria!
echo.
pause
