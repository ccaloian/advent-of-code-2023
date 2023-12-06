use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::path::Path;

fn main() {
    let (seeds, maps) = read_data("./data/test_part1.txt");
    println!("seeds: {:?}", seeds);
    println!("maps: {:?}", maps);
}

type RangeMap = HashMap<Range<u64>, Range<u64>>;

#[derive(Debug)]
struct Maps {
    seed_to_soil: RangeMap,
    soil_to_fert: RangeMap,
    fert_to_water: RangeMap,
    water_to_light: RangeMap,
    light_to_temp: RangeMap,
    temp_to_humid: RangeMap,
    humid_to_loc: RangeMap,
}

fn read_data(filepath: &str) -> (Vec<u64>, Maps) {
    let path = Path::new(filepath);
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    // read seeds
    let mut seeds_header = String::new();
    let _ = reader.read_line(&mut seeds_header);
    println!("seeds_header: {:?}", seeds_header.trim());
    let seeds_s = seeds_header.split_once(':').unwrap().1.to_string();
    println!("seeds_s: {:?}", seeds_s.trim());

    let seeds: Vec<u64> = seeds_s
        .trim()
        .split_whitespace()
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut seed_to_soil: RangeMap = HashMap::new();
    let mut soil_to_fert: RangeMap = HashMap::new();
    let mut fert_to_water: RangeMap = HashMap::new();
    let mut water_to_light: RangeMap = HashMap::new();
    let mut light_to_temp: RangeMap = HashMap::new();
    let mut temp_to_humid: RangeMap = HashMap::new();
    let mut humid_to_loc: RangeMap = HashMap::new();

    let mut cur_header = "seed-to-soil map:".to_string();
    for line in reader.lines() {
        let row = line.unwrap();
        if row.is_empty() {
            continue;
        } else if row.contains("map") {
            cur_header = row.clone();
        } else {
            let data = row
                .split_whitespace()
                .map(str::parse::<u64>)
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            assert_eq!(data.len(), 3);
            let src_range = Range {
                start: data[1],
                end: data[1] + data[2],
            };
            let dest_range = Range {
                start: data[0],
                end: data[0] + data[2],
            };

            if cur_header == "seed-to-soil map:" {
                seed_to_soil.insert(src_range, dest_range)
            } else if cur_header == "soil-to-fertilizer map:" {
                soil_to_fert.insert(src_range, dest_range)
            } else if cur_header == "fertilizer-to-water map:" {
                fert_to_water.insert(src_range, dest_range)
            } else if cur_header == "water-to-light map:" {
                water_to_light.insert(src_range, dest_range)
            } else if cur_header == "light-to-temperature map:" {
                light_to_temp.insert(src_range, dest_range)
            } else if cur_header == "temperature-to-humidity map:" {
                temp_to_humid.insert(src_range, dest_range)
            } else if cur_header == "humidity-to-location map:" {
                humid_to_loc.insert(src_range, dest_range)
            } else {
                panic!("invalid row");
            };
        };
    }

    let maps = Maps {
        seed_to_soil,
        soil_to_fert,
        fert_to_water,
        water_to_light,
        light_to_temp,
        temp_to_humid,
        humid_to_loc,
    };

    (seeds, maps)
}
