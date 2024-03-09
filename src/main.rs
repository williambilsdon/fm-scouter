use std::{error::Error, fs::File, process};

use clap::Parser;

struct Player {
    name: String,
    inf: Option<String>,
    position: String,
    attributes: Attributes,
}

struct Attributes {
    wor: u8,
    vis: u8,
    thr: u8,
    tec: u8,
    tea: u8,
    tck: u8,
    str: u8,
    sta: u8,
    tro: u8,
    reflexes: u8,
    pun: u8,
    pos: u8,
    pen: u8,
    pas: u8,
    pac: u8,
    onevsone: u8,
    otb: u8,
    nat: u8,
    mar: u8,
    lth: u8,
    lon: u8,
    ldr: u8,
    kic: u8,
    jum: u8,
    hea: u8,
    han: u8,
    fre: u8,
    fla: u8,
    fir: u8,
    fin: u8,
    ecc: u8,
    dri: u8,
    det: u8,
    dec: u8,
    cro: u8,
    cor: u8,
    age: u8,
    cnt: u8,
    cmp: u8,
    com: u8,
    cmd: u8,
    bra: u8,
    bal: u8,
    ant: u8,
    agi: u8,
    agg: u8,
    aer: u8,
    acc: u8,
}

fn parse_csv(current_squad_file: File) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(current_squad_file);
    println!("{:?}", rdr.headers());
    for result in rdr.records() {
        let record_sr = result?;
        for record in record_sr.iter() {
            print!(" {:?}", record);
        }
        print!("\n");
    }
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
        println!("error running example: {}", err);
        process::exit(1);
    }
}
