// this subproject or submodule for creating a win32 window.
// the purpose of this excercise is to create a common interface
// for creating a window and abstracting away platform dependant windows
// for the purpose of opengl / vulkan / directX / software rendering


// inclue common win32 headers
// we want to support unicode for now
#ifndef UNICODE
#define UNICODE
#endif

#include <windef.h>
#include <winuser.h>