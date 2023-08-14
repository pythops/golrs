use clap::{arg, Command};

pub fn cli() -> Command {
    Command::new("golrs")
        .about("Game of life using webgpu")
        .arg(
            arg!(--size <size>)
                .help("The grid size.")
                .default_value("128")
                .required(false)
                .value_parser(clap::value_parser!(u16).range(1..)),
        )
}
