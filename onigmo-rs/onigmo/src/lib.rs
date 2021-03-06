use onigmo_sys::*;
use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Error(OnigPosition, Option<OnigErrorInfo>, String);
type Result<T> = ::std::result::Result<T, Error>;

impl Error {
    fn new(pos: OnigPosition, error_info: Option<OnigErrorInfo>) -> Self {
        use std::str::from_utf8;
        let s: &mut [OnigUChar] = &mut [0; ONIG_MAX_ERROR_MESSAGE_LEN as usize];
        unsafe {
            let size = match error_info {
                Some(ei) => onig_error_code_to_str(s as *mut _ as *mut _, pos, ei),
                None => onig_error_code_to_str(s as *mut _ as *mut _, pos),
            };
            let size = size as usize;
            let s = from_utf8(&s[0..size]).unwrap().to_string();
            Error(pos, error_info, s)
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ERROR: {}\n", self.2)
    }
}

impl error::Error for Error {}

use std::mem;
use std::ops::Drop;

// 操作の主体
pub struct Regex(regex_t);
impl Regex {
    pub fn new(pattern: &str) -> Result<Self> {
        unsafe {
            let mut reg: regex_t = mem::uninitialized();
            let pattern = pattern.as_bytes();
            let mut einfo: OnigErrorInfo = mem::uninitialized();
            let r = onig_new_without_alloc(
                &mut reg as *mut _,
                pattern.as_ptr() as *const OnigUChar,
                (pattern.as_ptr() as *const OnigUChar).offset(pattern.len() as isize),
                ONIG_OPTION_NONE,
                &OnigEncodingUTF_8,
                OnigDefaultSyntax,
                &mut einfo,
            );
            if (r as ::std::os::raw::c_uint) == ONIG_NORMAL {
                Ok(Regex(reg))
            } else {
                Err(Error::new(r as OnigPosition, Some(einfo)))
            }
        }
    }

    // 検索対象はreadonly
    pub fn search(&mut self, s: &str) -> Option<Region> {
        unsafe {
            let s = s.as_bytes();
            let start = s.as_ptr();
            let end = start.offset(s.len() as isize);
            let range = end;
            let mut region = Region::new()?;

            let pos = onig_search(
                &mut self.0,
                start,
                end,
                start,
                range,
                region.as_ptr_mut(),
                ONIG_OPTION_NONE,
            );
            if 0 <= pos {
                Some(region)
            } else {
                debug_assert!(pos as std::os::raw::c_int == ONIG_MISMATCH);
                None
            }
        }
    }
}
impl Drop for Regex {
    fn drop(&mut self) {
        unsafe { onig_free_body(&mut self.0) }
    }
}

use std::ops::Range;
#[derive(Debug, Clone)]
pub struct PositionIter<'a>(&'a Region, Range<i32>);

impl<'a> Iterator for PositionIter<'a> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let region = *self.0.as_ptr();
            self.1.next().map(|i| {
                (
                    *region.beg.offset(i as isize) as usize,
                    *region.end.offset(i as isize) as usize,
                )
            })
        }
    }
}

use std::ptr::NonNull;
#[derive(Debug)]
pub struct Region(NonNull<OnigRegion>);

impl Region {
    pub fn new() -> Option<Self> {
        unsafe {
            let region: *mut OnigRegion = onig_region_new();
            Some(Region(NonNull::new(region)?))
        }
    }

    fn as_ptr_mut(&mut self) -> *mut OnigRegion {
        self.0.as_ptr()
    }

    fn as_ptr(&self) -> *mut OnigRegion {
        self.0.as_ptr()
    }

    pub fn position(&self) -> PositionIter {
        let num_regs;
        unsafe {
            num_regs = (*self.as_ptr()).num_regs;
        }
        PositionIter(self, 0..num_regs)
    }
}

impl Drop for Region {
    fn drop(&mut self) {
        unsafe { onig_region_free(self.0.as_ptr(), 1) }
    }
}

impl Clone for Region {
    fn clone(&self) -> Self {
        unsafe {
            let to: *mut OnigRegion = mem::uninitialized();
            onig_region_copy(to, self.0.as_ptr());
            Region(NonNull::new_unchecked(to))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
