use std::{error::Error, fs::File, process};

use clap::Parser;

#[derive(Debug)]
struct Player {
    name: String,
    info: Option<String>,
    position: String,
    attributes: Attributes,
}

#[derive(Debug)]
struct Attributes {
    workrate: u8,
    vision: u8,
    throwing: u8,
    technique: u8,
    teamwork: u8,
    tackling: u8,
    strength: u8,
    stamina: u8,
    rushing_out: u8,
    reflexes: u8,
    punching: u8,
    positioning: u8,
    penalties: u8,
    passing: u8,
    pace: u8,
    one_vs_one: u8,
    off_the_ball: u8,
    natural_fitness: u8,
    marking: u8,
    long_throws: u8,
    long_shots: u8,
    leadership: u8,
    kicking: u8,
    jumping: u8,
    heading: u8,
    handling: u8,
    free_kicks: u8,
    flair: u8,
    first_touch: u8,
    finishing: u8,
    eccentricity: u8,
    dribbling: u8,
    determination: u8,
    decision_making: u8,
    crossing: u8,
    corners: u8,
    age: u8,
    concentration: u8,
    composure: u8,
    communication: u8,
    command_of_area: u8,
    bravery: u8,
    balance: u8,
    anticipation: u8,
    agility: u8,
    aggression: u8,
    aerial: u8,
    acceleration: u8,
}

type PlayerStats = Vec<String>;

fn parse_csv(current_squad_file: File) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(current_squad_file);
    println!("{:?}", rdr.headers().unwrap());

    let mut headers_vec: Vec<(usize, String)> = Vec::new();

    for (idx, header) in rdr.headers()?.iter().enumerate() {
        headers_vec.push((idx, header.to_string()))
    }

    let player_vec: Vec<PlayerStats> = rdr
        .records()
        .map(|record_result| {
            record_result
                .expect("error parsing record result")
                .iter()
                .map(|s| s.to_string())
                .collect()
        })
        .collect();

    let players: Vec<Player> = player_vec
        .iter()
        .map(|player| {
            let mut player_iter = player.iter();
            Player {
                name: player_iter.next().unwrap().to_owned(),
                info: Some(player_iter.next().unwrap().to_owned()),
                position: player_iter.next().unwrap().to_owned(),
                attributes: Attributes {
                    workrate: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    vision: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    throwing: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    technique: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    teamwork: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    tackling: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    strength: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    stamina: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    rushing_out: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    reflexes: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    punching: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    positioning: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    penalties: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    passing: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    pace: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    one_vs_one: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    off_the_ball: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    natural_fitness: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    marking: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    long_throws: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    long_shots: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    leadership: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    kicking: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    jumping: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    heading: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    handling: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    free_kicks: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    flair: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    first_touch: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    finishing: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    eccentricity: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    dribbling: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    determination: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    decision_making: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    crossing: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    corners: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    age: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    concentration: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    composure: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    communication: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    command_of_area: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    bravery: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    balance: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    anticipation: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    agility: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    aggression: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    aerial: player_iter.next().unwrap().to_owned().parse().unwrap(),
                    acceleration: player_iter.next().unwrap().to_owned().parse().unwrap(),
                },
            }
        })
        .collect();

    for player in player_vec {
        for header in &headers_vec {
            print!("{}: {}, ", header.1, player[header.0]);
        }
        println!("\n");
    }
    // println!("{:?}", players);
    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    current_squad_file_path: String,
}

fn main() {
    let args = Args::parse();

    let current_squad_file = File::open(args.current_squad_file_path).unwrap();

    if let Err(err) = parse_csv(current_squad_file) {
        println!("error parsing current squad csv: {}", err);
        process::exit(1);
    }
}
