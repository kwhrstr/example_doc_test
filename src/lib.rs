#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

///  ## find_char
/// found char pos of string
///
/// if found, then return Some position
///
/// else return None
///
/// ## Examples
/// ```
/// use example_doc_test::*;
/// let str = "abcde";
/// let result1 = find_char(str, 'b');
/// assert_eq!(result1, Some(1));
///
/// let result2 = find_char(str, 'f');
/// assert_eq!(result2, None)
/// ```
pub fn find_char(str :&str, c: char) -> Option<usize> {
    str.find(c)
}



