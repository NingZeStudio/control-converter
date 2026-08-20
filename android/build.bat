@echo off
rem ============================================================
rem  Build ControlConverter JNI test APK from the command line.
rem  Uses the Android Studio bundled JBR (JDK) and the cached
rem  Gradle distribution. Edit the paths below to match your setup.
rem ============================================================
setlocal

rem --- Android Studio JBR (JDK) ---
set "JBR=D:\app\as\jbr"
if not exist "%JBR%\bin\java.exe" (
    echo [ERROR] JBR not found at %JBR%. Edit build.bat.
    exit /b 1
)
set "JAVA_HOME=%JBR%"

rem --- Android SDK ---
set "ANDROID_HOME=C:\Users\HASEE_Z7\AppData\Local\Android\Sdk"
if not exist "%ANDROID_HOME%\platform-tools\adb.exe" (
    echo [ERROR] Android SDK not found at %ANDROID_HOME%. Edit build.bat.
    exit /b 1
)
set "ANDROID_SDK_ROOT=%ANDROID_HOME%"

rem --- Gradle (cached distribution) ---
set "GRADLE_BAT=C:\Users\HASEE_Z7\.gradle\wrapper\dists\gradle-9.4.1-bin\arn2x92ynaizyzdaamcbpbhtj\gradle-9.4.1\bin\gradle.bat"
if not exist "%GRADLE_BAT%" (
    echo [ERROR] Gradle not found. Open the project once in Android Studio
    echo        to let it provision the wrapper, or edit build.bat.
    exit /b 1
)

rem --- Run from the project directory (avoid -p trailing-backslash issue) ---
cd /d "%~dp0"

echo [1/1] Compiling APK (assembleDebug)...
call "%GRADLE_BAT%" assembleDebug --console=plain --no-daemon
if errorlevel 1 (
    echo [ERROR] Build failed.
    exit /b 1
)

set "APK=%~dp0app\build\outputs\apk\debug\app-debug.apk"
if exist "%APK%" (
    echo.
    echo [OK] Build succeeded:
    echo   %APK%
) else (
    echo [WARN] APK not found at expected path: %APK%
)
endlocal
