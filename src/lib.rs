use std::fmt;
use std::ffi::OsStr;
use std::ops::Deref;
use std::path::*;

// NB: Internal PathBuf must only contain utf8 data
#[repr(transparent)]
pub struct Utf8PathBuf(PathBuf);

impl Deref for Utf8PathBuf {
    type Target = Utf8Path;

    fn deref(&self) -> &Utf8Path {
        unsafe { Utf8Path::from_path(&*self.0) }
    }
}

// NB: Internal Path must only contain utf8 data
#[repr(transparent)]
pub struct Utf8Path(Path);

impl Utf8Path {
    pub fn new(s: &(impl AsRef<str> + ?Sized)) -> &Utf8Path {
        unsafe { Utf8Path::from_path(Path::new(s.as_ref())) }
    }

    pub fn as_str(&self) -> &str {
        unsafe { assert_utf8(self.0.as_os_str()) }
    }

    pub fn to_path_buf(&self) -> Utf8PathBuf {
        Utf8PathBuf(self.0.to_path_buf())
    }

    pub fn is_absolute(&self) -> bool {
        self.0.is_absolute()
    }

    pub fn is_relative(&self) -> bool {
        self.0.is_relative()
    }

    pub fn has_root(&self) -> bool {
        self.0.has_root()
    }

    pub fn parent(&self) -> Option<&Utf8Path> {
        self.0.parent().map(|path| unsafe { Utf8Path::from_path(path) })
    }

    pub fn ancestors(&self) -> Utf8Ancestors<'_> {
        Utf8Ancestors(self.0.ancestors())
    }

    pub fn file_name(&self) -> Option<&str> {
        self.0.file_name().map(|s| unsafe { assert_utf8(s) })
    }

    pub fn strip_prefix(&self, base: impl AsRef<Utf8Path>) -> Result<&Utf8Path, StripPrefixError> {
        self.0.strip_prefix(&base.as_ref().0).map(|path| unsafe { Utf8Path::from_path(path) })
    }

    pub fn starts_with(&self, base: impl AsRef<Utf8Path>) -> bool {
        self.0.starts_with(&base.as_ref().0)
    }

    pub fn ends_with(&self, base: impl AsRef<Utf8Path>) -> bool {
        self.0.ends_with(&base.as_ref().0)
    }

    pub fn file_stem(&self) -> Option<&str> {
        self.0.file_stem().map(|s| unsafe { assert_utf8(s) })
    }

    pub fn extension(&self) -> Option<&str> {
        self.0.extension().map(|s| unsafe { assert_utf8(s) })
    }

    pub fn join(&self, path: impl AsRef<Utf8Path>) -> Utf8PathBuf {
        Utf8PathBuf(self.0.join(&path.as_ref().0))
    }

    pub fn with_file_name(&self, file_name: impl AsRef<str>) -> Utf8PathBuf {
        Utf8PathBuf(self.0.with_file_name(file_name.as_ref()))
    }

    pub fn with_extension(&self, extension: impl AsRef<str>) -> Utf8PathBuf {
        Utf8PathBuf(self.0.with_extension(extension.as_ref()))
    }

    pub fn components(&self) -> Utf8Components {
        Utf8Components(self.0.components())
    }

    pub fn into_path_buf(self: Box<Utf8Path>) -> Utf8PathBuf {
        unsafe {
            Utf8PathBuf(Box::from_raw(Box::into_raw(self) as *mut Path).into_path_buf())
        }
    }

    // invariant: Path must be guaranteed to be utf-8 data
    unsafe fn from_path(path: &Path) -> &Utf8Path {
        &*(path as *const Path as *const Utf8Path)
    }
}

impl fmt::Display for Utf8Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

pub struct Utf8Ancestors<'a>(Ancestors<'a>);

pub struct Utf8Components<'a>(Components<'a>);

impl AsRef<Utf8Path> for Utf8Path {
    fn as_ref(&self) -> &Utf8Path {
        self
    }
}

impl AsRef<Utf8Path> for Utf8PathBuf {
    fn as_ref(&self) -> &Utf8Path {
        &**self
    }
}

impl AsRef<Utf8Path> for str {
    fn as_ref(&self) -> &Utf8Path {
        Utf8Path::new(self)
    }
}

impl AsRef<Utf8Path> for String {
    fn as_ref(&self) -> &Utf8Path {
        Utf8Path::new(self)
    }
}

impl AsRef<str> for Utf8Path {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<str> for Utf8PathBuf {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

// invariant: OsStr must be guaranteed to be utf8 data
unsafe fn assert_utf8(string: &OsStr) -> &str {
    &*(string as *const OsStr as *const str)
}
