use bevy::prelude::*;
use clap::Parser;

#[derive(Parser, Debug, Resource)]
#[command(author, version, about, long_about = None)]
pub struct EmulatorArgs {
    /// Path to the ROM
    #[arg(short, long, required = true)]
    pub rom: String,

    /// Path to a custom font ROM
    #[arg(short, long)]
    pub font: Option<String>,
}