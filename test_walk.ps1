Add-Type -AssemblyName System.Windows.Forms,System.Drawing
Add-Type @"
using System;
using System.Runtime.InteropServices;
public class U32 {
  [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
  [DllImport("user32.dll")] public static extern bool SetCursorPos(int x, int y);
  [DllImport("user32.dll")] public static extern void mouse_event(uint f, uint x, uint y, uint d, int e);
  [DllImport("user32.dll")] public static extern void keybd_event(byte vk, byte sc, uint f, UIntPtr e);
  [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
}
"@
Start-Sleep -Milliseconds 2500
$p = Get-Process minicraft | Select-Object -First 1
$h = $p.MainWindowHandle
[U32]::SetForegroundWindow($h) | Out-Null
Start-Sleep -Milliseconds 400
$r = New-Object U32+RECT
[U32]::GetWindowRect($h, [ref]$r) | Out-Null
$cx = [int](($r.L + $r.R) / 2); $cy = [int](($r.T + $r.B) / 2)
$b = [System.Windows.Forms.Screen]::PrimaryScreen.Bounds
function Shot($path) {
  $bmp = New-Object System.Drawing.Bitmap $b.Width, $b.Height
  $g = [System.Drawing.Graphics]::FromImage($bmp)
  $g.CopyFromScreen($b.Location, [System.Drawing.Point]::Empty, $b.Size)
  $bmp.Save($path)
  $g.Dispose(); $bmp.Dispose()
}
$dir = 'C:\Users\arsen\Documents\Qoder\2026-11-08\f60619b1\minicraft'
Shot "$dir\screen5_idle.png"
[U32]::SetCursorPos($cx, $cy) | Out-Null
[U32]::mouse_event(2, 0, 0, 0, 0)
[U32]::mouse_event(4, 0, 0, 0, 0)
Start-Sleep -Milliseconds 400
[U32]::keybd_event(0x57, 0, 0, [UIntPtr]::Zero)
Start-Sleep -Milliseconds 1200
Shot "$dir\screen5_walk.png"
[U32]::keybd_event(0x57, 0, 2, [UIntPtr]::Zero)
Write-Output done