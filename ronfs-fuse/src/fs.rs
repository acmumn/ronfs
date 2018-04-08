use std::ffi::OsStr;
use std::path::Path;

use fuse::{FileAttr, FileType, ReplyAttr, ReplyBmap, ReplyCreate, ReplyData,
           ReplyDirectory, ReplyEmpty, ReplyEntry, ReplyLock, ReplyOpen,
           ReplyStatfs, ReplyWrite, ReplyXattr, Request};
use hyper::{Client, Uri, client::HttpConnector};
use libc::{c_int, ENOENT, ENOSYS};
use time::Timespec;

use client::RonFSClient;

/// The actual filesystem implementation.
#[derive(Debug)]
pub struct Filesystem {
    client: RonFSClient,
}

impl Filesystem {
    /// Creates a new filesystem.
    pub fn new(client: Client<HttpConnector>, base_uri: Uri) -> Filesystem {
        let client = RonFSClient::new(client, base_uri);
        Filesystem { client }
    }
}

impl ::fuse::Filesystem for Filesystem {
    fn init(&mut self, _req: &Request) -> Result<(), c_int> {
        Ok(())
    }

    fn destroy(&mut self, _req: &Request) {}

    fn lookup(
        &mut self,
        _req: &Request,
        parent: u64,
        name: &OsStr,
        reply: ReplyEntry,
    ) {
        info!("Filesystem::lookup(parent={:?}, name={:?})", parent, name);
        reply.error(ENOSYS);
    }

    fn forget(&mut self, _req: &Request, ino: u64, nlookup: u64) {
        info!("Filesystem::forget(ino={:?}, nlookup={:?})", ino, nlookup);
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        info!("Filesystem::getattr(ino={:?})", ino);
        reply.error(ENOSYS);
    }

    fn setattr(
        &mut self,
        _req: &Request,
        ino: u64,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        atime: Option<Timespec>,
        mtime: Option<Timespec>,
        fh: Option<u64>,
        crtime: Option<Timespec>,
        chgtime: Option<Timespec>,
        bkuptime: Option<Timespec>,
        flags: Option<u32>,
        reply: ReplyAttr,
    ) {
        info!("Filesystem::setattr(ino={:?}, mode={:?}, uid={:?}, gid={:?}, size={:?}, atime={:?}, mtime={:?}, fh={:?}, crtime={:?}, chgtime={:?}, bkuptime={:?}, flags={:?})", ino, mode, uid, gid, size, atime, mtime,fh, crtime, chgtime, bkuptime, flags);
        reply.error(ENOSYS);
    }

    fn readlink(&mut self, _req: &Request, ino: u64, reply: ReplyData) {
        info!("Filesystem::readlink(ino={:?})", ino);
        reply.error(ENOSYS);
    }

    fn mknod(
        &mut self,
        _req: &Request,
        parent: u64,
        name: &OsStr,
        mode: u32,
        rdev: u32,
        reply: ReplyEntry,
    ) {
        info!(
            "Filesystem::mknod(parent={:?}, name={:?}, mode={:?}, rdev={:?})",
            parent, name, mode, rdev
        );
        reply.error(ENOSYS);
    }

    fn mkdir(
        &mut self,
        _req: &Request,
        parent: u64,
        name: &OsStr,
        mode: u32,
        reply: ReplyEntry,
    ) {
        info!(
            "Filesystem::mkdir(parent={:?}, name={:?}, mode={:?})",
            parent, name, mode
        );
        reply.error(ENOSYS);
    }

    fn unlink(
        &mut self,
        _req: &Request,
        parent: u64,
        name: &OsStr,
        reply: ReplyEmpty,
    ) {
        info!("Filesystem::unlink(parent={:?}, name={:?})", parent, name);
        reply.error(ENOSYS);
    }

    fn rmdir(
        &mut self,
        _req: &Request,
        parent: u64,
        name: &OsStr,
        reply: ReplyEmpty,
    ) {
        info!("Filesystem::rmdir(parent={:?}, name={:?})", parent, name);
        reply.error(ENOSYS);
    }

    fn symlink(
        &mut self,
        _req: &Request,
        parent: u64,
        name: &OsStr,
        link: &Path,
        reply: ReplyEntry,
    ) {
        info!(
            "Filesystem::symlink(parent={:?}, name={:?}, link={:?})",
            parent, name, link
        );
        reply.error(ENOSYS);
    }

    fn rename(
        &mut self,
        _req: &Request,
        parent: u64,
        name: &OsStr,
        newparent: u64,
        newname: &OsStr,
        reply: ReplyEmpty,
    ) {
        info!(
            "Filesystem::rename(parent={:?}, name={:?}, newparent={:?}, newname={:?})",
            parent, name, newparent, newname
        );
        reply.error(ENOSYS);
    }

    fn link(
        &mut self,
        _req: &Request,
        ino: u64,
        newparent: u64,
        newname: &OsStr,
        reply: ReplyEntry,
    ) {
        info!(
            "Filesystem::link(ino={:?}, newparent={:?}, newname={:?})",
            ino, newparent, newname
        );
        reply.error(ENOSYS);
    }

    fn open(&mut self, _req: &Request, ino: u64, flags: u32, reply: ReplyOpen) {
        info!("Filesystem::open(ino={:?}, flags={:?})", ino, flags);
        reply.opened(0, 0);
    }

    fn read(
        &mut self,
        _req: &Request,
        ino: u64,
        fh: u64,
        offset: i64,
        size: u32,
        reply: ReplyData,
    ) {
        info!(
            "Filesystem::read(ino={:?}, fh={:?}, offset={:?}, size={:?})",
            ino, fh, offset, size
        );
        reply.error(ENOSYS);
    }

    fn write(
        &mut self,
        _req: &Request,
        ino: u64,
        fh: u64,
        offset: i64,
        data: &[u8],
        flags: u32,
        reply: ReplyWrite,
    ) {
        info!(
            "Filesystem::write(ino={:?}, fh={:?}, offset={:?}, data={:?}, flags={:?})",
            ino, fh, offset, data, flags
        );
        reply.error(ENOSYS);
    }

    fn flush(
        &mut self,
        _req: &Request,
        ino: u64,
        fh: u64,
        lock_owner: u64,
        reply: ReplyEmpty,
    ) {
        info!(
            "Filesystem::flush(ino={:?}, fh={:?}, lock_owner={:?})",
            ino, fh, lock_owner
        );
        reply.error(ENOSYS);
    }

    fn release(
        &mut self,
        _req: &Request,
        ino: u64,
        fh: u64,
        flags: u32,
        lock_owner: u64,
        flush: bool,
        reply: ReplyEmpty,
    ) {
        info!(
            "Filesystem::release(ino={:?}, fh={:?}, flags={:?}, lock_owner={:?}, flush={:?})",
            ino, fh, flags, lock_owner, flush
        );
        reply.ok();
    }

    fn fsync(
        &mut self,
        _req: &Request,
        ino: u64,
        fh: u64,
        datasync: bool,
        reply: ReplyEmpty,
    ) {
        info!(
            "Filesystem::fsync(ino={:?}, fh={:?}, datasync={:?})",
            ino, fh, datasync
        );
        reply.error(ENOSYS);
    }

    fn opendir(
        &mut self,
        _req: &Request,
        ino: u64,
        flags: u32,
        reply: ReplyOpen,
    ) {
        info!("Filesystem::opendir(ino={:?}, flags={:?})", ino, flags);
        reply.opened(0, 0);
    }

    fn readdir(
        &mut self,
        _req: &Request,
        ino: u64,
        fh: u64,
        offset: i64,
        mut reply: ReplyDirectory,
    ) {
        info!(
            "Filesystem::readdir(ino={:?}, fh={:?}, offset={:?})",
            ino, fh, offset
        );
        reply.error(ENOSYS);
    }

    fn releasedir(
        &mut self,
        _req: &Request,
        ino: u64,
        fh: u64,
        flags: u32,
        reply: ReplyEmpty,
    ) {
        info!(
            "Filesystem::releasedir(ino={:?}, fh={:?}, flags={:?})",
            ino, fh, flags
        );
        reply.ok();
    }

    fn fsyncdir(
        &mut self,
        _req: &Request,
        ino: u64,
        fh: u64,
        datasync: bool,
        reply: ReplyEmpty,
    ) {
        info!(
            "Filesystem::fsyncdir(ino={:?}, fh={:?}, datasync={:?})",
            ino, fh, datasync
        );
        reply.error(ENOSYS);
    }

    fn statfs(&mut self, _req: &Request, ino: u64, reply: ReplyStatfs) {
        info!("Filesystem::statfs(ino={:?})", ino);
        reply.statfs(0, 0, 0, 0, 0, 512, 255, 0);
    }

    fn setxattr(
        &mut self,
        _req: &Request,
        ino: u64,
        name: &OsStr,
        value: &[u8],
        flags: u32,
        position: u32,
        reply: ReplyEmpty,
    ) {
        info!(
            "Filesystem::setxattr(ino={:?}, name={:?}, value={:?}, flags={:?}, position={:?})",
            ino, name, value, flags, position
        );
        reply.error(ENOSYS);
    }

    fn getxattr(
        &mut self,
        _req: &Request,
        ino: u64,
        name: &OsStr,
        size: u32,
        reply: ReplyXattr,
    ) {
        info!(
            "Filesystem::listxattr(ino={:?}, name={:?}, size={:?})",
            ino, name, size
        );
        reply.error(ENOSYS);
    }

    fn listxattr(
        &mut self,
        _req: &Request,
        ino: u64,
        size: u32,
        reply: ReplyXattr,
    ) {
        info!("Filesystem::listxattr(ino={:?}, size={:?})", ino, size);
        reply.error(ENOSYS);
    }

    fn removexattr(
        &mut self,
        _req: &Request,
        ino: u64,
        name: &OsStr,
        reply: ReplyEmpty,
    ) {
        info!("Filesystem::removexattr(ino={:?}, name={:?})", ino, name);
        reply.error(ENOSYS);
    }

    fn access(
        &mut self,
        _req: &Request,
        ino: u64,
        mask: u32,
        reply: ReplyEmpty,
    ) {
        info!("Filesystem::access(ino={:?}, mask={:?})", ino, mask);
        reply.error(ENOSYS);
    }

    fn create(
        &mut self,
        _req: &Request,
        parent: u64,
        name: &OsStr,
        mode: u32,
        flags: u32,
        reply: ReplyCreate,
    ) {
        info!(
            "Filesystem::create(parent={:?}, name={:?}, mode={:?}, flags={:?})",
            parent, name, mode, flags
        );
        reply.error(ENOSYS);
    }

    fn getlk(
        &mut self,
        _req: &Request,
        ino: u64,
        fh: u64,
        lock_owner: u64,
        start: u64,
        end: u64,
        typ: u32,
        pid: u32,
        reply: ReplyLock,
    ) {
        info!(
            "Filesystem::getlk(ino={:?}, fh={:?}, lock_owner={:?}, start={:?}, end={:?}, typ={:?}, pid={:?})",
            ino, fh, lock_owner, start, end, typ, pid
        );
        reply.error(ENOSYS);
    }

    fn setlk(
        &mut self,
        _req: &Request,
        ino: u64,
        fh: u64,
        lock_owner: u64,
        start: u64,
        end: u64,
        typ: u32,
        pid: u32,
        sleep: bool,
        reply: ReplyEmpty,
    ) {
        info!(
            "Filesystem::setlk(ino={:?}, fh={:?}, lock_owner={:?}, start={:?}, end={:?}, typ={:?}, pid={:?}, sleep={:?})",
            ino, fh, lock_owner, start, end, typ, pid, sleep
        );
        reply.error(ENOSYS);
    }

    fn bmap(
        &mut self,
        _req: &Request,
        ino: u64,
        blocksize: u32,
        idx: u64,
        reply: ReplyBmap,
    ) {
        info!(
            "Filesystem::bmap(ino={:?}, blocksize={:?}, idx={:?})",
            ino, blocksize, idx
        );
        reply.error(ENOSYS);
    }
}
