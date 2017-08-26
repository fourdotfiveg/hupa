//! Unix only function for hupa

use error::*;
use libc::*;
use std::fs::Metadata;
use std::os::unix::fs::*;
use std::path::Path;
use super::*;

impl Hupa {
    /// Set eid
    pub fn set_eid<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let uid = unsafe { getuid() };
        if uid != 0 {
            return Ok(());
        }
        let path = path.as_ref();
        let metadata = match Self::get_metadata(path) {
            Ok(m) => m,
            Err(_) => path.parent().unwrap().metadata()?,
        };
        let file_uid = metadata.uid();
        let file_gid = metadata.gid();
        // Reset effective uid for gid just for safety
        unsafe { setresuid(0, 0, 0) };
        unsafe { setresgid(0, file_gid, 0) };
        unsafe { setresuid(0, file_uid, 0) };
        Ok(())
    }

    /// Set eid to backup file
    ///
    /// Use $HOME for getting file permissions
    pub fn set_eid_backup(&self) -> Result<()> {
        let home = ::std::env::var("HOME")?;
        self.set_eid(home)
    }

    /// Set eid to restore file
    ///
    /// Use origin path for getting file permissions
    pub fn set_eid_restore(&self) -> Result<()> {
        self.set_eid(&self.origin_path)
    }

    /// Get metadata or get parent one
    pub fn get_metadata<P: AsRef<Path>>(path: P) -> Result<Metadata> {
        let path = path.as_ref();
        if let Ok(m) = path.metadata() {
            return Ok(m);
        } else if let Some(p) = path.parent() {
            return Self::get_metadata(p);
        } else {
            path.metadata().map_err(|e| e.into())
        }
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
    (
        owner_mode & 2 == 2,
        group_mode & 2 == 2,
        other_mode & 2 == 2,
    )
}
