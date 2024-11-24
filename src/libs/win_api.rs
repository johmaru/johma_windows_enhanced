use open;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::{Path, PathBuf};
use std::ptr;
use winapi::shared::lmcons::NET_API_STATUS;
use winapi::shared::minwindef::{DWORD, LPBYTE};
use winapi::shared::ntdef::NULL;
use winapi::shared::ntdef::PWSTR;
use winapi::shared::sddl::ConvertSidToStringSidW;
use winapi::um::lmaccess::{NetUserEnum, USER_INFO_0};
use winapi::um::lmapibuf::NetApiBufferFree;
use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::shlobj::{SHGetFolderPathW, CSIDL_APPDATA, CSIDL_LOCAL_APPDATA};
use winapi::um::winbase::LocalFree;
use winapi::um::winbase::LookupAccountNameW;
use winapi::um::winbase::LookupAccountSidW;
use winapi::um::winnt::{TokenUser, HANDLE, PSID, SID_NAME_USE, TOKEN_QUERY};

use crate::libs::logger_control;

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
