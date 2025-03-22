<p align="center">
    <img src="https://github.com/user-attachments/assets/c281a87f-24f4-44ad-8922-f263bbe8c642" alt="text being typed"/>
</p>

<br/>

![Passing Status](https://github.com/Chase-William/utf8-to-windows-vkc/actions/workflows/build.yml/badge.svg)
![example workflow](https://github.com/Chase-William/utf8-to-windows-vkc/actions/workflows/test.yml/badge.svg)
[![License](https://img.shields.io/github/license/Chase-William/utf8-to-windows-vkc?color=594ae2&logo=github&style=flat-square)](https://github.com/Chase-William/utf8-to-windows-vkc/blob/main/LICENSE)
## Utf-8 `&str` to Windows Virtual Key Codes

A simple library that translates the following utf-8/ascii characters <code>[a-zA-Z0-9&#96;~!@#$%^&*()-_=+[{]}\|;:'",<.>?]</code> into keystrokes for Windows computers using US standard keyboards.

- ✅ Supports all ascii characters `[32, 127)` *(interval notation)* on US standard keyboards
- ✅ Provides both short-hand and more performant variants of `to_keystrokes`
- ✅ Uses a compile time map for optimal performance
- ✅ Has range checks for incoming character values using `Result<T, E>`
- ✅ Uses unit testing to ensure mapping validity

Use this project for mapping keys in small projects with simple uses cases, this is not mean't for large professional and/or multi-languaged projects.

## Example Usage

```rs
fn main() {
    // Receive a new collection
    match utf8_to_windows_vkc::to_keystrokes_new("Hello, World!") {
        // send keystrokes on successful map
        Ok(keystrokes) => send_keystrokes(&keystrokes),
        // print error on failure
        Err(err ) => println!("Byte: {}, Error-Code: {:?}", err.byte, err.error_code)
    }

    // -- OR --

    // Re-use an existing collection (it is your responsibility to clear when appropriate)
    let mut keystrokes: Vec<u8> = Vec::new();
    match utf8_to_windows_vkc::to_keystrokes_mut("Hello, World!", &mut keystrokes) {
        // send keystrokes on successful map
        Ok(()) => send_keystrokes(&keystrokes),
        // print error on failure
        Err(err ) => println!("Byte: {}, Error-Code: {:?}", err.byte, err.error_code)
    }
}
```

Then define a function using your preferred win32 rust library/approach to send keystrokes like below:

```rs
use std::thread;
use std::time::Duration;
use winapi::ctypes::c_int;
use winapi::um::winuser::{INPUT_u, SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, VK_SHIFT};
static INPUT_SIZE: c_int = size_of::<INPUT>() as c_int;

/// example impl using win32's SendInput function
fn send_keystrokes(keys: &Vec<u8>) {
    let len = keys.len();
    let mut is_shifting= false;

    for i in 0..len { // iter over keys
        // create windows input union structure
        let mut input_u: INPUT_u = unsafe { std::mem::zeroed() };
        let mut dw_flags = 0;

        // check if current key is the shifting key, then toggle
        if keys[i] as c_int == VK_SHIFT {
            if is_shifting { // currently shifting, so release
                is_shifting = false;
                dw_flags = KEYEVENTF_KEYUP
            } else { // start shifting
                is_shifting = true; // update state that we are now shifting
            }
        }

        unsafe {
            // attach our keystroke information
            *input_u.ki_mut() = KEYBDINPUT {
                wVk: keys[i] as u16,
                wScan: 0,
                dwFlags: dw_flags,
                time: 0,
                dwExtraInfo: 0,
            };
        }
        // create main input structure
        let mut input = INPUT {
            type_: INPUT_KEYBOARD,
            u: input_u
        };

        unsafe {
            // Use win32's SendInput to simulate keystroke
            if SendInput(1, &mut input, INPUT_SIZE) == 0 {
                print!("At least one keystroke failed to be enqueued.");
            }
        }

        // IMPORTANT: Example delay allowing receiving process to handle keystrokes before receiving more
        thread::sleep(Duration::from_millis(50));
    }
}
```

## About This Project

This library's goal is to provide common translations while remaining independent of both the [windows](https://crates.io/crates/windows) and [winapi](https://crates.io/crates/winapi) crates. I understand there are tools available in both of these crates that could reduce much of this library's limited logic, however, that would undermine the objective of this project. Moreover, I do not want to create my own bindings to win32 using a _c_ foreign function interface.

This library uses a compile-time map from the [phf](https://docs.rs/phf/latest/phf/).

## Translation Notes

Translating an [ascii](https://www.ascii-code.com/) character to a Window's [virtual-key-code](https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes) requires additional logic because of Window's use of the `shift` key.

Not all ascii characters have a key representation, meaning not everything can be translated. If translation fails, `Result<T, E>` is returned.

> For example, uppercase characters `[A-Z]` do not have a unique virtual key code, instead they depend on the usage of their lowercase variant in conjunction with the shift key. Therefore, to type 'A' you must simulate the shift key down, press the windows virtual key code for 'a' and then release the shift key.

> Note: There will always be an even number of `shift` keys present in every vector as every `shift` press _(down)_ will eventually have a corresponding `shift` release _(up)_. Think of it as curly brackets, opening always has a closing.
