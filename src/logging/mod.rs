use log::LevelFilter;
use simplelog::*;

pub fn init_logger() {
    let config = ConfigBuilder::new()
        .set_level_color(Level::Error, Some(Color::Rgb(191, 0, 0)))
        .set_level_color(Level::Warn, Some(Color::Rgb(255, 127, 0)))
        .set_level_color(Level::Info, Some(Color::Rgb(192, 192, 0)))
        .set_level_color(Level::Debug, Some(Color::Rgb(63, 127, 0)))
        .set_level_color(Level::Trace, Some(Color::Rgb(127, 127, 255)))
        .build();

    TermLogger::init(
        LevelFilter::Trace,
        config,
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )
    .unwrap();
}
