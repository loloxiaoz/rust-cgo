extern crate nom;
use nom::{
    bytes::complete::{is_not},
    combinator::{opt},
    IResult,
};
use std::os::raw::c_char;
use std::ffi::{CString, CStr};

#[repr(C)]
pub struct PairStr{
    left : *const c_char,
    right : *const c_char,
}

pub fn space_find(input: &str) -> IResult<&str, &str> {
    let (char, found) = opt(is_not(" "))(input)?;
    if let Some(left) = found {
        return Ok((char.trim(), left));
    }
    Ok((input, ""))
}

#[no_mangle]
pub extern fn space_find_export(input: *const c_char) -> PairStr {
    unsafe {
        let input_str: &str= CStr::from_ptr(input).to_str().unwrap();
        let (x, y) = space_find(input_str).unwrap();
        PairStr{
            left : CString::new(x).unwrap().into_raw(),
            right : CString::new(y).unwrap().into_raw(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_find() {
        let input_str = b"hello world\0";
        let input: *const c_char = input_str.as_ptr() as *const c_char;
        let ret = space_find_export(input);
        unsafe {
            assert_eq!(CStr::from_ptr(ret.left).to_str().unwrap(), "world");
            assert_eq!(CStr::from_ptr(ret.right).to_str().unwrap(), "hello");
        }
    }
}
