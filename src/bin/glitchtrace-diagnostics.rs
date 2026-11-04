#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
use std::{
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};
fn output(program: &str, args: &[&str]) -> String {
    let mut c = Command::new(program);
    c.args(args);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        c.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }
    match c.output() {
        Ok(v) if v.status.success() => String::from_utf8_lossy(&v.stdout)
            .trim()
            .chars()
            .take(6000)
            .collect(),
        Ok(v) => format!(
            "Query failed ({}): {}",
            v.status,
            String::from_utf8_lossy(&v.stderr)
                .chars()
                .take(500)
                .collect::<String>()
        ),
        Err(e) => format!("Query unavailable: {e}"),
    }
}
fn main() {
    let result = collect();
    if let Err(e) = result {
        notify(&format!("Could not save diagnostic report: {e}"));
    }
}
fn collect() -> Result<(), Box<dyn std::error::Error>> {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let os = output("cmd.exe", &["/c", "ver"]);
    let gpu = output(
        "powershell.exe",
        &[
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            "Get-CimInstance Win32_VideoController | Select-Object Name,DriverVersion,AdapterRAM | ConvertTo-Json -Compress",
        ],
    );
    let ae = output(
        "powershell.exe",
        &[
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            "Get-ChildItem -LiteralPath ($env:ProgramFiles + '\\Adobe') -Directory -Filter 'Adobe After Effects*' -ErrorAction SilentlyContinue | ForEach-Object { $f = Join-Path $_.FullName 'Support Files\\AfterFX.exe'; if (Test-Path -LiteralPath $f) { [pscustomobject]@{Name=$_.Name;Version=[Diagnostics.FileVersionInfo]::GetVersionInfo($f).FileVersion} } } | ConvertTo-Json -Compress",
        ],
    );
    let report = format!(
        "GlitchTrace {}\nBuild: {}\nReport UTC Unix time: {stamp}\nOS: {os}\nGPU: {gpu}\nAE installations: {ae}\nRenderer: CPU (GPU is environment information only)\n\nIssue: [describe]\nSteps to reproduce: [describe]\nExpected / actual: [describe]\n\nPaste the effect's Copy Diagnostic Report output here too.\nNo project content, license keys, username, or full file paths are intentionally collected.\nReview the report before sharing. Nothing is uploaded automatically.\n",
        env!("CARGO_PKG_VERSION"),
        env!("GLITCHTRACE_BUILD_SHA")
    );
    let root =
        PathBuf::from(std::env::var_os("LOCALAPPDATA").ok_or("LOCALAPPDATA is unavailable")?)
            .join("GlitchTrace")
            .join("Diagnostics");
    std::fs::create_dir_all(&root)?;
    let path = root.join(format!("GlitchTrace-{stamp}.txt"));
    std::fs::write(&path, &report)?;
    let copied = glitchtrace::clipboard::copy_report(&report).is_ok();
    // Notepad is also an editable fallback when the clipboard is busy.
    Command::new("notepad.exe").arg(&path).spawn()?;
    notify(if copied {
        "Report saved and copied to the clipboard. Review it in Notepad before sharing."
    } else {
        "Report saved and opened in Notepad. Copy it from there before sharing."
    });
    Ok(())
}
fn notify(text: &str) {
    #[cfg(windows)]
    unsafe {
        #[link(name = "user32")]
        unsafe extern "system" {
            fn MessageBoxW(
                owner: *mut std::ffi::c_void,
                text: *const u16,
                title: *const u16,
                flags: u32,
            ) -> i32;
        }
        let t: Vec<u16> = text.encode_utf16().chain(Some(0)).collect();
        let title: Vec<u16> = "GlitchTrace Diagnostics"
            .encode_utf16()
            .chain(Some(0))
            .collect();
        MessageBoxW(std::ptr::null_mut(), t.as_ptr(), title.as_ptr(), 0x40);
    }
    #[cfg(not(windows))]
    eprintln!("{text}");
}
