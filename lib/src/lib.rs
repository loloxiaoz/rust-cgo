extern crate nom;
use nom::{
    bytes::complete::{is_not, take},
    combinator::{opt, peek},
    IResult,
};

#[no_mangle]
pub extern fn peek_head(input: &str) -> IResult<&str, &str> {
    let len = input.len();
    if len > 50 {
        return peek(take(50usize))(input);
    }
    peek(take(len))(input)
}

#[no_mangle]
pub extern fn space_find(input: &str) -> IResult<&str, &str> {
    let (char, found) = opt(is_not(" "))(input)?;
    if let Some(left) = found {
        return Ok((char.trim(), left));
    }
    Ok((input, ""))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peek_head() {
        let input = "hello world";
        let (old, head) = peek_head(input).unwrap();
        assert_eq!(old, "hello world");
        assert_eq!(head, "hello world");
    }

    #[test]
    fn test_space_find() {
        let input = "hello world";
        let (char, left) = space_find(input).unwrap();
        assert_eq!(char, "world");
        assert_eq!(left, "hello");
    }
}
