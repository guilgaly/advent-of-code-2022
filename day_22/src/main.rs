extern crate core;

use crate::instructions::parse_instructions;
use crate::part_1::part_1;
use crate::part_2::part_2;
use crate::tiles::parse_tiles;
use common::time_execution;
use std::error::Error;

mod instructions;
mod part_1;
mod part_2;
mod tiles;

static TILES_INPUT: &str = include_str!("tiles");
static INSTRUCTIONS_INPUT: &str = include_str!("instructions");

fn main() -> Result<(), Box<dyn Error>> {
    let tiles = parse_tiles(TILES_INPUT)?;
    let instructions = parse_instructions(INSTRUCTIONS_INPUT)?;
    println!(
        "Part 1 result: {}",
        time_execution("Part 1", || part_1(&tiles, &instructions))
    );

    println!(
        "Part 2 result: {}",
        time_execution("Part 2", || part_2(&tiles, &instructions))
    );

    Ok(())
}
