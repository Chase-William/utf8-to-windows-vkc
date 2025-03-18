use std::collections::hash_map::Keys;
use std::collections::HashMap;

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

    // Special characters
    pub const SPACE: u8 = 0x20; // Space ( )
    pub const EXCLAMATION_MARK: u8 = 0x21; // Exclamation Mark (!)
    pub const DOUBLE_QUOTE: u8 = 0x22; // Double Quote (")
    pub const SINGLE_QUOTE: u8 = 0x27; // Single Quote (')
    pub const HASH: u8 = 0x23; // Hash (#)
    pub const DOLLAR: u8 = 0x24; // Dollar Sign ($)
    pub const PERCENT: u8 = 0x25; // Percent (%)
    pub const AMPERSAND: u8 = 0x26; // Ampersand (&)
    pub const SMALL_QUOTE: u8 = 0x27; // Single Quote (')
    pub const LEFT_PAREN: u8 = 0x28; // Left Parenthesis (
    pub const RIGHT_PAREN: u8 = 0x29; // Right Parenthesis )
    pub const ASTERISK: u8 = 0x2A; // Asterisk (*)
    pub const PLUS: u8 = 0x2B; // Plus (+)
    pub const COMMA: u8 = 0x2C; // Comma (,)
    pub const MINUS: u8 = 0x2D; // Minus (-)
    pub const EQUAL: u8 = 0x3D; // Equal (=)
    pub const PERIOD: u8 = 0x2E; // Period (.)
    pub const SLASH: u8 = 0x2F; // Slash (/)
    pub const AT_SIGN: u8 = 0x40; // At-Sign (@)
    pub const CARET: u8 = 0x5E; // Caret (^)
    pub const COLON: u8 = 0x3A; // Colon (:)
    pub const SEMICOLON: u8 = 0x3B; // Semicolon (;)
    pub const BACKTICK: u8 = 0x60; // Backtick (`)
    pub const TILDE: u8 = 0x7E; // Tilde (~)
    pub const LEFT_BRACKET: u8 = 0x5B; // Left bracket ([)
    pub const LEFT_CURLY_BRACKET: u8 = 0x7B; // Left curly bracket ({)
    pub const RIGHT_BRACKET: u8 = 0x5D; // Right bracket (])
    pub const RIGHT_CURLY_BRACKET: u8 = 0x7D; // Right curly bracket (})
    pub const BACKSLASH: u8 = 0x5C; // Backslash (\)
    pub const VERTICAL_BAR: u8 = 0x7C; // Vertical bar (|)
    pub const LESS_THAN: u8 = 0x3C; // Less than (<)
    pub const GREATER_THAN: u8 = 0x3E; // Greater than (>)

    // Hexadecimal values corresponding to OEM virtual key codes
    pub const UNDERSCORE: u8 = 0x5F; // ASCII for underscore (_)
    pub const QUESTION_MARK: u8 = 0x3F; // ASCII for question mark (?)
    pub const GRAVE_ACCENT: u8 = 0x60; // ASCII for grave accent (`)
}


mod vk {
    // A helpful source: http://www.kbdedit.com/manual/low_level_vk_list.html
    use std::collections::HashMap;
    use crate::{ascii, vk};

    pub const VK_SHIFT: u8 = 0x10; // Shift key
    pub const VK_SPACE: u8 = 0x20; // Space key

    // Key codes for [0-9] keys that are shift-able to produce other characters
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

    // Define constants for A-Z keys and the Shift key
    // Defining as u8 for now instead of u16 as not supporting extended windows virtual keys
    pub const VK_A: u8 = 0x41;  // 'A'
    pub const VK_B: u8 = 0x42;  // 'B'
    pub const VK_C: u8 = 0x43;  // 'C'
    pub const VK_D: u8 = 0x44;  // 'D'
    pub const VK_E: u8 = 0x45;  // 'E'
    pub const VK_F: u8 = 0x46;  // 'F'
    pub const VK_G: u8 = 0x47;  // 'G'
    pub const VK_H: u8 = 0x48;  // 'H'
    pub const VK_I: u8 = 0x49;  // 'I'
    pub const VK_J: u8 = 0x4A;  // 'J'
    pub const VK_K: u8 = 0x4B;  // 'K'
    pub const VK_L: u8 = 0x4C;  // 'L'
    pub const VK_M: u8 = 0x4D;  // 'M'
    pub const VK_N: u8 = 0x4E;  // 'N'
    pub const VK_O: u8 = 0x4F;  // 'O'
    pub const VK_P: u8 = 0x50;  // 'P'
    pub const VK_Q: u8 = 0x51;  // 'Q'
    pub const VK_R: u8 = 0x52;  // 'R'
    pub const VK_S: u8 = 0x53;  // 'S'
    pub const VK_T: u8 = 0x54;  // 'T'
    pub const VK_U: u8 = 0x55;  // 'U'
    pub const VK_V: u8 = 0x56;  // 'V'
    pub const VK_W: u8 = 0x57;  // 'W'
    pub const VK_X: u8 = 0x58;  // 'X'
    pub const VK_Y: u8 = 0x59;  // 'Y'
    pub const VK_Z: u8 = 0x5A;  // 'Z'

    pub const VK_MULTIPLY: u8 = 0x6A; // Virtual Key for Multiply (*)
    pub const VK_MINUS: u8 = 0x6D; // Virtual Key for Minus (-)
    pub const VK_DIVIDE: u8 = 0x6F; // Virtual Key for Slash (/)

    pub const VK_OEM_1: u8 = 0xBA; // OEM 1 - For semicolon (;) and colon (:) on QWERTY keyboards
    pub const VK_OEM_PLUS: u8 = 0xBB; // OEM + - For plus (+) and equals (=)
    pub const VK_OEM_COMMA: u8 = 0xBC; // OEM , - For comma (,) and less than (<)
    pub const VK_OEM_PERIOD: u8 = 0xBE; // OEM . - For period (.) and greater than (>)
    pub const VK_OEM_2: u8 = 0xBF; // OEM 2 - For forward slash (/) and question mark (?)
    pub const VK_OEM_3: u8 = 0xC0; // OEM 3 - For grave accent (`) and tilde (~)
    pub const VK_OEM_4: u8 = 0xDB; // OEM 4 - For left bracket ([) and open square bracket
    pub const VK_OEM_5: u8 = 0xDC; // OEM 5 - For backslash (\) and pipe (|)
    pub const VK_OEM_6: u8 = 0xDD; // OEM 6 - For right bracket (]) and closing square bracket
    pub const VK_OEM_7: u8 = 0xDE; // OEM 7 - For single quote (') and double quote (")
    pub const VK_OEM_8: u8 = 0xDF; // OEM 8 - Often used for miscellaneous keys, like a non-specific or unused key
}

/// The offset in the ASCII character table from [a-z] and [A-Z].
/// Can be calculated by subtracting the code value of 'a' from 'A'.
const ASCII_LOWERCASE_TO_UPPERCASE_OFFSET: u8 = 32;

struct KeyInfo {
    code: u8,
    requires_shift: bool
}

pub struct AsciiToVkApi {
    character_map: HashMap::<u8, KeyInfo>
}

impl AsciiToVkApi {
    pub fn new() -> AsciiToVkApi {
        let mut character_map = HashMap::<u8, KeyInfo>::new();

        // Maps that do not require shift key
        character_map.insert(ascii::ASTERISK, KeyInfo { code: vk::VK_MULTIPLY, requires_shift: false });
        character_map.insert(ascii::MINUS, KeyInfo { code: vk::VK_MINUS, requires_shift: false });
        character_map.insert(ascii::EQUAL, KeyInfo { code: vk::VK_OEM_PLUS, requires_shift: false });
        character_map.insert(ascii::PERIOD, KeyInfo { code: vk::VK_OEM_PERIOD, requires_shift: false });
        character_map.insert(ascii::SLASH, KeyInfo { code: vk::VK_DIVIDE, requires_shift: false });
        character_map.insert(ascii::SPACE, KeyInfo { code: vk::VK_SPACE, requires_shift: false });

        character_map.insert(ascii::SEMICOLON, KeyInfo { code: vk::VK_OEM_1, requires_shift: false });
        character_map.insert(ascii::BACKTICK, KeyInfo { code: vk::VK_OEM_3, requires_shift: false });
        character_map.insert(ascii::LEFT_BRACKET, KeyInfo { code: vk::VK_OEM_4, requires_shift: false });
        character_map.insert(ascii::BACKSLASH, KeyInfo { code: vk::VK_OEM_5, requires_shift: false });
        character_map.insert(ascii::RIGHT_BRACKET, KeyInfo { code: vk::VK_OEM_6, requires_shift: false });
        character_map.insert(ascii::SINGLE_QUOTE, KeyInfo { code: vk::VK_OEM_7, requires_shift: false });
        character_map.insert(ascii::COMMA, KeyInfo { code: vk::VK_OEM_COMMA, requires_shift: false });

        // Maps that require shift
        character_map.insert(ascii::EXCLAMATION_MARK, KeyInfo { code: vk::VK_1, requires_shift: true });
        character_map.insert(ascii::AT_SIGN, KeyInfo { code: vk::VK_2, requires_shift: true });
        character_map.insert(ascii::HASH, KeyInfo { code: vk::VK_3, requires_shift: true });
        character_map.insert(ascii::DOLLAR, KeyInfo { code: vk::VK_4, requires_shift: true });
        character_map.insert(ascii::PERCENT, KeyInfo { code: vk::VK_5, requires_shift: true });
        character_map.insert(ascii::CARET, KeyInfo { code: vk::VK_6, requires_shift: true });
        character_map.insert(ascii::AMPERSAND, KeyInfo { code: vk::VK_7, requires_shift: true });
        character_map.insert(ascii::LEFT_PAREN, KeyInfo { code: vk::VK_9, requires_shift: true });
        character_map.insert(ascii::RIGHT_PAREN, KeyInfo { code: vk::VK_0, requires_shift: true });

        character_map.insert(ascii::PLUS, KeyInfo { code: vk::VK_OEM_PLUS, requires_shift: true });
        character_map.insert(ascii::COLON, KeyInfo { code: vk::VK_OEM_1, requires_shift: true });
        character_map.insert(ascii::QUESTION_MARK, KeyInfo { code: vk::VK_OEM_2, requires_shift: true });
        character_map.insert(ascii::TILDE, KeyInfo { code: vk::VK_OEM_3, requires_shift: true });
        character_map.insert(ascii::LEFT_CURLY_BRACKET, KeyInfo { code: vk::VK_OEM_4, requires_shift: true });
        character_map.insert(ascii::VERTICAL_BAR, KeyInfo { code: vk::VK_OEM_5, requires_shift: true });
        character_map.insert(ascii::RIGHT_CURLY_BRACKET, KeyInfo { code: vk::VK_OEM_6, requires_shift: true });
        character_map.insert(ascii::DOUBLE_QUOTE, KeyInfo { code: vk::VK_OEM_7, requires_shift: true });
        character_map.insert(ascii::LESS_THAN, KeyInfo { code: vk::VK_OEM_COMMA, requires_shift: true });
        character_map.insert(ascii::GREATER_THAN, KeyInfo { code: vk::VK_OEM_PERIOD, requires_shift: true });

        AsciiToVkApi { character_map }
    }

    pub fn ascii_to_keystrokes(&self, keys: &str) -> Option<Vec::<u8>> {
        let characters = keys.as_bytes();
        let length = characters.len();

        // Allocate re-sizeable array to at-least our current character array length
        let mut keystrokes = Vec::<u8>::with_capacity(length);

        let mut char;
        let mut is_shifting = false;
        for i in 0..length {
            char = characters[i];

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
                keystrokes.push(char - ASCII_LOWERCASE_TO_UPPERCASE_OFFSET);
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

            if let Some(key_info) = self.character_map.get(&char) {
                if is_shifting && !key_info.requires_shift { // We no longer need to be shifting
                    is_shifting = false;
                    keystrokes.push(vk::VK_SHIFT);
                }
                else if !is_shifting && key_info.requires_shift { // We need to start shifting
                    is_shifting = true;
                    keystrokes.push(vk::VK_SHIFT);
                }
                keystrokes.push(key_info.code);
                continue
            }

            return None
        }

        // Add a trailing shift if needed
        // Occurs when last character as [a-z]
        if is_shifting { keystrokes.push(vk::VK_SHIFT); }

        Some(keystrokes)
    }
}



/* ### --- UNIT TEST --- ### */



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_failure_case() {
        let strokes = AsciiToVkApi::new().ascii_to_keystrokes("¾");

        assert_eq!(strokes.is_none(), true, "expect this character to not map to a backing key");
    }

    mod alphabetical {
        use super::*;


        #[test]
        fn test_lowercase_a() {
            let strokes = AsciiToVkApi::new().ascii_to_keystrokes("a").unwrap();

            assert_eq!(strokes.len(), 1, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_A);
        }

        #[test]
        fn test_lowercase_z() {
            let strokes = AsciiToVkApi::new().ascii_to_keystrokes("z").unwrap();

            assert_eq!(strokes.len(), 1, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_Z);
        }

        #[test]
        fn test_uppercase_a() {
            let strokes = AsciiToVkApi::new().ascii_to_keystrokes("A").unwrap();

            assert_eq!(strokes.len(), 3, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_SHIFT);
            assert_eq!(strokes[1], vk::VK_A);
            assert_eq!(strokes[2], vk::VK_SHIFT);
        }

        #[test]
        fn test_uppercase_z() {
            let strokes = AsciiToVkApi::new().ascii_to_keystrokes("Z").unwrap();

            assert_eq!(strokes.len(), 3, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_SHIFT);
            assert_eq!(strokes[1], vk::VK_Z);
            assert_eq!(strokes[2], vk::VK_SHIFT);
        }

        #[test]
        fn test_multiple_lowercases() {
            let strokes = AsciiToVkApi::new().ascii_to_keystrokes("abc").unwrap();

            assert_eq!(strokes.len(), 3, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_A);
            assert_eq!(strokes[1], vk::VK_B);
            assert_eq!(strokes[2], vk::VK_C);
        }

        #[test]
        fn test_multiple_uppercases() {
            let strokes = AsciiToVkApi::new().ascii_to_keystrokes("ABC").unwrap();

            assert_eq!(strokes.len(), 5, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_SHIFT);
            assert_eq!(strokes[1], vk::VK_A);
            assert_eq!(strokes[2], vk::VK_B);
            assert_eq!(strokes[3], vk::VK_C);
            assert_eq!(strokes[4], vk::VK_SHIFT);
        }

        #[test]
        fn test_multiple_mixed_cases_v1() {
            let strokes = AsciiToVkApi::new().ascii_to_keystrokes("aBc").unwrap();

            assert_eq!(strokes.len(), 5, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_A);
            assert_eq!(strokes[1], vk::VK_SHIFT);
            assert_eq!(strokes[2], vk::VK_B);
            assert_eq!(strokes[3], vk::VK_SHIFT);
            assert_eq!(strokes[4], vk::VK_C);
        }

        #[test]
        fn test_multiple_mixed_cases_v2() {
            let strokes = AsciiToVkApi::new().ascii_to_keystrokes("abcABCabc").unwrap();

            assert_eq!(strokes.len(), 11, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_A);
            assert_eq!(strokes[1], vk::VK_B);
            assert_eq!(strokes[2], vk::VK_C);

            assert_eq!(strokes[3], vk::VK_SHIFT);
            assert_eq!(strokes[4], vk::VK_A);
            assert_eq!(strokes[5], vk::VK_B);
            assert_eq!(strokes[6], vk::VK_C);
            assert_eq!(strokes[7], vk::VK_SHIFT);

            assert_eq!(strokes[8], vk::VK_A);
            assert_eq!(strokes[9], vk::VK_B);
            assert_eq!(strokes[10], vk::VK_C);
        }

        #[test]
        fn test_multiple_mixed_cases_v3() {
            let strokes = AsciiToVkApi::new().ascii_to_keystrokes("ZbcAzCaZc").unwrap();

            assert_eq!(strokes.len(), 17, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_SHIFT);
            assert_eq!(strokes[1], vk::VK_Z);
            assert_eq!(strokes[2], vk::VK_SHIFT);

            assert_eq!(strokes[3], vk::VK_B);
            assert_eq!(strokes[4], vk::VK_C);

            assert_eq!(strokes[5], vk::VK_SHIFT);
            assert_eq!(strokes[6], vk::VK_A);
            assert_eq!(strokes[7], vk::VK_SHIFT);

            assert_eq!(strokes[8], vk::VK_Z);

            assert_eq!(strokes[9], vk::VK_SHIFT);
            assert_eq!(strokes[10], vk::VK_C);
            assert_eq!(strokes[11], vk::VK_SHIFT);

            assert_eq!(strokes[12], vk::VK_A);

            assert_eq!(strokes[13], vk::VK_SHIFT);
            assert_eq!(strokes[14], vk::VK_Z);
            assert_eq!(strokes[15], vk::VK_SHIFT);

            assert_eq!(strokes[16], vk::VK_C);
        }
    }

    mod vk_0_through_vk_9 {
        use super::*;

        #[test]
        fn test_number_one() {
            let strokes = AsciiToVkApi::new().ascii_to_keystrokes("1").unwrap();

            assert_eq!(strokes.len(), 1, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_1);
        }

        #[test]
        fn test_exclamation_mark() {
            let strokes = AsciiToVkApi::new().ascii_to_keystrokes("!").unwrap();

            assert_eq!(strokes.len(), 3, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_SHIFT);
            assert_eq!(strokes[1], vk::VK_1);
            assert_eq!(strokes[2], vk::VK_SHIFT);
        }

        #[test]
        fn test_number_two() {
            let strokes = AsciiToVkApi::new().ascii_to_keystrokes("2").unwrap();

            assert_eq!(strokes.len(), 1, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_2);
        }

        #[test]
        fn test_at_sign() {
            let strokes = AsciiToVkApi::new().ascii_to_keystrokes("@").unwrap();

            assert_eq!(strokes.len(), 3, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_SHIFT);
            assert_eq!(strokes[1], vk::VK_2);
            assert_eq!(strokes[2], vk::VK_SHIFT);
        }

        #[test]
        fn test_number_three() {
            let strokes = AsciiToVkApi::new().ascii_to_keystrokes("3").unwrap();

            assert_eq!(strokes.len(), 1, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_3);
        }

        #[test]
        fn test_hash() {
            let strokes = AsciiToVkApi::new().ascii_to_keystrokes("#").unwrap();

            assert_eq!(strokes.len(), 3, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_SHIFT);
            assert_eq!(strokes[1], vk::VK_3);
            assert_eq!(strokes[2], vk::VK_SHIFT);
        }

        #[test]
        fn test_number_four() {
            let strokes = AsciiToVkApi::new().ascii_to_keystrokes("4").unwrap();

            assert_eq!(strokes.len(), 1, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_4);
        }

        #[test]
        fn test_dollar_sign() {
            let strokes = AsciiToVkApi::new().ascii_to_keystrokes("$").unwrap();

            assert_eq!(strokes.len(), 3, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_SHIFT);
            assert_eq!(strokes[1], vk::VK_4);
            assert_eq!(strokes[2], vk::VK_SHIFT);
        }

        #[test]
        fn test_number_five() {
            let strokes = AsciiToVkApi::new().ascii_to_keystrokes("5").unwrap();

            assert_eq!(strokes.len(), 1, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_5);
        }

        #[test]
        fn test_percent() {
            let strokes = AsciiToVkApi::new().ascii_to_keystrokes("%").unwrap();

            assert_eq!(strokes.len(), 3, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_SHIFT);
            assert_eq!(strokes[1], vk::VK_5);
            assert_eq!(strokes[2], vk::VK_SHIFT);
        }

        #[test]
        fn test_number_six() {
            let strokes = AsciiToVkApi::new().ascii_to_keystrokes("6").unwrap();

            assert_eq!(strokes.len(), 1, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_6);
        }

        #[test]
        fn test_caret() {
            let strokes = AsciiToVkApi::new().ascii_to_keystrokes("^").unwrap();

            assert_eq!(strokes.len(), 3, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_SHIFT);
            assert_eq!(strokes[1], vk::VK_6);
            assert_eq!(strokes[2], vk::VK_SHIFT);
        }

        #[test]
        fn test_seven() {
            let strokes = AsciiToVkApi::new().ascii_to_keystrokes("7").unwrap();

            assert_eq!(strokes.len(), 1, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_7);
        }

        #[test]
        fn test_ampersand() {
            let strokes = AsciiToVkApi::new().ascii_to_keystrokes("&").unwrap();

            assert_eq!(strokes.len(), 3, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_SHIFT);
            assert_eq!(strokes[1], vk::VK_7);
            assert_eq!(strokes[2], vk::VK_SHIFT);
        }

        #[test]
        fn test_number_eight() {
            let strokes = AsciiToVkApi::new().ascii_to_keystrokes("8").unwrap();

            assert_eq!(strokes.len(), 1, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_8);
        }

        #[test]
        fn test_number_9() {
            let strokes = AsciiToVkApi::new().ascii_to_keystrokes("9").unwrap();

            assert_eq!(strokes.len(), 1, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_9);
        }

        #[test]
        fn test_left_paren() {
            let strokes = AsciiToVkApi::new().ascii_to_keystrokes("(").unwrap();

            assert_eq!(strokes.len(), 3, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_SHIFT);
            assert_eq!(strokes[1], vk::VK_9);
            assert_eq!(strokes[2], vk::VK_SHIFT);
        }

        #[test]
        fn test_number_zero() {
            let strokes = AsciiToVkApi::new().ascii_to_keystrokes("0").unwrap();

            assert_eq!(strokes.len(), 1, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_0);
        }

        #[test]
        fn test_right_paren() {
            let strokes = AsciiToVkApi::new().ascii_to_keystrokes(")").unwrap();

            assert_eq!(strokes.len(), 3, "incorrect key vector length");
            assert_eq!(strokes[0], vk::VK_SHIFT);
            assert_eq!(strokes[1], vk::VK_0);
            assert_eq!(strokes[2], vk::VK_SHIFT);
        }
    }

    #[test]
    fn test_multiply() {
        let strokes = AsciiToVkApi::new().ascii_to_keystrokes("*").unwrap();

        assert_eq!(strokes.len(), 1, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_MULTIPLY);
    }

    #[test]
    fn test_plus() {
        let strokes = AsciiToVkApi::new().ascii_to_keystrokes("+").unwrap();

        assert_eq!(strokes.len(), 3, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_SHIFT);
        assert_eq!(strokes[1], vk::VK_OEM_PLUS);
        assert_eq!(strokes[2], vk::VK_SHIFT);
    }

    #[test]
    fn test_minus() {
        let strokes = AsciiToVkApi::new().ascii_to_keystrokes("-").unwrap();

        assert_eq!(strokes.len(), 1, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_MINUS);
    }

    #[test]
    fn test_comma() {
        let strokes = AsciiToVkApi::new().ascii_to_keystrokes(",").unwrap();

        assert_eq!(strokes.len(), 1, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_OEM_COMMA);
    }

    #[test]
    fn test_divide() {
        let strokes = AsciiToVkApi::new().ascii_to_keystrokes("/").unwrap();

        assert_eq!(strokes.len(), 1, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_DIVIDE);
    }

    #[test]
    fn test_period() {
        let strokes = AsciiToVkApi::new().ascii_to_keystrokes(".").unwrap();

        assert_eq!(strokes.len(), 1, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_OEM_PERIOD);
    }

    #[test]
    fn test_white_space() {
        let strokes = AsciiToVkApi::new().ascii_to_keystrokes(" ").unwrap();

        assert_eq!(strokes.len(), 1, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_SPACE);
    }

    #[test]
    fn test_semi_colon() {
        let strokes = AsciiToVkApi::new().ascii_to_keystrokes(";").unwrap();

        assert_eq!(strokes.len(), 1, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_OEM_1);
    }

    #[test]
    fn test_colon() {
        let strokes = AsciiToVkApi::new().ascii_to_keystrokes(":").unwrap();

        assert_eq!(strokes.len(), 3, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_SHIFT);
        assert_eq!(strokes[1], vk::VK_OEM_1);
        assert_eq!(strokes[2], vk::VK_SHIFT);
    }

    #[test]
    fn test_question_mark() {
        let strokes = AsciiToVkApi::new().ascii_to_keystrokes("?").unwrap();

        assert_eq!(strokes.len(), 3, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_SHIFT);
        assert_eq!(strokes[1], vk::VK_OEM_2);
        assert_eq!(strokes[2], vk::VK_SHIFT);
    }

    #[test]
    fn test_backtick() {
        let strokes = AsciiToVkApi::new().ascii_to_keystrokes("`").unwrap();

        assert_eq!(strokes.len(), 1, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_OEM_3);
    }

    #[test]
    fn test_tilde() {
        let strokes = AsciiToVkApi::new().ascii_to_keystrokes("~").unwrap();

        assert_eq!(strokes.len(), 3, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_SHIFT);
        assert_eq!(strokes[1], vk::VK_OEM_3);
        assert_eq!(strokes[2], vk::VK_SHIFT);
    }

    #[test]
    fn test_left_bracket() {
        let strokes = AsciiToVkApi::new().ascii_to_keystrokes("[").unwrap();

        assert_eq!(strokes.len(), 1, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_OEM_4);
    }

    #[test]
    fn test_left_curly_bracket() {
        let strokes = AsciiToVkApi::new().ascii_to_keystrokes("{").unwrap();

        assert_eq!(strokes.len(), 3, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_SHIFT);
        assert_eq!(strokes[1], vk::VK_OEM_4);
        assert_eq!(strokes[2], vk::VK_SHIFT);
    }

    #[test]
    fn test_backslash() {
        let strokes = AsciiToVkApi::new().ascii_to_keystrokes("\\").unwrap();

        assert_eq!(strokes.len(), 1, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_OEM_5);
    }

    #[test]
    fn test_vertical_bar() {
        let strokes = AsciiToVkApi::new().ascii_to_keystrokes("|").unwrap();

        assert_eq!(strokes.len(), 3, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_SHIFT);
        assert_eq!(strokes[1], vk::VK_OEM_5);
        assert_eq!(strokes[2], vk::VK_SHIFT);
    }

    #[test]
    fn test_right_bracket() {
        let strokes = AsciiToVkApi::new().ascii_to_keystrokes("]").unwrap();

        assert_eq!(strokes.len(), 1, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_OEM_6);
    }

    #[test]
    fn test_right_curly_bracket() {
        let strokes = AsciiToVkApi::new().ascii_to_keystrokes("}").unwrap();

        assert_eq!(strokes.len(), 3, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_SHIFT);
        assert_eq!(strokes[1], vk::VK_OEM_6);
        assert_eq!(strokes[2], vk::VK_SHIFT);
    }

    #[test]
    fn test_single_quote() {
        let strokes = AsciiToVkApi::new().ascii_to_keystrokes("'").unwrap();

        assert_eq!(strokes.len(), 1, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_OEM_7);
    }

    #[test]
    fn test_double_quote() {
        let strokes = AsciiToVkApi::new().ascii_to_keystrokes("\"").unwrap();

        assert_eq!(strokes.len(), 3, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_SHIFT);
        assert_eq!(strokes[1], vk::VK_OEM_7);
        assert_eq!(strokes[2], vk::VK_SHIFT);
    }

    #[test]
    fn test_less_than() {
        let strokes = AsciiToVkApi::new().ascii_to_keystrokes("<").unwrap();

        assert_eq!(strokes.len(), 3, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_SHIFT);
        assert_eq!(strokes[1], vk::VK_OEM_COMMA);
        assert_eq!(strokes[2], vk::VK_SHIFT);
    }

    #[test]
    fn test_greater_than() {
        let strokes = AsciiToVkApi::new().ascii_to_keystrokes(">").unwrap();

        assert_eq!(strokes.len(), 3, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_SHIFT);
        assert_eq!(strokes[1], vk::VK_OEM_PERIOD);
        assert_eq!(strokes[2], vk::VK_SHIFT);
    }

    #[test]
    fn test_equal() {
        let strokes = AsciiToVkApi::new().ascii_to_keystrokes("=").unwrap();

        assert_eq!(strokes.len(), 1, "incorrect key vector length");
        assert_eq!(strokes[0], vk::VK_OEM_PLUS);
    }
}