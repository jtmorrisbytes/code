$TOOLCHAIN_DIR="C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.44.35207"
$PROJECT_ROOT = $PSScriptRoot;

$HOST_ARCH = (Get-CimInstance Win32_OperatingSystem).OSArchitecture

if ($HOST_ARCH -ceq "64-bit") {
    $HOST_ARCH = "Hostx64"
}
else {
    $HOST_ARCH = "Hostx86"
}

Write-Output $HOST_ARCH

$COMPILER_PATH = "$TOOLCHAIN_DIR\bin\$HOST_ARCH\x64\cl.exe"
& $COMPILER_PATH /I "$PROJECT_ROOT\include" /I "$TOOLCHAIN_DIR/include" /I "C:\Program Files\Microsoft SDKs\Windows\v7.1\Include" /I "C:\Program Files (x86)\Windows Kits\10\Include\10.0.22000.0\ucrt" /Wall /MT "game.c" /link /LIBPATH:"$PROJECT_ROOT/lib" /LIBPATH:"$TOOLCHAIN_DIR\lib\x64" /LIBPATH:"$TOOLCHAIN_DIR\lib\onecore\x64" /LIBPATH:"C:\Program Files\Microsoft SDKs\Windows\v7.1\Lib\x64" /LIBPATH:"C:\Program Files (x86)\Windows Kits\10\Lib\10.0.22000.0\ucrt\x64" "glfw3_mt.lib" "gdi32.lib" "shell32.lib" "user32.lib"