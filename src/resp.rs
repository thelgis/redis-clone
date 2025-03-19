use crate::resp_result::RESPError::OutOfBounds;
use crate::resp_result::{RESPError, RESPResult};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum RESP {
    SimpleString(String),
}

impl Display for RESP {
    // Display is preferred (compared to Into<String>) when the output string is to be viewed on a Screen
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let data = match self {
            Self::SimpleString(data) => format!("+{}\r\n", data),
        };
        write!(f, "{}", data)
    }
}

/// Return one of the parsing functions according to the initial character of the buffer
fn parse_router(
    buffer: &[u8],
    index: &mut usize,
) -> Option<fn(&[u8], &mut usize) -> RESPResult<RESP>> {
    match buffer[*index] {
        b'+' => Some(parse_simple_string),
        _ => None,
    }
}

pub fn bytes_to_resp(buffer: &[u8], index: &mut usize) -> RESPResult<RESP> {
    match parse_router(buffer, index) {
        Some(parse_function) => {
            let result = parse_function(buffer, index)?;
            Ok(result)
        }
        None => Err(RESPError::Unknown),
    }
}

fn parse_simple_string(buffer: &[u8], index: &mut usize) -> RESPResult<RESP> {
    resp_remove_type('+', buffer, index)?;

    let line = binary_extract_line_as_string(buffer, index)?;
    Ok(RESP::SimpleString(line))
}

pub fn binary_extract_line_as_string(buffer: &[u8], index: &mut usize) -> RESPResult<String> {
    let line = binary_extract_line(buffer, index)?;
    Ok(String::from_utf8(line)?)
}

/// Check first character of buffer is the expected one and remove it
pub fn resp_remove_type(value: char, buffer: &[u8], index: &mut usize) -> RESPResult<()> {
    if buffer[*index] != value as u8 {
        Err(RESPError::WrongType)
    } else {
        *index += 1;
        Ok(())
    }
}

fn binary_extract_line(buffer: &[u8], index: &mut usize) -> RESPResult<Vec<u8>> {
    // pretty low level buffer handling

    if *index >= buffer.len() {
        return Err(OutOfBounds(*index));
    }

    // If there is not enough space for 2 byte characters (i.e.  \r\n) the buffer is definitely invalid
    if buffer.len() - *index - 1 < 2 {
        *index = buffer.len();
        return Err(OutOfBounds(*index));
    }

    let mut previous_elem = buffer[*index].clone();
    let mut separator_found = false;
    let mut final_index = *index;

    for &element in buffer[*index..].iter() {
        final_index += 1;

        if previous_elem == b'\r' && element == b'\n' {
            separator_found = true;
            break;
        }
        previous_elem = element.clone();
    }

    if !separator_found {
        *index = final_index;
        return Err(OutOfBounds(*index));
    }

    // Copy the bytes from the buffer to the output vector
    let mut output = Vec::new();
    output.extend_from_slice(&buffer[*index..final_index - 2]);

    // Make sure the index is updated with the latest position
    *index = final_index;
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_extract_line_empty_buffer() {
        let buffer = "".as_bytes();
        let mut index: usize = 0;

        match binary_extract_line(buffer, &mut index) {
            Err(OutOfBounds(index)) => {
                assert_eq!(index, 0);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_binary_extract_line_single_character() {
        let buffer = "O".as_bytes();
        let mut index: usize = 0;

        match binary_extract_line(buffer, &mut index) {
            Err(OutOfBounds(index)) => {
                assert_eq!(index, 1);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_binary_extract_line_index_too_advanced() {
        let buffer = "OK".as_bytes();
        let mut index: usize = 1;

        match binary_extract_line(buffer, &mut index) {
            Err(OutOfBounds(index)) => {
                assert_eq!(index, 2);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_binary_extract_line_no_separator() {
        let buffer = "OK".as_bytes();
        let mut index: usize = 0;

        match binary_extract_line(buffer, &mut index) {
            Err(OutOfBounds(index)) => {
                assert_eq!(index, 2);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_binary_extract_line_half_separator() {
        let buffer = "OK\r".as_bytes();
        let mut index: usize = 0;

        match binary_extract_line(buffer, &mut index) {
            Err(OutOfBounds(index)) => {
                assert_eq!(index, 3);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_binary_extract_line_incorrect_separator() {
        let buffer = "OK\n".as_bytes();
        let mut index: usize = 0;

        match binary_extract_line(buffer, &mut index) {
            Err(OutOfBounds(index)) => {
                assert_eq!(index, 3);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_binary_extract_line() {
        let buffer = "OK\r\n".as_bytes();
        let mut index: usize = 0;
        let output = binary_extract_line(buffer, &mut index).unwrap();

        assert_eq!(output, "OK".as_bytes());
        assert_eq!(index, 4);
    }

    #[test]
    fn test_binary_extract_line_as_string() {
        let buffer = "OK\r\n".as_bytes();
        let mut index: usize = 0;
        let output = binary_extract_line_as_string(buffer, &mut index).unwrap();

        assert_eq!(output, String::from("OK"));
        assert_eq!(index, 4);
    }

    #[test]
    fn test_binary_remove_type() {
        let buffer = "+OK\r\n".as_bytes();
        let mut index: usize = 0;
        resp_remove_type('+', buffer, &mut index).unwrap();

        assert_eq!(index, 1);
    }

    #[test]
    fn test_binary_remove_type_error() {
        let buffer = "*OK\r\n".as_bytes();
        let mut index: usize = 0;
        let error = resp_remove_type('+', buffer, &mut index).unwrap_err();

        assert_eq!(index, 0);
        assert_eq!(error, RESPError::WrongType);
    }

    #[test]
    fn test_parse_simple_string() {
        let buffer = "+OK\r\n".as_bytes();
        let mut index: usize = 0;
        let output = parse_simple_string(buffer, &mut index).unwrap();

        assert_eq!(output, RESP::SimpleString(String::from("OK")));
        assert_eq!(index, 5);
    }

    #[test]
    fn test_bytes_to_resp_simple_string() {
        let buffer = "+OK\r\n".as_bytes();
        let mut index: usize = 0;
        let output = bytes_to_resp(buffer, &mut index).unwrap();

        assert_eq!(output, RESP::SimpleString(String::from("OK")));
        assert_eq!(index, 5);
    }
    #[test]
    fn test_bytes_to_resp_unknown() {
        let buffer = "?OK\r\n".as_bytes();
        let mut index: usize = 0;
        let error = bytes_to_resp(buffer, &mut index).unwrap_err();

        assert_eq!(error, RESPError::Unknown);
        assert_eq!(index, 0);
    }
}
