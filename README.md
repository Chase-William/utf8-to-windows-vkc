<p align="center">
    <img src="https://github.com/user-attachments/assets/c281a87f-24f4-44ad-8922-f263bbe8c642" alt="text being typed"/>
</p>

<br/>
<br/>

![Passing Status](https://github.com/Chase-William/utf8-to-windows-vkc/actions/workflows/build.yml/badge.svg)
![example workflow](https://github.com/Chase-William/utf8-to-windows-vkc/actions/workflows/test.yml/badge.svg)
[![License](https://img.shields.io/github/license/Chase-William/utf8-to-windows-vkc?color=594ae2&logo=github&style=flat-square)](https://github.com/Chase-William/utf8-to-windows-vkc/blob/main/LICENSE)
## Utf-8 String to Windows Virtual Key Codes

A simple library that translates the following utf-8/ascii characters <code>[a-zA-Z0-0&#96;~!@#$%^&*()-_=+[{]}\|;:'",<.>?]</code> into keystrokes for Windows computers with no dependencies.

## Example Usage

```rs
use utf8_to_windows_vkc::Utf8KeyMapper;

fn main() {
    let key_mapper = Utf8KeyMapper::new(); // create api structure
    // unwrap keystrokes, returns error if map does not exists or invalid character
    let r = key_mapper.to_keystrokes("Hello, World!").unwrap();
    // Call your win32 implementation
    send_keystrokes(&r);
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


## Translation Notes

Translating an [ascii](https://www.ascii-code.com/) character to a Window's [virtual-key-code](https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes) requires additional logic because of Window's use of the `shift` key.

Not all ascii characters have a key representation, meaning not everything can be translated. If translation fails, `Result<T, E>` is returned.

> For example, uppercase characters `[A-Z]` do not have a unique virtual key code, instead they depend on the usage of their lowercase variant in conjunction with the shift key. Therefore, to type 'A' you must simulate the shift key down, press the windows virtual key code for 'a' and then release the shift key.

> Note: There will always be an even number of `shift` keys present in every vector as every `shift` press _(down)_ will eventually have a corresponding `shift` release _(up)_. Think of it as curly brackets, opening always has a closing.
