//! Parse the actual controlmap.txt file.
use anyhow::Result;
use controlmap_parser::ControlMap;
use tracing::trace;

fn main() -> Result<()> {
    let _guard = init_tracing()?;
    parse_and_print().map_err(|err| {
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
        tracing_appender::non_blocking(std::fs::File::create("./log/parse_and_print.log")?);
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

fn parse_and_print() -> Result<()> {
    let input = include_str!("./controlmap.txt");

    match ControlMap::from_txt(input) {
        Ok(control_map) => {
            trace!("txt => Struct:\n{:?}", &control_map);

            let json = serde_json::to_string_pretty(&control_map)?;
            trace!("Struct => Json:\n{}", &json);

            let re_control_map: ControlMap = serde_json::from_str(&json)?;
            trace!("Json => txt:\n{}", re_control_map);
        }
        Err(err) => eprintln!("Parsing error: {:?}", err),
    }

    Ok(())
}
