## ASCII to Windows Virtual-Key-Code

Converting an [ASCII](https://www.ascii-code.com/) character to a Window's [virtual-key-code](https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes) requires additional logic because of window's use of the `Shift` modifier to display some characters.

Uppercase character `[A-Z]` do not have a unique windows virtual key code, instead they depend on the usage of their lowercase variant in conjunction with the shift key.

> For example, to type 'A' using [SendInput](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendinput) you must press and hold shift, press the windows virtual key code for 'a' and then release shift.

There will always be an even number of `Shift` keys present as every `Shift` press _(down)_ will eventually have a corresponding `Shift` release _(up)_.