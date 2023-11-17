# ControlMap (De)Serializer Library

The controlmap.txt parser for Skyrim.

## Table of Contents

- [ControlMap (De)Serializer Library](#controlmap-deserializer-library)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
- [Examples](#examples)
  - [License](#license)

## Features

- [x] `controlmap.txt` => `json`
- [x] `json` => `controlmap.txt` (formatted with preserved comments)
- [x] Analysis using enum scanCodes.

# Examples

- PreRequirements

```toml
[dependencies]
controlmap_parser = "*"
serde_json = "1.0" # Json converter
```

```rust
use controlmap_parser::ControlMap;

type Result<T, E = Box<dyn std::error::Error + 'static>> = core::result::Result<T, E>;
fn main() -> Result<()> {
    let input = r#"
// Lockpicking
RotatePick	0xff		0xa		0x000b	0	0	0	0x8
RotateLock	0x1e,0xff	0xff	0x000c	0	0	0	0x8
DebugMode	0x35		0xff	0x4000	0	0	0	0x8
Cancel		0x0f		0xff	0x1000	0	0	0	0x8

// Favor
Cancel	0x0f	0xff	0x1000	0	0	0	0x108"#;

    let control_map = ControlMap::from_txt(input)?;
    println!("txt => Struct:\n{:?}", &control_map);


    // txt => json
    let expected_json = r#"{
  "lines": [
    "BlankLine",
    {
      "Comment": " Lockpicking"
    },
    {
      "EventLine": {
        "event_name": "RotatePick",
        "keyboard_id": {
          "One": "0xff"
        },
        "mouse_id": {
          "One": "0xa"
        },
        "gamepad_id": {
          "One": "0x000b"
        },
        "remap_key": false,
        "remap_mouse": false,
        "remap_gamepad": false,
        "event_binary_flag": "0x8"
      }
    },
    {
      "EventLine": {
        "event_name": "RotateLock",
        "keyboard_id": {
          "Or": [
            {
              "One": "0x1e"
            },
            {
              "One": "0xff"
            }
          ]
        },
        "mouse_id": {
          "One": "0xff"
        },
        "gamepad_id": {
          "One": "0x000c"
        },
        "remap_key": false,
        "remap_mouse": false,
        "remap_gamepad": false,
        "event_binary_flag": "0x8"
      }
    },
    {
      "EventLine": {
        "event_name": "DebugMode",
        "keyboard_id": {
          "One": "0x35"
        },
        "mouse_id": {
          "One": "0xff"
        },
        "gamepad_id": {
          "One": "0x4000"
        },
        "remap_key": false,
        "remap_mouse": false,
        "remap_gamepad": false,
        "event_binary_flag": "0x8"
      }
    },
    {
      "EventLine": {
        "event_name": "Cancel",
        "keyboard_id": {
          "One": "0x0f"
        },
        "mouse_id": {
          "One": "0xff"
        },
        "gamepad_id": {
          "One": "0x1000"
        },
        "remap_key": false,
        "remap_mouse": false,
        "remap_gamepad": false,
        "event_binary_flag": "0x8"
      }
    },
    "BlankLine",
    {
      "Comment": " Favor"
    },
    {
      "EventLine": {
        "event_name": "Cancel",
        "keyboard_id": {
          "One": "0x0f"
        },
        "mouse_id": {
          "One": "0xff"
        },
        "gamepad_id": {
          "One": "0x1000"
        },
        "remap_key": false,
        "remap_mouse": false,
        "remap_gamepad": false,
        "event_binary_flag": "0x108"
      }
    }
  ]
}"#;
    let json = serde_json::to_string_pretty(&control_map)?;
    println!("struct => json:\n{:?}", &control_map);
    assert_eq!(&json, expected_json);

    let re_control_map: ControlMap = serde_json::from_str(&json)?;
    println!("json => txt:\n{}", &re_control_map);
    // Yes, blank lines are not removed, they are applied as is.
    // There are places where it would be an error to do otherwise.
    let formatted_control_map = r#"
// Lockpicking
RotatePick	0xff	0xa	0x000b	0	0	0	0x8
RotateLock	0x1e+0xff	0xff	0x000c	0	0	0	0x8
DebugMode	0x35	0xff	0x4000	0	0	0	0x8
Cancel	0x0f	0xff	0x1000	0	0	0	0x8

// Favor
Cancel	0x0f	0xff	0x1000	0	0	0	0x108
"#;
    assert_eq!(re_control_map.to_string(), formatted_control_map);

    Ok(())
}
```

```shell
cargo run --example cast_and_fmt --features serde
cargo run --example parse_and_print --features serde
cargo run --example scan_code
```

## License

[MIT](https://opensource.org/licenses/MIT) or
[Apache 2.0](https://opensource.org/license/apache-2-0/)
