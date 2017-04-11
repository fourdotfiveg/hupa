//! Unix only function for hupa

use error::*;
use libc::*;
use std::os::unix::fs::*;
use super::*;

impl Hupa {
    /// Set euid to restore file
    pub fn set_eid(&self) -> Result<()> {
        let metadata = self.origin_path.metadata()?;
        let file_uid = metadata.uid();
        let file_gid = metadata.gid();
        unsafe { setresuid(0, file_uid, 0) };
        unsafe { setresgid(0, file_gid, 0) };
        Ok(())
    }

    /// Check if user needs to be root to restore this hupa
    pub fn needs_root(&self) -> bool {
        let uid = unsafe { getuid() };
        if uid == 0 {
            return false;
        }
        let metadata = match self.origin_path.metadata() {
            Ok(m) => m,
            Err(_) => return true,
        };
        let euid = unsafe { geteuid() };
        let egid = unsafe { getegid() };
        let file_uid = metadata.uid();
        let file_gid = metadata.gid();
        let permissions = metadata.permissions();
        let mode = permissions.mode();
        let (owner_w, group_w, other_w) = can_write(mode);
        if euid == file_uid {
            !owner_w
        } else if egid == file_gid {
            !group_w
        } else {
            !other_w
        }
    }
}

/// Convert mode into bool for owner, group and other
fn can_write(mode: u32) -> (bool, bool, bool) {
    let owner_mode = (mode & 0o700) >> 6;
    let group_mode = (mode & 0o070) >> 3;
    let other_mode = mode & 0o007;
    (owner_mode & 2 == 2, group_mode & 2 == 2, other_mode & 2 == 2)
}
