@echo off
rem ============================================================
rem  Rebuild dist/libcc.so (Android aarch64 JNI library) from the
rem  Go sources using the Android NDK, then refresh the copy used
rem  by the JNI test app.
rem
rem  Requires: Go 1.21+, Android NDK r25+ (edit NDK_ROOT below).
rem ============================================================
setlocal

rem --- NDK root (edit if needed) ---
set "NDK_ROOT=C:\Users\HASEE_Z7\AppData\Local\Android\Sdk\ndk\25.2.9519653"
set "CC=%NDK_ROOT%\toolchains\llvm\prebuilt\windows-x86_64\bin\aarch64-linux-android21-clang.cmd"

if not exist "%CC%" (
    echo [ERROR] NDK clang not found: %CC%
    echo         Edit NDK_ROOT in rebuild_so.bat.
    exit /b 1
)

rem --- Go sources live in ..\go ---
set "GOSRC=%~dp0..\go"
if not exist "%GOSRC%\main.go" (
    echo [ERROR] Go sources not found at %GOSRC%
    exit /b 1
)

echo [1/2] Cross-compiling libcc.so (GOOS=android GOARCH=arm64)...
set "CGO_ENABLED=1"
set "GOOS=android"
set "GOARCH=arm64"
set "CC=%CC%"
pushd "%GOSRC%"
go build -buildmode=c-shared -o "%~dp0..\dist\libcc.so" .
if errorlevel 1 (
    popd
    echo [ERROR] Go build failed.
    exit /b 1
)
popd

echo [2/2] Copying fresh libcc.so into the JNI test app...
copy /Y "%~dp0..\dist\libcc.so" "%~dp0app\src\main\jniLibs\arm64-v8a\libcc.so" >nul

echo.
echo Done. dist\libcc.so and the app copy are rebuilt.
endlocal
