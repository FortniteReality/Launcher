using System;
using System.Diagnostics;
using System.IO;
using System.Text;
using RealityLauncher.Native;

namespace RealityLauncher;

public static class Program
{
    private static bool Inject(int processId, string dllPath)
    {
        IntPtr hProcess = Win32.OpenProcess(0x1F0FFF, 0, processId);
        if (hProcess == IntPtr.Zero) return false;

        byte[] bytes = Encoding.UTF8.GetBytes(dllPath);
        IntPtr lpAddress = Win32.VirtualAllocEx(hProcess, IntPtr.Zero, (uint)bytes.Length, 0x1000, 0x40);
        if (lpAddress == IntPtr.Zero) return false;

        if (Win32.WriteProcessMemory(hProcess, lpAddress, bytes, (uint)bytes.Length, 0) == 0)
            return false;

        UIntPtr loadLibrary = Win32.GetProcAddress(Win32.GetModuleHandle("kernel32.dll"), "LoadLibraryA");
        IntPtr hThread = Win32.CreateRemoteThread(hProcess, IntPtr.Zero, 0, loadLibrary, lpAddress, 0, out _);

        Win32.CloseHandle(hProcess);
        return hThread != IntPtr.Zero;
    }

    private static int PreInject(int processId, string dllPath)
    {
        if (!File.Exists(dllPath)) return 0;   // DLL not found
        if (processId == 0) return 1;          // Invalid process
        if (!Inject(processId, dllPath)) return 2;
        return 3;                              // Success
    }

    public static int Main(string[] args)
    {
        const string fortnite = "FortniteClient-Win64-Shipping.exe";
        const string easyAntiCheat = "FortniteClient-Win64-Shipping_EAC.exe";
        const string fortniteLauncher = "FortniteLauncher.exe";

        if (!File.Exists(fortnite)) return -1;
        if (!File.Exists(easyAntiCheat)) return -2;
        if (!File.Exists(fortniteLauncher)) return -3;

        // resolve DLL path: exe dir + Reality/Equinox.dll
        string exeDir = Path.GetDirectoryName(Environment.ProcessPath)!;
        string dllPath = Path.Combine(exeDir, "Reality", "Equinox.dll");

        using var launcherProcess = Process.Start(new ProcessStartInfo(fortniteLauncher));
        if (launcherProcess != null)
        {
            foreach (ProcessThread thread in launcherProcess.Threads)
                Win32.SuspendThread(Win32.OpenThread(ThreadAccess.SUSPEND_RESUME, false, thread.Id));
        }

        using var eacProcess = Process.Start(new ProcessStartInfo(easyAntiCheat)
        {
            Arguments = "-epicapp=Fortnite -epicenv=Prod -epiclocale=en-us -epicportal -noeac -fromfl=be -fltoken=none -skippatchcheck"
        });
        if (eacProcess != null)
        {
            foreach (ProcessThread thread in eacProcess.Threads)
                Win32.SuspendThread(Win32.OpenThread(ThreadAccess.SUSPEND_RESUME, false, thread.Id));
        }

        using var fortniteProcess = new Process
        {
            StartInfo = new ProcessStartInfo(fortnite, string.Join(" ", args))
            {
                UseShellExecute = false,
                CreateNoWindow = true
            }
        };
        fortniteProcess.Start();
        fortniteProcess.WaitForInputIdle();

        int result = PreInject(fortniteProcess.Id, dllPath);
        if (result != 3) return result;

        fortniteProcess.WaitForExit();
        return 0; // success
    }
}
