extern crate clap;
extern crate ears;
#[macro_use]
extern crate glium;
#[macro_use]
extern crate nom;
extern crate time;

mod cpm;
mod cpu;
mod debug;
mod disassemble;
mod machine;
mod memory;
mod spaceinvaders;

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::u8;

use clap::{Arg, App, SubCommand};
use ears::{Sound, AudioController};

use cpu::Cpu;
use disassemble::disassemble;
use spaceinvaders::SpaceInvadersMachine;

enum MachineType {
    SpaceInvaders,
    Cpm,
}

enum Options {
    SpaceInvaders {
        filename: String,
    },
    Cpm {
        filename: String,
    },
    Disassemble {
        filename: String,
        offset: usize,
    },
    Debug {
        filename: String,
    },
}

fn get_opts() -> Options {
    let matches = App::new("emu8080")
        .version("0.1")
        .subcommand(SubCommand::with_name("cpm")
            .arg(Arg::with_name("FILENAME")
                .required(true)))
        .subcommand(SubCommand::with_name("dis")
            .arg(Arg::with_name("FILENAME")
                .required(true))
            .arg(Arg::with_name("OFFSET")
                .short("o")
                .long("offset")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("spaceinvaders")
            .arg(Arg::with_name("FILENAME")
                .required(true)))
        .subcommand(SubCommand::with_name("debug")
            .arg(Arg::with_name("FILENAME")
                .required(true)))
        .get_matches();

    if let Some(sub_matches) = matches.subcommand_matches("spaceinvaders") {
        Options::SpaceInvaders {
            filename: String::from(sub_matches.value_of("FILENAME").unwrap()),
        }
    } else {
        if let Some(sub_matches) = matches.subcommand_matches("cpm") {
            Options::Cpm {
                filename: String::from(sub_matches.value_of("FILENAME").unwrap()),
            }
        } else {
            if let Some(sub_matches) = matches.subcommand_matches("dis") {
                let offset = sub_matches.value_of("OFFSET").unwrap_or("0").parse::<usize>().ok().expect("--offset is not valid usize");
                Options::Disassemble {
                    filename: String::from(sub_matches.value_of("FILENAME").unwrap()),
                    offset: offset,
                }
            } else {
                let sub_matches = matches.subcommand_matches("debug").unwrap();
                Options::Debug {
                    filename: String::from(sub_matches.value_of("FILENAME").unwrap()),
                }
            }
        }
    }
}

fn read_file(filename: &str) -> Vec<u8> {
    let mut file = File::open(filename).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    buf
}

fn main() {
    let options = get_opts();

    match options {
        Options::SpaceInvaders { filename } => {
            let buf = read_file(&filename);
            let path = PathBuf::from(filename);
            let dir = path.parent().expect("Given path has no parent directory.");

            let mut sound0 = Sound::new(dir.join("0.wav").to_str().unwrap())
                .expect("Could not load sound from `0.wav`.");
            sound0.set_looping(true);
            let sound1 = Sound::new(dir.join("1.wav").to_str().unwrap())
                .expect("Could not load sound from `1.wav`.");
            let sound2 = Sound::new(dir.join("2.wav").to_str().unwrap())
                .expect("Could not load sound from `2.wav`.");
            let sound3 = Sound::new(dir.join("3.wav").to_str().unwrap())
                .expect("Could not load sound from `3.wav`.");
            let sound4 = Sound::new(dir.join("4.wav").to_str().unwrap())
                .expect("Could not load sound from `4.wav`.");
            let sound5 = Sound::new(dir.join("5.wav").to_str().unwrap())
                .expect("Could not load sound from `5.wav`.");
            let sound6 = Sound::new(dir.join("6.wav").to_str().unwrap())
                .expect("Could not load sound from `6.wav`.");
            let sound7 = Sound::new(dir.join("7.wav").to_str().unwrap())
                .expect("Could not load sound from `7.wav`.");
            let sound8 = Sound::new(dir.join("8.wav").to_str().unwrap())
                .expect("Could not load sound from `8.wav`.");

            let mut machine = SpaceInvadersMachine::new(&buf,
                sound0, sound1, sound2, sound3, sound4,
                sound5, sound6, sound7, sound8);
            machine.run();
        }
        Options::Cpm { filename } => {
            let mut buf = read_file(&filename);
            let mut machine = cpm::Cpm::new(&buf);
            machine.cpu.pc = 0x100;
            machine.run();
        },
        Options::Disassemble { filename, offset } => {
            let buf = read_file(&filename);
            disassemble(&buf, offset);
        },
        Options::Debug { filename } => {},
    }
}
