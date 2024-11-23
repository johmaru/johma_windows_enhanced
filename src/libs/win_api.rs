use std::ffi::OsString;
use std::{path::PathBuf, ptr};
use winapi::um::shlobj::{SHGetFolderPathW, CSIDL_LOCAL_APPDATA};
use winapi::um::winnt::PWSTR;

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
