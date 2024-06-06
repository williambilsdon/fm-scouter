use std::{
    fs::{read_to_string, File},
    path::Path,
};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use csv::StringRecord;
use fm_scouter::{attribute::Weights, par_parse_records, parse_records, parse_weights};

fn parse_records_benchmark(c: &mut Criterion) {
    let mut record_parsing_group = c.benchmark_group("parsing records from scouted players");
    record_parsing_group.bench_function("single threaded", |b| {
        let csv_file = File::open("ScoutedPlayers.csv").unwrap();
        let mut rdr = csv::Reader::from_reader(csv_file);

        let string_records: Vec<StringRecord> = rdr
            .records()
            .collect::<Result<Vec<StringRecord>, csv::Error>>()
            .unwrap();

        let headers = { rdr.headers().unwrap().clone() };

        let weights_string = read_to_string(Path::new("./weights/advanced_forward.json")).unwrap();
        let weights: Weights = parse_weights(weights_string.as_str()).unwrap();

        b.iter(move || {
            parse_records(
                black_box(&string_records),
                black_box(&headers),
                black_box(&weights),
            )
        })
    });

    record_parsing_group.bench_function("in parallel", |b| {
        let csv_file = File::open("ScoutedPlayers.csv").unwrap();
        let mut rdr = csv::Reader::from_reader(csv_file);

        let string_records: Vec<StringRecord> = rdr
            .records()
            .collect::<Result<Vec<StringRecord>, csv::Error>>()
            .unwrap();

        let headers = { rdr.headers().unwrap().clone() };

        let weights_string = read_to_string(Path::new("./weights/advanced_forward.json")).unwrap();
        let weights: Weights = parse_weights(weights_string.as_str()).unwrap();

        b.iter(move || {
            par_parse_records(
                black_box(&string_records),
                black_box(&headers),
                black_box(&weights),
            )
        })
    });
    record_parsing_group.finish()
}

criterion_group!(benches, parse_records_benchmark);
criterion_main!(benches);
