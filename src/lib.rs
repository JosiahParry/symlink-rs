//! A small, cross-platform crate for creating symlinks.
//!
//! For efficiency, you should prefer to use [`symlink_file`] or [`symlink_dir`]—whichever is
//! appropriate—rather than [`symlink_auto`].

// Building docs produces rustdoc::broken_intra_doc_links warnings on std::os::{windows, unix},
// depending on your platform. This is unfortunate because I then can’t RUSTDOCFLAGS="-D warnings"
// unless I suppress those, but suppressing those ones alone is messy, and suppressing all harmful,
// so I’m just leaving it at spurious square brackets being left in the output.

// It’s generally nicer to produce an empty crate on unsupported platforms than to explode.

use std::fs;
use std::io;
use std::path::Path;

#[cfg(windows)]
#[path = "windows/mod.rs"]
mod internal;

#[cfg(not(windows))]
mod internal {
    pub use std::fs::remove_file as remove_symlink_dir;
    pub use std::fs::remove_file as remove_symlink_auto;
    // Look, frankly, std::fs::soft_link and std::os::unix::fs::symlink call the same function,
    // so this probably whole separate mod probably isn’t even warranted.
    // But deprecated blah blah blah so I decided to use the std::os one anyway.
    #[cfg(unix)]
    pub use std::os::unix::fs::{symlink as symlink_auto,
                                symlink as symlink_file,
                                symlink as symlink_dir};
    #[cfg(not(unix))]
    // The compiler claims that std::fs::soft_link has been “replaced with
    // std::os::unix::fs::symlink and std::os::windows::fs::{symlink_file, symlink_dir}”
    // (rustc nightly 2021-12-26 deprecation warning message), but although that was true enough
    // when it was deprecated, it’s no longer quite true because of the wasm32-wasi target, which
    // supports symlinks through std::fs::soft_link but has no stable alternative (as I write,
    // std::os::wasi::fs::symlink_path is behind feature(wasi_ext)). Frankly, I think that’s a fair
    // (though imperfect) reason to *undeprecate* soft_link. Who knows what other platforms may in
    // the future stop returning std::io::ErrorKind::Unsupported errors and start supporting
    // std::fs::soft_link? (And for clarity, I note that no others do at the time of writing.)
    #[allow(deprecated)]
    pub use std::fs::{soft_link as symlink_auto,
                      soft_link as symlink_file,
                      soft_link as symlink_dir};
}

/// Create a symlink (non-preferred way).
///
/// On Windows, file and directory symlinks are created by distinct methods; to cope with that,
/// this function checks whether the destination is a file or a folder and creates the appropriate
/// type of symlink based on that result. Therefore, if the destination does not exist or if you do
/// not have permission to fetch its metadata, this will return an error on Windows.
///
/// On other platforms there is no distinction, so this isn’t magic: it’s precisely equivalent to
/// calling [`std::os::unix::fs::symlink`] or [`std::fs::soft_link`].
///
/// # A note on using this function
///
/// Because this is slightly less efficient and more hazardous on Windows, you should prefer to use
/// [`symlink_file`] or [`symlink_dir`] instead. Only use this if you don’t know or care whether
/// the destination is a file or a directory (but even then, you do need to know that it exists).
///
/// # Errors
///
/// An error will be returned if the symlink cannot be created, or—on Windows—if the destination
/// does not exist or cannot be read.
#[inline]
pub fn symlink_auto<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    internal::symlink_auto(src.as_ref(), dst.as_ref())
}

/// Create a symlink to a file.
///
/// On Windows, this is equivalent to [`std::os::windows::fs::symlink_file`]. If you call it with a
/// directory as the destination, [something may happen; you never know what][fow].
///
/// On Unix, this is equivalent to [`std::os::unix::fs::symlink`], and on other platforms it’s
/// equivalent to [`std::fs::soft_link`]. If you call it with a directory as the destination,
/// nothing bad will happen, but you’re ruining your cross-platform technique and ruining the point
/// of this crate, so please don’t.
///
/// # Errors
///
/// An error will be returned if the symlink cannot be created.
///
/// [fow]: https://en.wikipedia.org/wiki/A_Fish_Out_of_Water_(book)
#[inline]
pub fn symlink_file<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    internal::symlink_file(src.as_ref(), dst.as_ref())
}

/// Create a symlink to a directory.
///
/// On Windows, this is equivalent to [`std::os::windows::fs::symlink_dir`]. If you call it with a
/// directory as the destination, [something may happen; you never know what][fow].
///
/// On Unix, this is equivalent to [`std::os::unix::fs::symlink`], and on other platforms it’s
/// equivalent to [`std::fs::soft_link`]. If you call it with a directory as the destination,
/// nothing bad will happen, but you’re ruining your cross-platform technique and ruining the point
/// of this crate, so please don’t.
///
/// # Errors
///
/// An error will be returned if the symlink cannot be created.
///
/// [fow]: https://en.wikipedia.org/wiki/A_Fish_Out_of_Water_(book)
#[inline]
pub fn symlink_dir<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    internal::symlink_dir(src.as_ref(), dst.as_ref())
}

/// Remove a symlink (non-preferred way).
///
/// This inspects the path metadata to remove the symlink as a file or directory, whichever is
/// necessary.
///
/// # A note on using this function
///
/// Because this is slightly less efficient on Windows, you should prefer to use
/// [`remove_symlink_file`] or [`remove_symlink_dir`] instead. Only use this if you don’t know or
/// care whether the destination is a file or a directory (but even then, you do need to know that
/// it exists).
///
/// # Errors
///
/// An error will be returned if the symlink cannot be removed.
#[inline]
pub fn remove_symlink_auto<P: AsRef<Path>>(path: P) -> io::Result<()> {
    internal::remove_symlink_auto(path)
}

/// Remove a directory symlink.
///
/// On Windows, this corresponds to [`std::fs::remove_dir`].
///
/// On Unix, this corresponds to [`std::fs::remove_file`].
#[inline]
pub fn remove_symlink_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    internal::remove_symlink_dir(path)
}

/// Remove a file symlink.
///
/// This just calls [`std::fs::remove_file`], but the function is provided here to correspond to
/// [`remove_symlink_dir`].
#[inline]
pub fn remove_symlink_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fs::remove_file(path)
}
