use aoclib::{config::Config, website::get_input};
use day18::{part1, part2};

use color_eyre::eyre::Result;
use std::path::PathBuf;
use structopt::StructOpt;

const DAY: u8 = 18;

#[derive(StructOpt, Debug)]
struct RunArgs {
    /// input file
    #[structopt(long, parse(from_os_str))]
    input: Option<PathBuf>,

    /// skip part 1
    #[structopt(long = "no-part1")]
    no_part1: bool,

    /// run part 2
    #[structopt(long)]
    part2: bool,

    /// create an animation of the lights at the specified path
    #[structopt(long, parse(from_os_str))]
    #[cfg(feature = "animate")]
    create_animation: Option<PathBuf>,
}

impl RunArgs {
    fn input(&self) -> Result<PathBuf> {
        match self.input {
            None => {
                let config = Config::load()?;
                // this does nothing if the input file already exists, but
                // simplifies the workflow after cloning the repo on a new computer
                get_input(&config, 2015, DAY)?;
                Ok(config.input_for(2015, DAY))
            }
            Some(ref path) => Ok(path.clone()),
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = RunArgs::from_args();
    let input_path = args.input()?;

    if !args.no_part1 {
        part1(&input_path)?;
    }
    if args.part2 {
        part2(&input_path)?;
    }
    #[cfg(feature = "animate")]
    if let Some(output_path) = args.create_animation {
        day18::animate::animate(&input_path, &output_path)?;
    }
    Ok(())
}
