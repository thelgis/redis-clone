use crate::resp_result::RESPError::OutOfBounds;
use crate::resp_result::RESPResult;

pub fn binary_extract_line_as_string(buffer: &[u8], index: &mut usize) -> RESPResult<String> {
    let line = binary_extract_line(buffer, index)?;
    Ok(String::from_utf8(line)?)
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
}
