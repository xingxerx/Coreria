@echo off
echo ╔══════════════════════════════════════════════════════════════╗
echo ║           Epoch of Elria - Windowed Games Builder           ║
echo ╚══════════════════════════════════════════════════════════════╝
echo.

echo Checking for C++ compiler...
where g++ >nul 2>&1
if %errorlevel% == 0 (
    echo ✓ g++ found! Building windowed games...
    goto :build_windowed
)

where cl >nul 2>&1
if %errorlevel% == 0 (
    echo ✓ Visual Studio compiler found! Building windowed games...
    goto :build_windowed_msvc
)

echo ❌ No C++ compiler found!
echo Please install MinGW-w64 or Visual Studio
pause
exit /b 1

:build_windowed
echo.
echo Building windowed games with g++...
echo ═══════════════════════════════════════

cd /d "%~dp0src\epoch_of_elria"

echo Building windowed game engine...
g++ -std=c++17 -O2 windowed_game_engine.cpp -o windowed_game_engine.exe -lgdi32 -luser32 2>nul
if %errorlevel% neq 0 (
    g++ -std=c++11 windowed_game_engine.cpp -o windowed_game_engine.exe -lgdi32 -luser32
)

echo Building windowed app engine...
g++ -std=c++17 -O2 windowed_app_engine.cpp -o windowed_app_engine.exe -lgdi32 -luser32 2>nul
if %errorlevel% neq 0 (
    g++ -std=c++11 windowed_app_engine.cpp -o windowed_app_engine.exe -lgdi32 -luser32
)

echo Building simple windowed engine...
g++ -std=c++17 -O2 simple_windowed_engine.cpp -o simple_windowed_engine.exe -lgdi32 -luser32 2>nul
if %errorlevel% neq 0 (
    g++ -std=c++11 simple_windowed_engine.cpp -o simple_windowed_engine.exe -lgdi32 -luser32
)

echo Building windowed sandbox game...
g++ -std=c++17 -O2 windowed_sandbox_game.cpp -o windowed_sandbox_game.exe -lgdi32 -luser32 2>nul
if %errorlevel% neq 0 (
    g++ -std=c++11 windowed_sandbox_game.cpp -o windowed_sandbox_game.exe -lgdi32 -luser32
)

echo Building create windowed app...
g++ -std=c++17 -O2 create_windowed_app.cpp -o create_windowed_app.exe -lgdi32 -luser32 2>nul
if %errorlevel% neq 0 (
    g++ -std=c++11 create_windowed_app.cpp -o create_windowed_app.exe -lgdi32 -luser32
)

goto :show_results

:build_windowed_msvc
echo.
echo Building windowed games with Visual Studio...
echo ═══════════════════════════════════════════════

cd /d "%~dp0src\epoch_of_elria"

echo Building windowed game engine...
cl /EHsc windowed_game_engine.cpp /Fe:windowed_game_engine.exe user32.lib gdi32.lib >nul 2>&1

echo Building windowed app engine...
cl /EHsc windowed_app_engine.cpp /Fe:windowed_app_engine.exe user32.lib gdi32.lib >nul 2>&1

echo Building simple windowed engine...
cl /EHsc simple_windowed_engine.cpp /Fe:simple_windowed_engine.exe user32.lib gdi32.lib >nul 2>&1

echo Building windowed sandbox game...
cl /EHsc windowed_sandbox_game.cpp /Fe:windowed_sandbox_game.exe user32.lib gdi32.lib >nul 2>&1

echo Building create windowed app...
cl /EHsc create_windowed_app.cpp /Fe:create_windowed_app.exe user32.lib gdi32.lib >nul 2>&1

goto :show_results

:show_results
echo.
echo ✓ Build complete!
echo.
echo Available windowed games:
echo ═══════════════════════════

if exist windowed_game_engine.exe (
    echo ✓ windowed_game_engine.exe - Main windowed game engine
)
if exist windowed_app_engine.exe (
    echo ✓ windowed_app_engine.exe - Windowed application engine
)
if exist simple_windowed_engine.exe (
    echo ✓ simple_windowed_engine.exe - Simple windowed engine
)
if exist windowed_sandbox_game.exe (
    echo ✓ windowed_sandbox_game.exe - Windowed sandbox game
)
if exist create_windowed_app.exe (
    echo ✓ create_windowed_app.exe - Create windowed application
)
if exist game_engine_parallel_demo.exe (
    echo ✓ game_engine_parallel_demo.exe - Parallel demo (already built)
)

echo.
echo Choose which windowed game to run:
echo 1. Windowed Game Engine
echo 2. Windowed App Engine  
echo 3. Simple Windowed Engine
echo 4. Windowed Sandbox Game
echo 5. Create Windowed App
echo 6. Parallel Demo (already running)
echo 7. Exit
echo.
set /p choice="Enter your choice (1-7): "

if "%choice%"=="1" (
    if exist windowed_game_engine.exe (
        echo Starting Windowed Game Engine...
        start windowed_game_engine.exe
    ) else (
        echo ❌ windowed_game_engine.exe not found!
    )
)

if "%choice%"=="2" (
    if exist windowed_app_engine.exe (
        echo Starting Windowed App Engine...
        start windowed_app_engine.exe
    ) else (
        echo ❌ windowed_app_engine.exe not found!
    )
)

if "%choice%"=="3" (
    if exist simple_windowed_engine.exe (
        echo Starting Simple Windowed Engine...
        start simple_windowed_engine.exe
    ) else (
        echo ❌ simple_windowed_engine.exe not found!
    )
)

if "%choice%"=="4" (
    if exist windowed_sandbox_game.exe (
        echo Starting Windowed Sandbox Game...
        start windowed_sandbox_game.exe
    ) else (
        echo ❌ windowed_sandbox_game.exe not found!
    )
)

if "%choice%"=="5" (
    if exist create_windowed_app.exe (
        echo Starting Create Windowed App...
        start create_windowed_app.exe
    ) else (
        echo ❌ create_windowed_app.exe not found!
    )
)

if "%choice%"=="6" (
    echo The parallel demo is already running in a separate window.
)

if "%choice%"=="7" (
    echo Goodbye!
    exit /b 0
)

echo.
echo Press any key to exit...
pause >nul
