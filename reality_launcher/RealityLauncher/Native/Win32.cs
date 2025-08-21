using System;
using System.Runtime.InteropServices;

namespace RealityLauncher.Native;

public enum ThreadAccess
{
    TERMINATE = 1,
    SUSPEND_RESUME = 2,
    GET_CONTEXT = 8,
    SET_CONTEXT = 0x10,
    SET_INFORMATION = 0x20,
    QUERY_INFORMATION = 0x40,
    SET_THREAD_TOKEN = 0x80,
    IMPERSONATE = 0x100,
    DIRECT_IMPERSONATION = 0x200
}

public class Win32
{
    [DllImport("kernel32.dll")]
    public static extern IntPtr CreateRemoteThread(IntPtr hProcess, IntPtr lpThreadAttributes, uint dwStackSize, UIntPtr lpStartAddress, IntPtr lpParameter, uint dwCreationFlags, out IntPtr lpThreadId);

    [DllImport("kernel32.dll")]
    public static extern IntPtr OpenProcess(UInt32 dwDesiredAccess, Int32 bInheritHandle, Int32 dwProcessId);

    [DllImport("kernel32.dll")]
    public static extern Int32 CloseHandle(IntPtr hObject);

    [DllImport("kernel32.dll")]
    static extern bool VirtualFreeEx(IntPtr hProcess, IntPtr lpAddress, UIntPtr dwSize, uint dwFreeType);

    [DllImport("kernel32.dll")]
    public static extern UIntPtr GetProcAddress(IntPtr hModule, string procName);
    
    [DllImport("kernel32.dll")]
    public static extern IntPtr VirtualAllocEx(IntPtr hProcess, IntPtr lpAddress, uint dwSize, uint flAllocaationType, uint flProtect);
    
    [DllImport("kernel32.dll")]
    public static extern int WriteProcessMemory(IntPtr hProcess, IntPtr lpBaseAddress, byte[] lpBuffer, uint nSize, int lpNumberOfBytesWritten);

    [DllImport("kernel32.dll")]
    public static extern IntPtr GetModuleHandle(string lpModuleName);

    [DllImport("kernel32.dll")]
    public static extern Int32 WaitForSingleObject(IntPtr handle, Int32 milliseconds);
    
    [DllImport("kernel32.dll")]
    public static extern IntPtr OpenThread(ThreadAccess dwDesiredAccess, bool bInheritHandle, int dwThreadId);

    [DllImport("kernel32.dll")]
    public static extern uint SuspendThread(IntPtr hThread);

    [DllImport("kernel32.dll")]
    public static extern int ResumeThread(IntPtr hThread);

}