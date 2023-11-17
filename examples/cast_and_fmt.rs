use anyhow::Result;
use controlmap_parser::ControlMap;

fn main() -> Result<()> {
    let _guard = init_tracing()?;
    cast_and_fmt().map_err(|err| {
        tracing::error!("Error: {:?}", err);
        err
    })?;
    Ok(())
}

/// color std::io::out & non color file out init logger
fn init_tracing() -> Result<tracing_appender::non_blocking::WorkerGuard> {
    use tracing_subscriber::{fmt, layer::SubscriberExt};
    std::fs::create_dir_all("./log")?;
    let (file_writer, guard) =
        tracing_appender::non_blocking(std::fs::File::create("./log/cast_and_fmt.log")?);
    tracing::subscriber::set_global_default(
        fmt::Subscriber::builder()
            .with_max_level(tracing::Level::TRACE)
            .finish()
            .with(
                fmt::Layer::default()
                    .with_writer(file_writer)
                    .with_ansi(false),
            ),
    )
    .expect("Unable to set global tracing subscriber");
    tracing::debug!("Tracing initialized.");
    Ok(guard)
}

fn cast_and_fmt() -> Result<()> {
    let input = r#"
// Lockpicking
RotatePick	0xff		0xa		0x000b	0	0	0	0x8
RotateLock	0x1e,0xff	0xff	0x000c	0	0	0	0x8
DebugMode	0x35		0xff	0x4000	0	0	0	0x8
Cancel		0x0f		0xff	0x1000	0	0	0	0x8

// Favor
Cancel	0x0f	0xff	0x1000	0	0	0	0x108"#;

    let control_map = ControlMap::from_txt(input)?;
    tracing::debug!("txt => Struct:\n{:?}", &control_map);

    let json = serde_json::to_string_pretty(&control_map)?;
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
    assert_eq!(&json, expected_json);

    let control_map: ControlMap = serde_json::from_str(&json)?;
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
    assert_eq!(control_map.to_string(), formatted_control_map);

    Ok(())
}
