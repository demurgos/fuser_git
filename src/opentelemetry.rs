use fuser::{
    fuse_forget_one, Filesystem, KernelConfig, ReplyAttr, ReplyBmap, ReplyCreate, ReplyData,
    ReplyDirectory, ReplyDirectoryPlus, ReplyEmpty, ReplyEntry, ReplyIoctl, ReplyLock, ReplyLseek,
    ReplyOpen, ReplyPoll, ReplyStatfs, ReplyWrite, ReplyXattr, Request, TimeOrNow,
};
use libc::{c_int, ENOSYS, EPERM};
use opentelemetry::trace::{FutureExt, Span, Tracer, TracerProvider};
use std::ffi::OsStr;
use std::path::Path;
use std::time::SystemTime;

/// Wrapper around a `fuser::Filesystem` implementation to trace calls
pub struct OpentelemetryFuser<TyFs, TyTracer> {
    fs: TyFs,
    tracer: TyTracer,
}

impl<TyFs, TyTracer> OpentelemetryFuser<TyFs, TyTracer> {
    pub fn new(fs: TyFs, tracer: TyTracer) -> Self {
        Self { fs, tracer }
    }

    pub fn new_with_provider<TyTracerProvider>(fs: TyFs, provider: &TyTracerProvider) -> Self
    where
        TyTracerProvider: TracerProvider<Tracer = TyTracer>,
    {
        Self::new(fs, provider.tracer("fuser"))
    }
}

impl<TyFs, TyTracer> Filesystem for OpentelemetryFuser<TyFs, TyTracer>
where
    TyFs: Filesystem,
    TyTracer: Tracer,
{
    fn init(&mut self, req: &Request<'_>, config: &mut KernelConfig) -> Result<(), c_int> {
        let mut span = self.tracer.start("Filesystem::init");
        let res = self.fs.init(req, config);
        span.end();
        res
    }

    fn destroy(&mut self) {
        let mut span = self.tracer.start("Filesystem::init");
        self.fs.destroy();
        span.end();
    }

    fn lookup(&mut self, req: &Request<'_>, parent: u64, name: &OsStr, reply: ReplyEntry) {
        let mut span = self.tracer.start("Filesystem::lookup");
        self.fs.lookup(req, parent, name, reply);
        span.end();
    }

    fn forget(&mut self, req: &Request<'_>, ino: u64, nlookup: u64) {
        let mut span = self.tracer.start("Filesystem::forget");
        self.fs.forget(req, ino, nlookup);
        span.end();
    }

    /// Like forget, but take multiple forget requests at once for performance. The default
    /// implementation will fallback to forget.
    fn batch_forget(&mut self, req: &Request<'_>, nodes: &[fuse_forget_one]) {
        let mut span = self.tracer.start("Filesystem::batch_forget");
        self.fs.batch_forget(req, nodes);
        span.end();
    }

    /// Get file attributes.
    fn getattr(&mut self, req: &Request<'_>, ino: u64, reply: ReplyAttr) {
        // warn!("[Not Implemented] getattr(ino: {:#x?})", ino);
        let mut span = self.tracer.start("Filesystem::getattr");
        self.fs.getattr(req, ino, reply);
        span.end();
    }

    /// Set file attributes.
    fn setattr(
        &mut self,
        req: &Request<'_>,
        ino: u64,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        atime: Option<TimeOrNow>,
        mtime: Option<TimeOrNow>,
        ctime: Option<SystemTime>,
        fh: Option<u64>,
        crtime: Option<SystemTime>,
        chgtime: Option<SystemTime>,
        bkuptime: Option<SystemTime>,
        flags: Option<u32>,
        reply: ReplyAttr,
    ) {
        // debug!(
        //     "[Not Implemented] setattr(ino: {:#x?}, mode: {:?}, uid: {:?}, \
        //     gid: {:?}, size: {:?}, fh: {:?}, flags: {:?})",
        //     ino, mode, uid, gid, size, fh, flags
        // );
        // reply.error(ENOSYS);
        let mut span = self.tracer.start("Filesystem::setattr");
        self.fs.setattr(
            req, ino, mode, uid, gid, size, atime, mtime, ctime, fh, crtime, chgtime, bkuptime,
            flags, reply,
        );
        span.end();
    }

    /// Read symbolic link.
    fn readlink(&mut self, req: &Request<'_>, ino: u64, reply: ReplyData) {
        // debug!("[Not Implemented] readlink(ino: {:#x?})", ino);
        // reply.error(ENOSYS);
        let mut span = self.tracer.start("Filesystem::readlink");
        self.fs.readlink(req, ino, reply);
        span.end();
    }

    /// Create file node.
    /// Create a regular file, character device, block device, fifo or socket node.
    fn mknod(
        &mut self,
        req: &Request<'_>,
        parent: u64,
        name: &OsStr,
        mode: u32,
        umask: u32,
        rdev: u32,
        reply: ReplyEntry,
    ) {
        // debug!(
        //     "[Not Implemented] mknod(parent: {:#x?}, name: {:?}, mode: {}, \
        //     umask: {:#x?}, rdev: {})",
        //     parent, name, mode, umask, rdev
        // );
        // reply.error(ENOSYS);
        let mut span = self.tracer.start("Filesystem::mknod");
        self.fs.mknod(req, parent, name, mode, umask, rdev, reply);
        span.end();
    }

    /// Create a directory.
    fn mkdir(
        &mut self,
        req: &Request<'_>,
        parent: u64,
        name: &OsStr,
        mode: u32,
        umask: u32,
        reply: ReplyEntry,
    ) {
        // debug!(
        //     "[Not Implemented] mkdir(parent: {:#x?}, name: {:?}, mode: {}, umask: {:#x?})",
        //     parent, name, mode, umask
        // );
        // reply.error(ENOSYS);
        let mut span = self.tracer.start("Filesystem::mkdir");
        self.fs.mkdir(req, parent, name, mode, umask, reply);
        span.end();
    }

    /// Remove a file.
    fn unlink(&mut self, req: &Request<'_>, parent: u64, name: &OsStr, reply: ReplyEmpty) {
        // debug!(
        //     "[Not Implemented] unlink(parent: {:#x?}, name: {:?})",
        //     parent, name,
        // );
        // reply.error(ENOSYS);
        let mut span = self.tracer.start("Filesystem::unlink");
        self.fs.unlink(req, parent, name, reply);
        span.end();
    }

    /// Remove a directory.
    fn rmdir(&mut self, req: &Request<'_>, parent: u64, name: &OsStr, reply: ReplyEmpty) {
        // debug!(
        //     "[Not Implemented] rmdir(parent: {:#x?}, name: {:?})",
        //     parent, name,
        // );
        // reply.error(ENOSYS);
        let mut span = self.tracer.start("Filesystem::rmdir");
        self.fs.rmdir(req, parent, name, reply);
        span.end();
    }

    /// Create a symbolic link.
    fn symlink(
        &mut self,
        req: &Request<'_>,
        parent: u64,
        link_name: &OsStr,
        target: &Path,
        reply: ReplyEntry,
    ) {
        // debug!(
        //     "[Not Implemented] symlink(parent: {:#x?}, link_name: {:?}, target: {:?})",
        //     parent, link_name, target,
        // );
        // reply.error(EPERM);
        let mut span = self.tracer.start("Filesystem::symlink");
        self.fs.symlink(req, parent, link_name, target, reply);
        span.end();
    }

    /// Rename a file.
    fn rename(
        &mut self,
        req: &Request<'_>,
        parent: u64,
        name: &OsStr,
        newparent: u64,
        newname: &OsStr,
        flags: u32,
        reply: ReplyEmpty,
    ) {
        // debug!(
        //     "[Not Implemented] rename(parent: {:#x?}, name: {:?}, newparent: {:#x?}, \
        //     newname: {:?}, flags: {})",
        //     parent, name, newparent, newname, flags,
        // );
        // reply.error(ENOSYS);
        let mut span = self.tracer.start("Filesystem::rename");
        self.fs
            .rename(req, parent, name, newparent, newname, flags, reply);
        span.end();
    }

    /// Create a hard link.
    fn link(
        &mut self,
        req: &Request<'_>,
        ino: u64,
        newparent: u64,
        newname: &OsStr,
        reply: ReplyEntry,
    ) {
        // debug!(
        //     "[Not Implemented] link(ino: {:#x?}, newparent: {:#x?}, newname: {:?})",
        //     ino, newparent, newname
        // );
        // reply.error(EPERM);
        let mut span = self.tracer.start("Filesystem::link");
        self.fs.link(req, ino, newparent, newname, reply);
        span.end();
    }

    /// Open a file.
    /// Open flags (with the exception of O_CREAT, O_EXCL, O_NOCTTY and O_TRUNC) are
    /// available in flags. Filesystem may store an arbitrary file handle (pointer, index,
    /// etc) in fh, and use this in other all other file operations (read, write, flush,
    /// release, fsync). Filesystem may also implement stateless file I/O and not store
    /// anything in fh. There are also some flags (direct_io, keep_cache) which the
    /// filesystem may set, to change the way the file is opened. See fuse_file_info
    /// structure in <fuse_common.h> for more details.
    fn open(&mut self, req: &Request<'_>, ino: u64, flags: i32, reply: ReplyOpen) {
        // reply.opened(0, 0);
        let mut span = self.tracer.start("Filesystem::open");
        self.fs.open(req, ino, flags, reply);
        span.end();
    }

    /// Read data.
    /// Read should send exactly the number of bytes requested except on EOF or error,
    /// otherwise the rest of the data will be substituted with zeroes. An exception to
    /// this is when the file has been opened in 'direct_io' mode, in which case the
    /// return value of the read system call will reflect the return value of this
    /// operation. fh will contain the value set by the open method, or will be undefined
    /// if the open method didn't set any value.
    ///
    /// flags: these are the file flags, such as O_SYNC. Only supported with ABI >= 7.9
    /// lock_owner: only supported with ABI >= 7.9
    fn read(
        &mut self,
        req: &Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        size: u32,
        flags: i32,
        lock_owner: Option<u64>,
        reply: ReplyData,
    ) {
        // warn!(
        //     "[Not Implemented] read(ino: {:#x?}, fh: {}, offset: {}, size: {}, \
        //     flags: {:#x?}, lock_owner: {:?})",
        //     ino, fh, offset, size, flags, lock_owner
        // );
        // reply.error(ENOSYS);
        let mut span = self.tracer.start("Filesystem::read");
        self.fs
            .read(req, ino, fh, offset, size, flags, lock_owner, reply);
        span.end();
    }

    /// Write data.
    /// Write should return exactly the number of bytes requested except on error. An
    /// exception to this is when the file has been opened in 'direct_io' mode, in
    /// which case the return value of the write system call will reflect the return
    /// value of this operation. fh will contain the value set by the open method, or
    /// will be undefined if the open method didn't set any value.
    ///
    /// write_flags: will contain FUSE_WRITE_CACHE, if this write is from the page cache. If set,
    /// the pid, uid, gid, and fh may not match the value that would have been sent if write cachin
    /// is disabled
    /// flags: these are the file flags, such as O_SYNC. Only supported with ABI >= 7.9
    /// lock_owner: only supported with ABI >= 7.9
    fn write(
        &mut self,
        req: &Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        data: &[u8],
        write_flags: u32,
        flags: i32,
        lock_owner: Option<u64>,
        reply: ReplyWrite,
    ) {
        // debug!(
        //     "[Not Implemented] write(ino: {:#x?}, fh: {}, offset: {}, data.len(): {}, \
        //     write_flags: {:#x?}, flags: {:#x?}, lock_owner: {:?})",
        //     ino,
        //     fh,
        //     offset,
        //     data.len(),
        //     write_flags,
        //     flags,
        //     lock_owner
        // );
        // reply.error(ENOSYS);
        let mut span = self.tracer.start("Filesystem::write");
        self.fs.write(
            req,
            ino,
            fh,
            offset,
            data,
            write_flags,
            flags,
            lock_owner,
            reply,
        );
        span.end();
    }

    /// Flush method.
    /// This is called on each close() of the opened file. Since file descriptors can
    /// be duplicated (dup, dup2, fork), for one open call there may be many flush
    /// calls. Filesystems shouldn't assume that flush will always be called after some
    /// writes, or that if will be called at all. fh will contain the value set by the
    /// open method, or will be undefined if the open method didn't set any value.
    /// NOTE: the name of the method is misleading, since (unlike fsync) the filesystem
    /// is not forced to flush pending writes. One reason to flush data, is if the
    /// filesystem wants to return write errors. If the filesystem supports file locking
    /// operations (setlk, getlk) it should remove all locks belonging to 'lock_owner'.
    fn flush(&mut self, req: &Request<'_>, ino: u64, fh: u64, lock_owner: u64, reply: ReplyEmpty) {
        // debug!(
        //     "[Not Implemented] flush(ino: {:#x?}, fh: {}, lock_owner: {:?})",
        //     ino, fh, lock_owner
        // );
        // reply.error(ENOSYS);
        let mut span = self.tracer.start("Filesystem::flush");
        self.fs.flush(req, ino, fh, lock_owner, reply);
        span.end();
    }

    /// Release an open file.
    /// Release is called when there are no more references to an open file: all file
    /// descriptors are closed and all memory mappings are unmapped. For every open
    /// call there will be exactly one release call. The filesystem may reply with an
    /// error, but error values are not returned to close() or munmap() which triggered
    /// the release. fh will contain the value set by the open method, or will be undefined
    /// if the open method didn't set any value. flags will contain the same flags as for
    /// open.
    fn release(
        &mut self,
        req: &Request<'_>,
        ino: u64,
        fh: u64,
        flags: i32,
        lock_owner: Option<u64>,
        flush: bool,
        reply: ReplyEmpty,
    ) {
        // reply.ok();
        self.fs
            .release(req, ino, fh, flags, lock_owner, flush, reply)
    }

    /// Synchronize file contents.
    /// If the datasync parameter is non-zero, then only the user data should be flushed,
    /// not the meta data.
    fn fsync(&mut self, req: &Request<'_>, ino: u64, fh: u64, datasync: bool, reply: ReplyEmpty) {
        // debug!(
        //     "[Not Implemented] fsync(ino: {:#x?}, fh: {}, datasync: {})",
        //     ino, fh, datasync
        // );
        // reply.error(ENOSYS);
        self.fs.fsync(req, ino, fh, datasync, reply)
    }

    /// Open a directory.
    /// Filesystem may store an arbitrary file handle (pointer, index, etc) in fh, and
    /// use this in other all other directory stream operations (readdir, releasedir,
    /// fsyncdir). Filesystem may also implement stateless directory I/O and not store
    /// anything in fh, though that makes it impossible to implement standard conforming
    /// directory stream operations in case the contents of the directory can change
    /// between opendir and releasedir.
    fn opendir(&mut self, req: &Request<'_>, ino: u64, flags: i32, reply: ReplyOpen) {
        // reply.opened(0, 0);
        self.fs.opendir(req, ino, flags, reply)
    }

    /// Read directory.
    /// Send a buffer filled using buffer.fill(), with size not exceeding the
    /// requested size. Send an empty buffer on end of stream. fh will contain the
    /// value set by the opendir method, or will be undefined if the opendir method
    /// didn't set any value.
    fn readdir(
        &mut self,
        req: &Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        reply: ReplyDirectory,
    ) {
        // warn!(
        //     "[Not Implemented] readdir(ino: {:#x?}, fh: {}, offset: {})",
        //     ino, fh, offset
        // );
        // reply.error(ENOSYS);
        self.fs.readdir(req, ino, fh, offset, reply)
    }

    /// Read directory.
    /// Send a buffer filled using buffer.fill(), with size not exceeding the
    /// requested size. Send an empty buffer on end of stream. fh will contain the
    /// value set by the opendir method, or will be undefined if the opendir method
    /// didn't set any value.
    fn readdirplus(
        &mut self,
        req: &Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        reply: ReplyDirectoryPlus,
    ) {
        // debug!(
        //     "[Not Implemented] readdirplus(ino: {:#x?}, fh: {}, offset: {})",
        //     ino, fh, offset
        // );
        // reply.error(ENOSYS);
        self.fs.readdirplus(req, ino, fh, offset, reply)
    }

    /// Release an open directory.
    /// For every opendir call there will be exactly one releasedir call. fh will
    /// contain the value set by the opendir method, or will be undefined if the
    /// opendir method didn't set any value.
    fn releasedir(&mut self, req: &Request<'_>, ino: u64, fh: u64, flags: i32, reply: ReplyEmpty) {
        // reply.ok();
        self.fs.releasedir(req, ino, fh, flags, reply)
    }

    /// Synchronize directory contents.
    /// If the datasync parameter is set, then only the directory contents should
    /// be flushed, not the meta data. fh will contain the value set by the opendir
    /// method, or will be undefined if the opendir method didn't set any value.
    fn fsyncdir(
        &mut self,
        req: &Request<'_>,
        ino: u64,
        fh: u64,
        datasync: bool,
        reply: ReplyEmpty,
    ) {
        // debug!(
        //     "[Not Implemented] fsyncdir(ino: {:#x?}, fh: {}, datasync: {})",
        //     ino, fh, datasync
        // );
        // reply.error(ENOSYS);
        self.fs.fsyncdir(req, ino, fh, datasync, reply)
    }

    /// Get file system statistics.
    fn statfs(&mut self, req: &Request<'_>, ino: u64, reply: ReplyStatfs) {
        // reply.statfs(0, 0, 0, 0, 0, 512, 255, 0);
        self.fs.statfs(req, ino, reply)
    }

    /// Set an extended attribute.
    fn setxattr(
        &mut self,
        req: &Request<'_>,
        ino: u64,
        name: &OsStr,
        value: &[u8],
        flags: i32,
        position: u32,
        reply: ReplyEmpty,
    ) {
        // debug!(
        //     "[Not Implemented] setxattr(ino: {:#x?}, name: {:?}, flags: {:#x?}, position: {})",
        //     ino, name, flags, position
        // );
        // reply.error(ENOSYS);
        self.fs
            .setxattr(req, ino, name, value, flags, position, reply)
    }

    /// Get an extended attribute.
    /// If `size` is 0, the size of the value should be sent with `reply.size()`.
    /// If `size` is not 0, and the value fits, send it with `reply.data()`, or
    /// `reply.error(ERANGE)` if it doesn't.
    fn getxattr(
        &mut self,
        req: &Request<'_>,
        ino: u64,
        name: &OsStr,
        size: u32,
        reply: ReplyXattr,
    ) {
        // debug!(
        //     "[Not Implemented] getxattr(ino: {:#x?}, name: {:?}, size: {})",
        //     ino, name, size
        // );
        // reply.error(ENOSYS);
        self.fs.getxattr(req, ino, name, size, reply)
    }

    /// List extended attribute names.
    /// If `size` is 0, the size of the value should be sent with `reply.size()`.
    /// If `size` is not 0, and the value fits, send it with `reply.data()`, or
    /// `reply.error(ERANGE)` if it doesn't.
    fn listxattr(&mut self, req: &Request<'_>, ino: u64, size: u32, reply: ReplyXattr) {
        // debug!(
        //     "[Not Implemented] listxattr(ino: {:#x?}, size: {})",
        //     ino, size
        // );
        // reply.error(ENOSYS);
        self.fs.listxattr(req, ino, size, reply)
    }

    /// Remove an extended attribute.
    fn removexattr(&mut self, req: &Request<'_>, ino: u64, name: &OsStr, reply: ReplyEmpty) {
        // debug!(
        //     "[Not Implemented] removexattr(ino: {:#x?}, name: {:?})",
        //     ino, name
        // );
        // reply.error(ENOSYS);
        self.fs.removexattr(req, ino, name, reply)
    }

    /// Check file access permissions.
    /// This will be called for the access() system call. If the 'default_permissions'
    /// mount option is given, this method is not called. This method is not called
    /// under Linux kernel versions 2.4.x
    fn access(&mut self, req: &Request<'_>, ino: u64, mask: i32, reply: ReplyEmpty) {
        // debug!("[Not Implemented] access(ino: {:#x?}, mask: {})", ino, mask);
        // reply.error(ENOSYS);
        self.fs.access(req, ino, mask, reply)
    }

    /// Create and open a file.
    /// If the file does not exist, first create it with the specified mode, and then
    /// open it. Open flags (with the exception of O_NOCTTY) are available in flags.
    /// Filesystem may store an arbitrary file handle (pointer, index, etc) in fh,
    /// and use this in other all other file operations (read, write, flush, release,
    /// fsync). There are also some flags (direct_io, keep_cache) which the
    /// filesystem may set, to change the way the file is opened. See fuse_file_info
    /// structure in <fuse_common.h> for more details. If this method is not
    /// implemented or under Linux kernel versions earlier than 2.6.15, the mknod()
    /// and open() methods will be called instead.
    fn create(
        &mut self,
        req: &Request<'_>,
        parent: u64,
        name: &OsStr,
        mode: u32,
        umask: u32,
        flags: i32,
        reply: ReplyCreate,
    ) {
        // debug!(
        //     "[Not Implemented] create(parent: {:#x?}, name: {:?}, mode: {}, umask: {:#x?}, \
        //     flags: {:#x?})",
        //     parent, name, mode, umask, flags
        // );
        // reply.error(ENOSYS);
        self.fs.create(req, parent, name, mode, umask, flags, reply)
    }

    /// Test for a POSIX file lock.
    fn getlk(
        &mut self,
        req: &Request<'_>,
        ino: u64,
        fh: u64,
        lock_owner: u64,
        start: u64,
        end: u64,
        typ: i32,
        pid: u32,
        reply: ReplyLock,
    ) {
        // debug!(
        //     "[Not Implemented] getlk(ino: {:#x?}, fh: {}, lock_owner: {}, start: {}, \
        //     end: {}, typ: {}, pid: {})",
        //     ino, fh, lock_owner, start, end, typ, pid
        // );
        // reply.error(ENOSYS);
        self.fs
            .getlk(req, ino, fh, lock_owner, start, end, typ, pid, reply)
    }

    /// Acquire, modify or release a POSIX file lock.
    /// For POSIX threads (NPTL) there's a 1-1 relation between pid and owner, but
    /// otherwise this is not always the case.  For checking lock ownership,
    /// 'fi->owner' must be used. The l_pid field in 'struct flock' should only be
    /// used to fill in this field in getlk(). Note: if the locking methods are not
    /// implemented, the kernel will still allow file locking to work locally.
    /// Hence these are only interesting for network filesystems and similar.
    fn setlk(
        &mut self,
        req: &Request<'_>,
        ino: u64,
        fh: u64,
        lock_owner: u64,
        start: u64,
        end: u64,
        typ: i32,
        pid: u32,
        sleep: bool,
        reply: ReplyEmpty,
    ) {
        // debug!(
        //     "[Not Implemented] setlk(ino: {:#x?}, fh: {}, lock_owner: {}, start: {}, \
        //     end: {}, typ: {}, pid: {}, sleep: {})",
        //     ino, fh, lock_owner, start, end, typ, pid, sleep
        // );
        // reply.error(ENOSYS);
        self.fs
            .setlk(req, ino, fh, lock_owner, start, end, typ, pid, sleep, reply)
    }

    /// Map block index within file to block index within device.
    /// Note: This makes sense only for block device backed filesystems mounted
    /// with the 'blkdev' option
    fn bmap(&mut self, req: &Request<'_>, ino: u64, blocksize: u32, idx: u64, reply: ReplyBmap) {
        // debug!(
        //     "[Not Implemented] bmap(ino: {:#x?}, blocksize: {}, idx: {})",
        //     ino, blocksize, idx,
        // );
        // reply.error(ENOSYS);
        self.fs.bmap(req, ino, blocksize, idx, reply)
    }

    /// control device
    fn ioctl(
        &mut self,
        req: &Request<'_>,
        ino: u64,
        fh: u64,
        flags: u32,
        cmd: u32,
        in_data: &[u8],
        out_size: u32,
        reply: ReplyIoctl,
    ) {
        // debug!(
        //     "[Not Implemented] ioctl(ino: {:#x?}, fh: {}, flags: {}, cmd: {}, \
        //     in_data.len(): {}, out_size: {})",
        //     ino,
        //     fh,
        //     flags,
        //     cmd,
        //     in_data.len(),
        //     out_size,
        // );
        // reply.error(ENOSYS);
        self.fs
            .ioctl(req, ino, fh, flags, cmd, in_data, out_size, reply)
    }

    /// Poll for events
    fn poll(
        &mut self,
        req: &Request<'_>,
        ino: u64,
        fh: u64,
        kh: u64,
        events: u32,
        flags: u32,
        reply: ReplyPoll,
    ) {
        // debug!(
        //     "[Not Implemented] poll(ino: {:#x?}, fh: {}, kh: {}, events: {}, flags: {})",
        //     ino, fh, kh, events, flags
        // );
        // reply.error(ENOSYS);
        self.fs.poll(req, ino, fh, kh, events, flags, reply)
    }

    /// Preallocate or deallocate space to a file
    fn fallocate(
        &mut self,
        req: &Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        length: i64,
        mode: i32,
        reply: ReplyEmpty,
    ) {
        // debug!(
        //     "[Not Implemented] fallocate(ino: {:#x?}, fh: {}, offset: {}, \
        //     length: {}, mode: {})",
        //     ino, fh, offset, length, mode
        // );
        // reply.error(ENOSYS);
        self.fs.fallocate(req, ino, fh, offset, length, mode, reply)
    }

    /// Reposition read/write file offset
    fn lseek(
        &mut self,
        req: &Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        whence: i32,
        reply: ReplyLseek,
    ) {
        // debug!(
        //     "[Not Implemented] lseek(ino: {:#x?}, fh: {}, offset: {}, whence: {})",
        //     ino, fh, offset, whence
        // );
        // reply.error(ENOSYS);
        self.fs.lseek(req, ino, fh, offset, whence, reply)
    }

    /// Copy the specified range from the source inode to the destination inode
    fn copy_file_range(
        &mut self,
        req: &Request<'_>,
        ino_in: u64,
        fh_in: u64,
        offset_in: i64,
        ino_out: u64,
        fh_out: u64,
        offset_out: i64,
        len: u64,
        flags: u32,
        reply: ReplyWrite,
    ) {
        // debug!(
        //     "[Not Implemented] copy_file_range(ino_in: {:#x?}, fh_in: {}, \
        //     offset_in: {}, ino_out: {:#x?}, fh_out: {}, offset_out: {}, \
        //     len: {}, flags: {})",
        //     ino_in, fh_in, offset_in, ino_out, fh_out, offset_out, len, flags
        // );
        // reply.error(ENOSYS);
        self.fs.copy_file_range(
            req, ino_in, fh_in, offset_in, ino_out, fh_out, offset_out, len, flags, reply,
        )
    }

    /// macOS only: Rename the volume. Set fuse_init_out.flags during init to
    /// FUSE_VOL_RENAME to enable
    #[cfg(target_os = "macos")]
    fn setvolname(&mut self, req: &Request<'_>, name: &OsStr, reply: ReplyEmpty) {
        // debug!("[Not Implemented] setvolname(name: {:?})", name);
        // reply.error(ENOSYS);
        self.fs.setvolname(req, name, reply)
    }

    /// macOS only (undocumented)
    #[cfg(target_os = "macos")]
    fn exchange(
        &mut self,
        req: &Request<'_>,
        parent: u64,
        name: &OsStr,
        newparent: u64,
        newname: &OsStr,
        options: u64,
        reply: ReplyEmpty,
    ) {
        // debug!(
        //     "[Not Implemented] exchange(parent: {:#x?}, name: {:?}, newparent: {:#x?}, \
        //     newname: {:?}, options: {})",
        //     parent, name, newparent, newname, options
        // );
        // reply.error(ENOSYS);
        self.fs
            .exchange(req, parent, name, newparent, newname, options, reply)
    }

    /// macOS only: Query extended times (bkuptime and crtime). Set fuse_init_out.flags
    /// during init to FUSE_XTIMES to enable
    #[cfg(target_os = "macos")]
    fn getxtimes(&mut self, req: &Request<'_>, ino: u64, reply: ReplyXTimes) {
        // debug!("[Not Implemented] getxtimes(ino: {:#x?})", ino);
        // reply.error(ENOSYS);
        self.fs.getxtimes(req, ino, reply)
    }
}
