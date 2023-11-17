use anyhow::Result;
use controlmap_parser::{
    parser::{EventLine, KeyID, Line},
    scan_code::{GamepadCode, KeyboardCode, MouseCode},
    ControlMap,
};
use core::str::FromStr;
use tracing::trace;

fn main() -> Result<()> {
    let _guard = init_tracing()?;
    scan_code().map_err(|err| {
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
        tracing_appender::non_blocking(std::fs::File::create("./log/scan_code.log")?);
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

fn scan_code() -> Result<()> {
    let input = r#"
// Lockpicking
RotatePick	0xff		0xa		0x000b	0	0	0	0x8
RotateLock	0x1e,0xff	0xff	0x000c	0	0	0	0x8
DebugMode	0x35		0xff	0x4000	0	0	0	0x8
Cancel		0x0f		0xff	0x1000	0	0	0	0x8

// Favor
Cancel	0x0f	0xff	0x1000	0	0	0	0x108"#;

    let control_map = ControlMap::from_txt(input)?;

    for line in control_map {
        match line {
            Line::Comment(comment) => trace!("{}", comment),
            Line::EventLine(event) => parse_event_line(event),
            Line::BlankLine => trace!("\n"),
        }
    }

    Ok(())
}

fn parse_event_line(event: EventLine) {
    trace!("Event Name: {}", event.event_name);
    parse_keyboard(event.keyboard_id);
    parse_game(event.gamepad_id);
    parse_mouse(event.mouse_id);
    trace!("\n");
}

fn parse_game(key_id: KeyID) {
    match key_id {
        KeyID::Or(vec) => {
            for key in vec {
                parse_game(key)
            }
        }
        KeyID::And(vec) => {
            for key in vec {
                parse_game(key)
            }
        }
        KeyID::One(key) => trace!("Game: {} => {:?}", &key, GamepadCode::from_str(&key)),
        KeyID::Alias(alias) => trace!("GameAlias: {}", alias),
    };
}

fn parse_mouse(key_id: KeyID) {
    match key_id {
        KeyID::Or(vec) => {
            for key in vec {
                parse_mouse(key)
            }
        }
        KeyID::And(vec) => {
            for key in vec {
                parse_mouse(key)
            }
        }
        KeyID::One(key) => trace!("Mouse: {} => {:?}", &key, MouseCode::from_str(&key)),
        KeyID::Alias(alias) => trace!("MouseAlias: {}", alias),
    };
}

fn parse_keyboard(key_id: KeyID) {
    match key_id {
        KeyID::Or(vec) => {
            for key in vec {
                parse_keyboard(key)
            }
        }
        KeyID::And(vec) => {
            for key in vec {
                parse_keyboard(key)
            }
        }
        KeyID::One(key) => trace!("Keyboard: {} => {:?}", &key, KeyboardCode::from_str(&key)),
        KeyID::Alias(alias) => trace!("KeyAlias: {}", alias),
    };
}
