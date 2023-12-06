use std::collections::HashMap;
use std::fs::{read, File};
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::path::Path;

fn main() {
    println!("Hello, world!");
}

type RangeMap = HashMap<Range<usize>, Range<usize>>;

fn read_data(filepath: &str) {
    let path = Path::new(filepath);
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    // read seeds
    let mut seeds_s = String::new();
    let _ = reader.read_line(&mut seeds_s);

    let mut str_to_map: HashMap<String, RangeMap> = HashMap::new();

    let mut seed_to_soil: RangeMap = HashMap::new();
    str_to_map.insert("seed-to-soil".to_string(), seed_to_soil);

    let mut soil_to_fert: RangeMap = HashMap::new();
    str_to_map.insert("soil-to-fertilizer".to_string(), soil_to_fert);

    let mut fert_to_water: RangeMap = HashMap::new();
    str_to_map.insert("fertilizer-to-water".to_string(), fert_to_water);

    let mut water_to_light: RangeMap = HashMap::new();
    str_to_map.insert("water-to-light".to_string(), water_to_light);

    let mut light_to_temp: RangeMap = HashMap::new();
    str_to_map.insert("light-to-temperature".to_string(), light_to_temp);

    let mut temp_to_humid: RangeMap = HashMap::new();
    str_to_map.insert("temperature-to-humidity".to_string(), temp_to_humid);

    let mut humid_to_loc: RangeMap = HashMap::new();
    str_to_map.insert("humidity-to-location".to_string(), humid_to_loc);

    let mut cur_map = &mut seed_to_soil;
    for line in reader.lines() {
        let row = line.unwrap();
        if row.is_empty() {
            continue;
        } else if row.contains("map") {
            let cur_map_s = row.split_once(' ').unwrap().0.to_string();
            let cur_map = &mut str_to_map.get(&cur_map_s).unwrap();
        } else {
            let data = row
                .split_whitespace()
                .map(str::parse::<usize>)
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            assert_eq!(data.len(), 3);
            let _ = cur_map.insert(
                Range {
                    start: data[1],
                    end: data[1] + data[2],
                },
                Range {
                    start: data[0],
                    end: data[0] + data[2],
                },
            );
        }
    }
}
