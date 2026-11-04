"""Load/unload the built DLL on the Windows runner; this is NOT an AE test."""
import ctypes
import sys
from pathlib import Path
k = ctypes.WinDLL('kernel32', use_last_error=True)
k.LoadLibraryW.argtypes = [ctypes.c_wchar_p]
k.LoadLibraryW.restype = ctypes.c_void_p
k.FreeLibrary.argtypes = [ctypes.c_void_p]
k.FreeLibrary.restype = ctypes.c_int
handle = k.LoadLibraryW(str(Path(sys.argv[1]).resolve()))
if not handle: raise ctypes.WinError(ctypes.get_last_error())
if not k.FreeLibrary(handle): raise ctypes.WinError(ctypes.get_last_error())
print('PASS: Windows LoadLibrary/FreeLibrary. AE registration/render not tested.')
