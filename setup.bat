@echo off
echo DNotepadX - Renaissance Text Editor Setup
echo ==========================================
echo.

echo Checking if Rust is installed...
where cargo >nul 2>&1
if %errorlevel% neq 0 (
    echo Rust is not installed on this system.
    echo.
    echo To install Rust:
    echo 1. Visit https://rustup.rs/
    echo 2. Download and run rustup-init.exe
    echo 3. Follow the installation instructions
    echo 4. Restart your command prompt
    echo 5. Run this script again
    echo.
    pause
    exit /b 1
)

echo Rust is installed! Building DNotepadX...
echo.

cargo build --release
if %errorlevel% neq 0 (
    echo Build failed! Please check the error messages above.
    pause
    exit /b 1
)

echo.
echo Build successful! 
echo.
echo To run DNotepadX:
echo   cargo run --release
echo.
echo Or run the executable directly:
echo   target\release\dnotepadx.exe
echo.
pause
