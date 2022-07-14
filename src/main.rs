mod core;
mod utils;

use crate::core::{BirdCli, BirdConfig};
use crate::utils::errors::BirdError;
use clap::Parser;

fn main() {
   match run_main() {
      Ok(_) => (),
      Err(err) => eprintln!("{}", err),
   }
}

fn run_main() -> Result<(), BirdError> {
   let config = BirdConfig::new()?;
   let values = BirdCli::parse();

   values.subcmd.call(config)?;
   Ok(())
}
