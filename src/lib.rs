use phf::phf_map;

mod ascii {
    // Uppercase letters
    pub const UPPERCASE_A: u8 = 0x41; // 'A'
    pub const UPPERCASE_Z: u8 = 0x5A; // 'Z'
    // Lowercase letters
    pub const LOWERCASE_A: u8 = 0x61; // 'a'
    pub const LOWERCASE_Z: u8 = 0x7A; // 'z'
    // Numbers (0-9)
    pub const ZERO: u8 = 0x30; // '0'
    pub const NINE: u8 = 0x39; // '9'
}

mod vk {
    // A helpful source: http://www.kbdedit.com/manual/low_level_vk_list.html
    pub const VK_SHIFT: u8 = 0x10; // Shift key
}

/// Errors that can occur when mapping a character to a key.
#[derive(PartialEq)]
#[derive(Debug)]
pub enum ErrorCodes {
    /// Was unable to find a matching key for the given character.
    NotFound,
    /// The given character was outside the valid ASCII character range.
    OutOfRange
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct KeyError {
    /// The byte being processed when the error occurred. Do not depend on this field as it can vary
    /// wildly because not all characters occupy a single byte in memory. Rather this should be used
    /// as a context clue to help determine the issue.
    pub byte: u8,
    /// Type of error that occurred.
    pub error_code: ErrorCodes
}

/// The offset in the ASCII character table from [a-z] and [A-Z].
/// Can be calculated by subtracting the code value of 'a' from 'A'.
const ASCII_LOWERCASE_TO_UPPERCASE_OFFSET_AND_ASCII_MIN_VALUE: u8 = 32;
const MAX_VALID_ASCII_CHAR_SIZE: u8 = 127;

static CHAR_TO_KEY_MAP: phf::Map<u8, u16> = phf_map! {
    // Shift not required
    0x2Au8 => 0x6A, // Asterisk -> VK_MULTIPLY
    0x2Du8 => 0x6D, // Minus -> VK_MINUS
    0x3Du8 => 0xBB, // Equal -> VK_OEM_PLUS
    0x2Eu8 => 0xBE, // Period -> VK_OME_PERIOD
    0x2Fu8 => 0x6F, // Slash -> VK_DIVIDE
    0x20u8 => 0x20, // Space -> VK_SPACE
    0x3Bu8 => 0xBA, // Semicolon -> VK_OEM_1
    0x60u8 => 0xC0, // Backtick -> VK_OEM_3
    0x5Bu8 => 0xDB, // Left Bracket -> VK_OEM_4
    0x5Cu8 => 0xDC, // Backslash -> VK_OEM_5
    0x5Du8 => 0xDD, // Right Bracket -> VK_OEM_6
    0x27u8 => 0xDE, // Single Quote -> VK_OEM_7
    0x2Cu8 => 0xBC, // Comma -> VK_OEM_COMMA

    // Shift required, left most bit of u8 is flag for shift
    // See: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-vkkeyscanexa#return-value
    0x21u8 => 0x0131, // Exclamation Mark -> VK_1
    0x40u8 => 0x0132, // At Sign -> VK_2
    0x23u8 => 0x0133, // Hash -> VK_3
    0x24u8 => 0x0134, // Dollar -> VK_4
    0x25u8 => 0x0135, // Percentage -> VK_5
    0x5Eu8 => 0x0136, // Caret -> VK_6
    0x26u8 => 0x0137, // Ampersand -> VK_7
    0x28u8 => 0x0139, // Left Paren -> VK_9
    0x29u8 => 0x0130, // Right Paren -> VK_0
    0x2Bu8 => 0x01BB, // Plus -> VK_OEM_PLUS
    0x3Au8 => 0x01BA, // Colon -> VK_OEM_1
    0x3Fu8 => 0x01BF, // Question Mark -> VK_OEM_2
    0x7Eu8 => 0x01C0, // Tilde -> VK_OEM_3
    0x7Bu8 => 0x01DB, // Left Curly Bracket -> VK_OEM_4
    0x7Du8 => 0x01DD, // Right Curly Bracket -> VK_OEM_5
    0x7Cu8 => 0x01DC, // Vertical Bar -> VK_OEM_5
    0x22u8 => 0x01DE, // Double Quote -> VK_OEM_7
    0x3Cu8 => 0x01BC, // Less Than -> VK_OEM_COMMA
    0x3Eu8 => 0x01BE, // Greater Than -> VK_OEM_PERIOD
    0x5Fu8 => 0x01BD, // Underscore -> VK_OEM_MINUS
};

// Function that returns a new Vec<u8>
pub fn to_keystrokes_new(keys: &str) -> Result<Vec<u8>, KeyError> {
    let mut keystrokes = Vec::new();
    to_keystrokes_mut(keys, &mut keystrokes)?;
    Ok(keystrokes)
}

pub fn to_keystrokes_mut(keys: &str, keystrokes: &mut Vec<u8>) -> Result<(), KeyError> {
    let characters = keys.as_bytes();
    let length = keys.len();

    let mut char;
    let mut is_shifting = false;
    let mut key_requires_shift: bool; // 0 is false, 1 is true with this var
    for i in 0..length {
        char = characters[i];

        // Ensure key value is within valid range
        if char >= MAX_VALID_ASCII_CHAR_SIZE || char < ASCII_LOWERCASE_TO_UPPERCASE_OFFSET_AND_ASCII_MIN_VALUE {
            return Err(KeyError{
                byte: char,
                error_code: ErrorCodes::OutOfRange
            })
        }

        // If the char is within [A-Z] push the value on as these values map directly to window's codes
        if char >= ascii::UPPERCASE_A && char <= ascii::UPPERCASE_Z {
            if !is_shifting {
                is_shifting = true;
                keystrokes.push(vk::VK_SHIFT);
            }
            keystrokes.push(char);
            continue // Skip
        }

        // If the char is within [a-z], offset to the uppercase codes and take the shift key into consideration
        if char >= ascii::LOWERCASE_A && char <= ascii::LOWERCASE_Z {
            if is_shifting { // We no longer need to be shifting
                is_shifting = false;
                keystrokes.push(vk::VK_SHIFT);
            }
            keystrokes.push(char - ASCII_LOWERCASE_TO_UPPERCASE_OFFSET_AND_ASCII_MIN_VALUE);
            continue // Skip
        }

        // If the char is within [0-9]
        if char >= ascii::ZERO && char <= ascii::NINE {
            if is_shifting { // We no longer need to be shifting
                is_shifting = false;
                keystrokes.push(vk::VK_SHIFT);
            }
            keystrokes.push(char); // ASCII [0-9] maps directly to window's virtual key code values for [0-9]
            continue
        }

        if let Some(code) = CHAR_TO_KEY_MAP.get(&char) {
            let vkc = code & 0xFF;
            let high_bits = code >> 0x8;

            /*
            VK_0: 0x0130 // KEY_0 with shift key flag
            0x0130 is 304 in decimal

            Decimal Perspective:
            304 / 256 gives us 1 (int math)(mod)
            304 % 256 just gives us 48 which is 0x30 (vk)

            ** I guess as long as the right hand (denominator) doesn't fit into the left hand (nominator) more than once, subtraction works too, 304 - 256 == 48 (vk)

            Hex Perspective:
            0x130 / 0x100 us 0x01 (mod)
            0x130 % 0x100 is 0x30 (vk)

            Binary Bit Shfit Perspective:
            0000 0001 0011 0000 & 1111 1111 => 0000 0000 0011 0000 = 48 (vk)
            0000 0001 0011 0000 >> 8 => 0000 0001 => 1 (mod)
             */

            // Do not bother masking to view only the shift aspect as that is all we support anyway
            key_requires_shift = high_bits == 0x1u16;
            if is_shifting && !key_requires_shift { // We no longer need to be shifting
                is_shifting = false;
                keystrokes.push(vk::VK_SHIFT);
            }
            else if !is_shifting && key_requires_shift { // We need to start shifting
                is_shifting = true;
                keystrokes.push(vk::VK_SHIFT);
            }
            // Isolate the key code form the shift and push
            keystrokes.push(vkc as u8);
            continue
        }

        return Err(KeyError {
            byte: char,
            error_code: ErrorCodes::NotFound
        })
    }

    // Add a trailing shift if needed
    // Occurs when last character as [a-z]
    if is_shifting { keystrokes.push(vk::VK_SHIFT); }

    Ok(())
}



/* ### --- UNIT TEST --- ### */



#[cfg(test)]
mod tests {
    use super::*;

    // Define constants for A-Z keys and the Shift key
    // Defining as u8 for now instead of u16 as not supporting extended windows virtual keys
    pub const VK_A: u8 = 0x41;  // 'A'
    pub const VK_B: u8 = 0x42;  // 'B'
    pub const VK_C: u8 = 0x43;  // 'C'
    pub const VK_Z: u8 = 0x5A;  // 'Z'
    pub const VK_0: u8 = 0x30; // 0 or )
    pub const VK_1: u8 = 0x31; // 1 or @
    pub const VK_2: u8 = 0x32; // 2 or @
    pub const VK_3: u8 = 0x33; // 3 or #
    pub const VK_4: u8 = 0x34; // 4 or $
    pub const VK_5: u8 = 0x35; // 5 or %
    pub const VK_6: u8 = 0x36; // 6 or ^
    pub const VK_7: u8 = 0x37; // 7 or &
    pub const VK_8: u8 = 0x38; // 8 or *
    pub const VK_9: u8 = 0x39; // 9 or (

    pub const VK_SPACE: u8 = 0x20; // Space key
    pub const VK_MULTIPLY: u8 = 0x6A; // Virtual Key for Multiply (*)
    pub const VK_MINUS: u8 = 0x6D; // Virtual Key for Minus (-)
    pub const VK_DIVIDE: u8 = 0x6F; // Virtual Key for Slash (/)

    pub const VK_OEM_1: u8 = 0xBA; // OEM 1 - For semicolon (;) and colon (:) on QWERTY keyboards
    pub const VK_OEM_PLUS: u8 = 0xBB; // OEM + - For plus (+) and equals (=)
    pub const VK_OEM_MINUS: u8 = 0xBD; // OEM - + For minus (-) and underscore (_)
    pub const VK_OEM_COMMA: u8 = 0xBC; // OEM , - For comma (,) and less than (<)
    pub const VK_OEM_PERIOD: u8 = 0xBE; // OEM . - For period (.) and greater than (>)
    pub const VK_OEM_2: u8 = 0xBF; // OEM 2 - For forward slash (/) and question mark (?)
    pub const VK_OEM_3: u8 = 0xC0; // OEM 3 - For grave accent (`) and tilde (~)
    pub const VK_OEM_4: u8 = 0xDB; // OEM 4 - For left bracket ([) and open square bracket
    pub const VK_OEM_5: u8 = 0xDC; // OEM 5 - For backslash (\) and pipe (|)
    pub const VK_OEM_6: u8 = 0xDD; // OEM 6 - For right bracket (]) and closing square bracket
    pub const VK_OEM_7: u8 = 0xDE; // OEM 7 - For single quote (') and double quote (")

    #[test]
    fn test_non_valid_character_results_in_error() {
        let strokes = to_keystrokes_new("æ");

        assert_eq!(strokes.is_err(), true, "should contain error from bounds check");
        // Do not check the byte field as it can vary depending on character width
        let err_code = strokes.unwrap_err().error_code;
        assert_eq!(err_code, ErrorCodes::OutOfRange, "character provided exceeds valid ASCII character range");
    }

    mod alphabetical {
        use super::*;


        #[test]
        fn test_lowercase_a() {
            let strokes = to_keystrokes_new("a").unwrap();

            assert_eq!(strokes.len(), 1, "incorrect key vector length");
            assert_eq!(strokes[0], VK_A);
        }

        #[test]
        fn test_lowercase_z() {
            let strokes = to_keystrokes_new("z").unwrap();

            assert_eq!(strokes.len(), 1, "incorrect key vector length");
            assert_eq!(strokes[0], VK_Z);
        }

        #[test]
        fn test_uppercase_a() {
            let strokes = to_keystrokes_new("A").unwrap();

            assert_eq!(strokes.len(), 3, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_SHIFT);
            assert_eq!(strokes[1], VK_A);
            assert_eq!(strokes[2], vk::VK_SHIFT);
        }

        #[test]
        fn test_uppercase_z() {
            let strokes = to_keystrokes_new("Z").unwrap();

            assert_eq!(strokes.len(), 3, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_SHIFT);
            assert_eq!(strokes[1], VK_Z);
            assert_eq!(strokes[2], vk::VK_SHIFT);
        }

        #[test]
        fn test_multiple_lowercases() {
            let strokes = to_keystrokes_new("abc").unwrap();

            assert_eq!(strokes.len(), 3, "incorrect key vector length");
            assert_eq!(strokes[0], VK_A);
            assert_eq!(strokes[1], VK_B);
            assert_eq!(strokes[2], VK_C);
        }

        #[test]
        fn test_multiple_uppercases() {
            let strokes = to_keystrokes_new("ABC").unwrap();

            assert_eq!(strokes.len(), 5, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_SHIFT);
            assert_eq!(strokes[1], VK_A);
            assert_eq!(strokes[2], VK_B);
            assert_eq!(strokes[3], VK_C);
            assert_eq!(strokes[4], vk::VK_SHIFT);
        }

        #[test]
        fn test_multiple_mixed_cases_v1() {
            let strokes = to_keystrokes_new("aBc").unwrap();

            assert_eq!(strokes.len(), 5, "incorrect key vector length");
            assert_eq!(strokes[0], VK_A);
            assert_eq!(strokes[1], vk::VK_SHIFT);
            assert_eq!(strokes[2], VK_B);
            assert_eq!(strokes[3], vk::VK_SHIFT);
            assert_eq!(strokes[4], VK_C);
        }

        #[test]
        fn test_multiple_mixed_cases_v2() {
            let strokes = to_keystrokes_new("abcABCabc").unwrap();

            assert_eq!(strokes.len(), 11, "incorrect key vector length");
            assert_eq!(strokes[0], VK_A);
            assert_eq!(strokes[1], VK_B);
            assert_eq!(strokes[2], VK_C);

            assert_eq!(strokes[3], vk::VK_SHIFT);
            assert_eq!(strokes[4], VK_A);
            assert_eq!(strokes[5], VK_B);
            assert_eq!(strokes[6], VK_C);
            assert_eq!(strokes[7], vk::VK_SHIFT);

            assert_eq!(strokes[8], VK_A);
            assert_eq!(strokes[9], VK_B);
            assert_eq!(strokes[10], VK_C);
        }

        #[test]
        fn test_multiple_mixed_cases_v3() {
            let strokes = to_keystrokes_new("ZbcAzCaZc").unwrap();

            assert_eq!(strokes.len(), 17, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_SHIFT);
            assert_eq!(strokes[1], VK_Z);
            assert_eq!(strokes[2], vk::VK_SHIFT);

            assert_eq!(strokes[3], VK_B);
            assert_eq!(strokes[4], VK_C);

            assert_eq!(strokes[5], vk::VK_SHIFT);
            assert_eq!(strokes[6], VK_A);
            assert_eq!(strokes[7], vk::VK_SHIFT);

            assert_eq!(strokes[8], VK_Z);

            assert_eq!(strokes[9], vk::VK_SHIFT);
            assert_eq!(strokes[10], VK_C);
            assert_eq!(strokes[11], vk::VK_SHIFT);

            assert_eq!(strokes[12], VK_A);

            assert_eq!(strokes[13], vk::VK_SHIFT);
            assert_eq!(strokes[14], VK_Z);
            assert_eq!(strokes[15], vk::VK_SHIFT);

            assert_eq!(strokes[16], VK_C);
        }
    }

    mod vk_0_through_vk_9 {
        use super::*;

        #[test]
        fn test_number_one() {
            let strokes = to_keystrokes_new("1").unwrap();

            assert_eq!(strokes.len(), 1, "incorrect key vector length");
            assert_eq!(strokes[0], VK_1);
        }

        #[test]
        fn test_exclamation_mark() {
            let strokes = to_keystrokes_new("!").unwrap();

            assert_eq!(strokes.len(), 3, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_SHIFT);
            assert_eq!(strokes[1], VK_1);
            assert_eq!(strokes[2], vk::VK_SHIFT);
        }

        #[test]
        fn test_number_two() {
            let strokes = to_keystrokes_new("2").unwrap();

            assert_eq!(strokes.len(), 1, "incorrect key vector length");
            assert_eq!(strokes[0], VK_2);
        }

        #[test]
        fn test_at_sign() {
            let strokes = to_keystrokes_new("@").unwrap();

            assert_eq!(strokes.len(), 3, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_SHIFT);
            assert_eq!(strokes[1], VK_2);
            assert_eq!(strokes[2], vk::VK_SHIFT);
        }

        #[test]
        fn test_number_three() {
            let strokes = to_keystrokes_new("3").unwrap();

            assert_eq!(strokes.len(), 1, "incorrect key vector length");
            assert_eq!(strokes[0], VK_3);
        }

        #[test]
        fn test_hash() {
            let strokes = to_keystrokes_new("#").unwrap();

            assert_eq!(strokes.len(), 3, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_SHIFT);
            assert_eq!(strokes[1], VK_3);
            assert_eq!(strokes[2], vk::VK_SHIFT);
        }

        #[test]
        fn test_number_four() {
            let strokes = to_keystrokes_new("4").unwrap();

            assert_eq!(strokes.len(), 1, "incorrect key vector length");
            assert_eq!(strokes[0], VK_4);
        }

        #[test]
        fn test_dollar_sign() {
            let strokes = to_keystrokes_new("$").unwrap();

            assert_eq!(strokes.len(), 3, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_SHIFT);
            assert_eq!(strokes[1], VK_4);
            assert_eq!(strokes[2], vk::VK_SHIFT);
        }

        #[test]
        fn test_number_five() {
            let strokes = to_keystrokes_new("5").unwrap();

            assert_eq!(strokes.len(), 1, "incorrect key vector length");
            assert_eq!(strokes[0], VK_5);
        }

        #[test]
        fn test_percent() {
            let strokes = to_keystrokes_new("%").unwrap();

            assert_eq!(strokes.len(), 3, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_SHIFT);
            assert_eq!(strokes[1], VK_5);
            assert_eq!(strokes[2], vk::VK_SHIFT);
        }

        #[test]
        fn test_number_six() {
            let strokes = to_keystrokes_new("6").unwrap();

            assert_eq!(strokes.len(), 1, "incorrect key vector length");
            assert_eq!(strokes[0], VK_6);
        }

        #[test]
        fn test_caret() {
            let strokes = to_keystrokes_new("^").unwrap();

            assert_eq!(strokes.len(), 3, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_SHIFT);
            assert_eq!(strokes[1], VK_6);
            assert_eq!(strokes[2], vk::VK_SHIFT);
        }

        #[test]
        fn test_seven() {
            let strokes = to_keystrokes_new("7").unwrap();

            assert_eq!(strokes.len(), 1, "incorrect key vector length");
            assert_eq!(strokes[0], VK_7);
        }

        #[test]
        fn test_ampersand() {
            let strokes = to_keystrokes_new("&").unwrap();

            assert_eq!(strokes.len(), 3, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_SHIFT);
            assert_eq!(strokes[1], VK_7);
            assert_eq!(strokes[2], vk::VK_SHIFT);
        }

        #[test]
        fn test_number_eight() {
            let strokes = to_keystrokes_new("8").unwrap();

            assert_eq!(strokes.len(), 1, "incorrect key vector length");
            assert_eq!(strokes[0], VK_8);
        }

        #[test]
        fn test_number_9() {
            let strokes = to_keystrokes_new("9").unwrap();

            assert_eq!(strokes.len(), 1, "incorrect key vector length");
            assert_eq!(strokes[0], VK_9);
        }

        #[test]
        fn test_left_paren() {
            let strokes = to_keystrokes_new("(").unwrap();

            assert_eq!(strokes.len(), 3, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_SHIFT);
            assert_eq!(strokes[1], VK_9);
            assert_eq!(strokes[2], vk::VK_SHIFT);
        }

        #[test]
        fn test_number_zero() {
            let strokes = to_keystrokes_new("0").unwrap();

            assert_eq!(strokes.len(), 1, "incorrect key vector length");
            assert_eq!(strokes[0], VK_0);
        }

        #[test]
        fn test_right_paren() {
            let strokes = to_keystrokes_new(")").unwrap();

            assert_eq!(strokes.len(), 3, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_SHIFT);
            assert_eq!(strokes[1], VK_0);
            assert_eq!(strokes[2], vk::VK_SHIFT);
        }
    }

    #[test]
    fn test_multiply() {
        let strokes = to_keystrokes_new("*").unwrap();

        assert_eq!(strokes.len(), 1, "incorrect key vector length");
        assert_eq!(strokes[0], VK_MULTIPLY);
    }

    #[test]
    fn test_plus() {
        let strokes = to_keystrokes_new("+").unwrap();

        assert_eq!(strokes.len(), 3, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_SHIFT);
        assert_eq!(strokes[1], VK_OEM_PLUS);
        assert_eq!(strokes[2], vk::VK_SHIFT);
    }

    #[test]
    fn test_minus() {
        let strokes = to_keystrokes_new("-").unwrap();

        assert_eq!(strokes.len(), 1, "incorrect key vector length");
        assert_eq!(strokes[0], VK_MINUS);
    }

    #[test]
    fn test_comma() {
        let strokes = to_keystrokes_new(",").unwrap();

        assert_eq!(strokes.len(), 1, "incorrect key vector length");
        assert_eq!(strokes[0], VK_OEM_COMMA);
    }

    #[test]
    fn test_divide() {
        let strokes = to_keystrokes_new("/").unwrap();

        assert_eq!(strokes.len(), 1, "incorrect key vector length");
        assert_eq!(strokes[0], VK_DIVIDE);
    }

    #[test]
    fn test_period() {
        let strokes = to_keystrokes_new(".").unwrap();

        assert_eq!(strokes.len(), 1, "incorrect key vector length");
        assert_eq!(strokes[0], VK_OEM_PERIOD);
    }

    #[test]
    fn test_white_space() {
        let strokes = to_keystrokes_new(" ").unwrap();

        assert_eq!(strokes.len(), 1, "incorrect key vector length");
        assert_eq!(strokes[0], VK_SPACE);
    }

    #[test]
    fn test_semi_colon() {
        let strokes = to_keystrokes_new(";").unwrap();

        assert_eq!(strokes.len(), 1, "incorrect key vector length");
        assert_eq!(strokes[0], VK_OEM_1);
    }

    #[test]
    fn test_colon() {
        let strokes = to_keystrokes_new(":").unwrap();

        assert_eq!(strokes.len(), 3, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_SHIFT);
        assert_eq!(strokes[1], VK_OEM_1);
        assert_eq!(strokes[2], vk::VK_SHIFT);
    }

    #[test]
    fn test_question_mark() {
        let strokes = to_keystrokes_new("?").unwrap();

        assert_eq!(strokes.len(), 3, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_SHIFT);
        assert_eq!(strokes[1], VK_OEM_2);
        assert_eq!(strokes[2], vk::VK_SHIFT);
    }

    #[test]
    fn test_backtick() {
        let strokes = to_keystrokes_new("`").unwrap();

        assert_eq!(strokes.len(), 1, "incorrect key vector length");
        assert_eq!(strokes[0], VK_OEM_3);
    }

    #[test]
    fn test_tilde() {
        let strokes = to_keystrokes_new("~").unwrap();

        assert_eq!(strokes.len(), 3, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_SHIFT);
        assert_eq!(strokes[1], VK_OEM_3);
        assert_eq!(strokes[2], vk::VK_SHIFT);
    }

    #[test]
    fn test_left_bracket() {
        let strokes = to_keystrokes_new("[").unwrap();

        assert_eq!(strokes.len(), 1, "incorrect key vector length");
        assert_eq!(strokes[0], VK_OEM_4);
    }

    #[test]
    fn test_left_curly_bracket() {
        let strokes = to_keystrokes_new("{").unwrap();

        assert_eq!(strokes.len(), 3, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_SHIFT);
        assert_eq!(strokes[1], VK_OEM_4);
        assert_eq!(strokes[2], vk::VK_SHIFT);
    }

    #[test]
    fn test_backslash() {
        let strokes = to_keystrokes_new("\\").unwrap();

        assert_eq!(strokes.len(), 1, "incorrect key vector length");
        assert_eq!(strokes[0], VK_OEM_5);
    }

    #[test]
    fn test_vertical_bar() {
        let strokes = to_keystrokes_new("|").unwrap();

        assert_eq!(strokes.len(), 3, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_SHIFT);
        assert_eq!(strokes[1], VK_OEM_5);
        assert_eq!(strokes[2], vk::VK_SHIFT);
    }

    #[test]
    fn test_right_bracket() {
        let strokes = to_keystrokes_new("]").unwrap();

        assert_eq!(strokes.len(), 1, "incorrect key vector length");
        assert_eq!(strokes[0], VK_OEM_6);
    }

    #[test]
    fn test_right_curly_bracket() {
        let strokes = to_keystrokes_new("}").unwrap();

        assert_eq!(strokes.len(), 3, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_SHIFT);
        assert_eq!(strokes[1], VK_OEM_6);
        assert_eq!(strokes[2], vk::VK_SHIFT);
    }

    #[test]
    fn test_single_quote() {
        let strokes = to_keystrokes_new("'").unwrap();

        assert_eq!(strokes.len(), 1, "incorrect key vector length");
        assert_eq!(strokes[0], VK_OEM_7);
    }

    #[test]
    fn test_double_quote() {
        let strokes = to_keystrokes_new("\"").unwrap();

        assert_eq!(strokes.len(), 3, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_SHIFT);
        assert_eq!(strokes[1], VK_OEM_7);
        assert_eq!(strokes[2], vk::VK_SHIFT);
    }

    #[test]
    fn test_less_than() {
        let strokes = to_keystrokes_new("<").unwrap();

        assert_eq!(strokes.len(), 3, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_SHIFT);
        assert_eq!(strokes[1], VK_OEM_COMMA);
        assert_eq!(strokes[2], vk::VK_SHIFT);
    }

    #[test]
    fn test_greater_than() {
        let strokes = to_keystrokes_new(">").unwrap();

        assert_eq!(strokes.len(), 3, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_SHIFT);
        assert_eq!(strokes[1], VK_OEM_PERIOD);
        assert_eq!(strokes[2], vk::VK_SHIFT);
    }

    #[test]
    fn test_equal() {
        let strokes = to_keystrokes_new("=").unwrap();

        assert_eq!(strokes.len(), 1, "incorrect key vector length");
        assert_eq!(strokes[0], VK_OEM_PLUS);
    }

    #[test]
    fn test_underscore() {
        let strokes = to_keystrokes_new("_").unwrap();

        assert_eq!(strokes.len(), 3, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_SHIFT);
        assert_eq!(strokes[1], VK_OEM_MINUS);
        assert_eq!(strokes[2], vk::VK_SHIFT);
    }
}