# ControlMap Parser Library

The controlmap.txt parser for Skyrim.

## Table of Contents

- [ControlMap Parser Library](#controlmap-parser-library)
  - [Table of Contents](#table-of-contents)
  - [features](#features)
- [Examples](#examples)
  - [License](#license)

## features

- [x] controlmap.txt => json
- [ ] json => controlmap.txt

# Examples

```rust
use pretty_assertions::assert_eq;
use controlmap_parser::{control_map_parser, EventLine, Line, KeyID};

let input = r#"
// Main Gameplay
Forward				0x11		0xff	0xff			1	1	0	0x801
Back				0x1f		0xff	0xff			1	1	0	0x801
// Menu Mode
Accept		!0,Activate	0xff	0x2000	0	0	0	0x8
"#;
let actual = control_map_parser(input);

let expected = Ok((
    "",
    vec![
        Line::BlankLine,
        Line::Comment(" Main Gameplay"),
        Line::EventLine(EventLine {
            event_name: "Forward",
            keyboard_id: KeyID::One("0x11"),
            mouse_id: KeyID::One("0xff"),
            gamepad_id: KeyID::One("0xff"),
            remap_key: true,
            remap_mouse: true,
            remap_gamepad: false,
            event_binary_flag: Some("0x801"),
        }),
        Line::EventLine(EventLine {
            event_name: "Back",
            keyboard_id: KeyID::One("0x1f"),
            mouse_id: KeyID::One("0xff"),
            gamepad_id: KeyID::One("0xff"),
            remap_key: true,
            remap_mouse: true,
            remap_gamepad: false,
            event_binary_flag: Some("0x801"),
        }),
        Line::Comment(" Menu Mode"),
        Line::EventLine(EventLine {
            event_name: "Accept",
            keyboard_id: KeyID::Alias("Activate"),
            mouse_id: KeyID::One("0xff"),
            gamepad_id: KeyID::One("0x2000"),
            remap_key: false,
            remap_mouse: false,
            remap_gamepad: false,
            event_binary_flag: Some("0x8"),
        }),
    ],
));
assert_eq!(actual, expected);
```

## License

[MIT](https://opensource.org/licenses/MIT)
