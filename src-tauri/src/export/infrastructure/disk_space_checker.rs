/// Disk space checker utility
use std::path::Path;

use crate::export::domain::error::ExportError;

/// Check available disk space at a given path.
///
/// Uses `statvfs(2)` on Unix/Android and `GetDiskFreeSpaceExW` via the Windows
/// API on Windows to return the number of bytes available to an unprivileged
/// process on the filesystem that contains `path`.
///
/// # Arguments
/// * `path` - The path whose filesystem should be queried
///
/// # Returns
/// Available bytes for the calling process, or an [`ExportError`] on failure.
pub fn check_available_space(path: &Path) -> Result<u64, ExportError> {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::ffi::OsStrExt;

        // Raw FFI — avoids a direct `windows-sys` dependency.
        extern "system" {
            fn GetDiskFreeSpaceExW(
                lpDirectoryName: *const u16,
                lpFreeBytesAvailableToCaller: *mut u64,
                lpTotalNumberOfBytes: *mut u64,
                lpTotalNumberOfFreeBytes: *mut u64,
            ) -> i32;
        }

        let wide: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
        let mut free_bytes: u64 = 0;
        // SAFETY: `wide` is a valid null-terminated UTF-16 path; `free_bytes` is
        // a valid out-pointer. The other two out-pointers are optional and may be null.
        let ok = unsafe {
            GetDiskFreeSpaceExW(
                wide.as_ptr(),
                &mut free_bytes,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        };
        if ok == 0 {
            return Err(ExportError::IoError(std::io::Error::last_os_error()));
        }
        Ok(free_bytes)
    }

    #[cfg(not(target_os = "windows"))]
    {
        use std::ffi::CString;
        use std::os::unix::ffi::OsStrExt;

        let path_cstr = CString::new(path.as_os_str().as_bytes())
            .map_err(|_| ExportError::InvalidPath(path.display().to_string()))?;

        // SAFETY: `stat` is fully initialised by `statvfs` before it is read.
        let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };
        let ret = unsafe { libc::statvfs(path_cstr.as_ptr(), &mut stat) };
        if ret != 0 {
            return Err(ExportError::IoError(std::io::Error::last_os_error()));
        }

        // f_bavail: blocks available to unprivileged processes
        // f_frsize: fundamental filesystem block size in bytes
        Ok(stat.f_bavail as u64 * stat.f_frsize as u64)
    }
}

/// Validate that there's enough disk space for export
///
/// # Arguments
/// * `path` - The destination path
/// * `estimated_size` - Estimated archive size in bytes
/// * `safety_buffer` - Safety buffer in bytes (default 100MB)
///
/// # Returns
/// Ok if there's sufficient space, or an error
pub fn validate_disk_space(path: &Path, estimated_size: u64) -> Result<(), ExportError> {
    const SAFETY_BUFFER: u64 = 100 * 1024 * 1024; // 100 MB

    let available = check_available_space(path)?;
    let required = estimated_size + SAFETY_BUFFER;

    if available < required {
        Err(ExportError::DiskSpaceError)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_disk_space_sufficient() {
        // This test will pass on any system with more than 200MB free space
        if let Ok(available) = check_available_space(Path::new("/tmp"))
            && available > 200 * 1024 * 1024
        {
            assert!(validate_disk_space(Path::new("/tmp"), 50 * 1024 * 1024).is_ok());
        }
    }
}
