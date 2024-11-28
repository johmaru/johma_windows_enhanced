use open;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::ptr;
use std::ptr::null_mut;
use winapi::shared::lmcons::NET_API_STATUS;
use winapi::shared::minwindef::{DWORD, HINSTANCE, LPBYTE};
use winapi::shared::ntdef::PWSTR;
use winapi::shared::sddl::ConvertSidToStringSidW;
use winapi::um::handleapi::CloseHandle;
use winapi::um::knownfolders::FOLDERID_LocalAppDataLow;
use winapi::um::libloaderapi::GetModuleFileNameW;
use winapi::um::lmaccess::{NetUserEnum, USER_INFO_0};
use winapi::um::lmapibuf::NetApiBufferFree;
use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcess, OpenProcessToken};
use winapi::um::shlobj::{
    SHGetFolderPathW, SHGetKnownFolderPath, CSIDL_APPDATA, CSIDL_LOCAL_APPDATA,
};
use winapi::um::winbase::LocalFree;
use winapi::um::winbase::LookupAccountNameW;
use winapi::um::winnt::{
    TokenUser, HANDLE, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ, PSID, SID_NAME_USE, TOKEN_QUERY,
};
use windows::Win32::System::Com::CoTaskMemFree;

use crate::libs::logger_control;
use crate::VERISON;

#[link(name = "win_sys_api", kind = "static")]
extern "C" {
    fn restart_explorer();
    fn list_all_pids() -> *mut DWORD;
    fn free_pids(pids: *mut DWORD);
}

pub fn get_all_user_sids() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    unsafe {
        let mut user_info: *mut USER_INFO_0 = ptr::null_mut();
        let mut entries_read: DWORD = 0;
        let mut total_entries: DWORD = 0;
        let mut resume_handle: DWORD = 0;
        const NERR_Success: NET_API_STATUS = 0;

        let status: NET_API_STATUS = NetUserEnum(
            ptr::null_mut(),
            0,
            0,
            &mut user_info as *mut _ as *mut LPBYTE,
            DWORD::MAX,
            &mut entries_read,
            &mut total_entries,
            &mut resume_handle,
        );

        if status != NERR_Success {
            return Err("Failed to enumerate users".into());
        }

        let mut sids = Vec::new();
        for i in 0..entries_read {
            let user_info_ptr = user_info.offset(i as isize);
            let username = (*user_info_ptr).usri0_name;
            let username_len = (0..).take_while(|&i| *username.offset(i) != 0).count();
            let username_slice = std::slice::from_raw_parts(username, username_len);
            let username_os = OsString::from_wide(username_slice);

            let mut sid: PSID = ptr::null_mut();
            let mut sid_size: DWORD = 0;
            let mut domain_name: [u16; 256] = [0; 256];
            let mut domain_name_size: DWORD = domain_name.len() as DWORD;
            let mut sid_name_use: SID_NAME_USE = 0;

            LookupAccountNameW(
                ptr::null(),
                username,
                sid,
                &mut sid_size,
                domain_name.as_mut_ptr(),
                &mut domain_name_size,
                &mut sid_name_use,
            );

            let mut sid_buffer: Vec<u8> = vec![0; sid_size as usize];
            sid = sid_buffer.as_mut_ptr() as PSID;

            if LookupAccountNameW(
                ptr::null(),
                username,
                sid,
                &mut sid_size,
                domain_name.as_mut_ptr(),
                &mut domain_name_size,
                &mut sid_name_use,
            ) != 0
            {
                let mut sid_string: *mut u16 = ptr::null_mut();
                if ConvertSidToStringSidW(sid, &mut sid_string) != 0 {
                    let sid_os_string = OsString::from_wide(std::slice::from_raw_parts(
                        sid_string,
                        sid_size as usize,
                    ));
                    sids.push(sid_os_string.to_string_lossy().into_owned());
                    LocalFree(sid_string as *mut _);
                }
            }
        }

        NetApiBufferFree(user_info as *mut _);
        Ok(sids)
    }
}

pub fn get_local_appdata() -> Option<PathBuf> {
    let mut path: [u16; 260] = [0; 260];

    unsafe {
        let result = SHGetFolderPathW(
            ptr::null_mut(),
            CSIDL_LOCAL_APPDATA,
            ptr::null_mut(),
            0,
            path.as_mut_ptr() as PWSTR,
        );

        if result >= 0 {
            let len = path.iter().take_while(|&&c| c != 0).count();
            let os_str: OsString = std::os::windows::ffi::OsStringExt::from_wide(&path[..len]);
            Some(PathBuf::from(os_str))
        } else {
            None
        }
    }
}

pub fn get_local_low() -> Option<PathBuf> {
    unsafe {
        let mut path_ptr: PWSTR = ptr::null_mut();
        let result =
            SHGetKnownFolderPath(&FOLDERID_LocalAppDataLow, 0, ptr::null_mut(), &mut path_ptr);

        if result >= 0 {
            let len = (0..).take_while(|&i| *path_ptr.add(i) != 0).count();
            let path_slice = std::slice::from_raw_parts(path_ptr, len);
            let os_str: OsString = std::os::windows::ffi::OsStringExt::from_wide(path_slice);
            CoTaskMemFree(Some(path_ptr as *const _));
            Some(PathBuf::from(os_str))
        } else {
            None
        }
    }
}

pub fn get_roaming() -> Option<PathBuf> {
    let mut path: [u16; 260] = [0; 260];

    unsafe {
        let result = SHGetFolderPathW(
            ptr::null_mut(),
            CSIDL_APPDATA,
            ptr::null_mut(),
            0,
            path.as_mut_ptr() as PWSTR,
        );

        if result >= 0 {
            let len = path.iter().take_while(|&&c| c != 0).count();
            let os_str: OsString = std::os::windows::ffi::OsStringExt::from_wide(&path[..len]);
            Some(PathBuf::from(os_str))
        } else {
            None
        }
    }
}

// This function is not used in the current implementation and to begin with make a get user function
pub fn get_local_appdata_with_token(htoken: *mut winapi::ctypes::c_void) -> Option<PathBuf> {
    let mut path: [u16; 260] = [0; 260];

    unsafe {
        let result = SHGetFolderPathW(
            ptr::null_mut(),
            CSIDL_LOCAL_APPDATA,
            htoken,
            0,
            path.as_mut_ptr() as PWSTR,
        );

        if result >= 0 {
            let len = path.iter().take_while(|&&c| c != 0).count();
            let os_str: OsString = std::os::windows::ffi::OsStringExt::from_wide(&path[..len]);
            Some(PathBuf::from(os_str))
        } else {
            None
        }
    }
}

pub fn get_appdata() -> Option<PathBuf> {
    let mut path: [u16; 260] = [0; 260];

    unsafe {
        let result = SHGetFolderPathW(
            ptr::null_mut(),
            CSIDL_APPDATA,
            ptr::null_mut(),
            0,
            path.as_mut_ptr() as PWSTR,
        );

        if result >= 0 {
            let len = path.iter().take_while(|&&c| c != 0).count();
            let os_str: OsString = std::os::windows::ffi::OsStringExt::from_wide(&path[..len]);
            Some(PathBuf::from(os_str))
        } else {
            None
        }
    }
}

pub fn get_app_folder() -> Option<PathBuf> {
    let local_data = dirs::data_local_dir().expect("Failed to get local app data directory");

    let launcher_loc = local_data.join("johma_windows_enhanced");

    return Some(launcher_loc);
}

pub fn open_explorer<P>(path: P) -> Result<(), Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let path = PathBuf::from(path);
    let path = path.canonicalize().unwrap();

    if let Err(e) = open::that(&path) {
        eprintln!("Failed to open explorer: {}", e);
        logger_control::log(
            &format!("Failed to open explorer: {}", e),
            logger_control::LogLevel::CRITICAL,
        );
    }
    logger_control::log(
        &format!("Opened explorer at: {}", path.display()),
        logger_control::LogLevel::INFO,
    );
    Ok(())
}

pub fn open_task_manager() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = open::that("taskmgr") {
        eprintln!("Failed to open task manager: {}", e);
        logger_control::log(
            &format!("Failed to open task manager: {}", e),
            logger_control::LogLevel::CRITICAL,
        );
    }
    logger_control::log("Opened task manager", logger_control::LogLevel::INFO);
    Ok(())
}

pub fn refresh_exprorer() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        restart_explorer();
    }
    logger_control::log("Refreshed explorer", logger_control::LogLevel::INFO);
    Ok(())
}

pub fn show_all_pid() {
    unsafe {
        let pids = list_all_pids();
        if pids.is_null() {
            eprintln!("Failed to get PIDs: list_all_pids returned NULL");
            return;
        }
        free_pids(pids);
    }
}

pub fn open_environment_variables_window() -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        let result = Command::new("SystemPropertiesAdvanced.exe")
            .arg("/c")
            .spawn();

        match result {
            Ok(_) => {
                logger_control::log(
                    "Opened environment variables window",
                    logger_control::LogLevel::INFO,
                );
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to open environment variables window: {}", e);
                logger_control::log(
                    &format!("Failed to open environment variables window: {}", e),
                    logger_control::LogLevel::CRITICAL,
                );
                Err(e.into())
            }
        }
    } else {
        eprintln!("Failed to open environment variables window: Unsupported OS");
        logger_control::log(
            "Failed to open environment variables window: Unsupported OS",
            logger_control::LogLevel::CRITICAL,
        );
        Err("Unsupported OS".into())
    }
}

pub fn kill_pid(pid: u32) -> Result<(), Box<dyn std::error::Error>> {
    let result = Command::new("taskkill")
        .arg("/F")
        .arg("/PID")
        .arg(pid.to_string())
        .spawn();

    match result {
        Ok(_) => {
            logger_control::log(
                &format!("Killed PID: {}", pid),
                logger_control::LogLevel::INFO,
            );
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to kill PID: {}", e);
            logger_control::log(
                &format!("Failed to kill PID: {}", e),
                logger_control::LogLevel::CRITICAL,
            );
            Err(e.into())
        }
    }
}

pub fn run_launcher(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let result = Command::new(path).spawn();

    match result {
        Ok(_) => {
            logger_control::log(
                &format!("Ran launcher: {}", path),
                logger_control::LogLevel::INFO,
            );
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to run launcher: {}", e);
            logger_control::log(
                &format!("Failed to run launcher: {}", e),
                logger_control::LogLevel::CRITICAL,
            );
            Err(e.into())
        }
    }
}
