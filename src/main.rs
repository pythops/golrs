use golrs::cli;
use golrs::ui;

use clap::crate_version;

fn main() {
    let matches = cli::cli().version(crate_version!()).get_matches();

    let grid_size = matches.get_one::<u16>("size").unwrap();

    pollster::block_on(ui::render(*grid_size));
}
