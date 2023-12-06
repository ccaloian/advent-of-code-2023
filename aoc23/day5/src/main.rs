use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::path::Path;
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();
    let (seeds, maps) = read_data("./data/input.txt");
    let min_loc = find_min_location(seeds, &maps);
    let duration_1 = start.elapsed();
    println!("Day 4, Part 1: {}", min_loc);
    println!("Time elapsed in find_min_location() is: {:?}", duration_1);
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

impl Maps {
    fn get_soil(&self, seed: u64) -> u64 {
        let seed_ranges: Vec<(&Range<u64>, &Range<u64>)> = self
            .seed_to_soil
            .iter()
            .filter(|(k, _)| k.contains(&seed))
            .collect();
        assert_eq!(seed_ranges.len(), 1);
        let (seed_range, soil_range) = seed_ranges[0];
        let seek = seed - seed_range.start;
        soil_range.start + seek
    }
}

fn get_mapped_id(id: u64, name: &str, maps: &HashMap<&str, RangeMap>) -> u64 {
    let src_ranges: Vec<(&Range<u64>, &Range<u64>)> =
        maps[name].iter().filter(|(k, _)| k.contains(&id)).collect();
    if src_ranges.is_empty() {
        return id;
    }
    let (src_range, dst_range) = src_ranges[0];
    let seek = id - src_range.start;
    dst_range.start + seek
}

fn seed_location(seed_id: u64, maps: &HashMap<&str, RangeMap>) -> u64 {
    let soil_id = get_mapped_id(seed_id, "seed-to-soil", maps);
    let fert_id = get_mapped_id(soil_id, "soil-to-fertilizer", maps);
    let water_id = get_mapped_id(fert_id, "fertilizer-to-water", maps);
    let light_id = get_mapped_id(water_id, "water-to-light", maps);
    let temp_id = get_mapped_id(light_id, "light-to-temperature", maps);
    let humid_id = get_mapped_id(temp_id, "temperature-to-humidity", maps);
    let loc_id = get_mapped_id(humid_id, "humidity-to-location", maps);
    loc_id
}

fn find_min_location(seeds: Vec<u64>, maps: &HashMap<&str, RangeMap>) -> u64 {
    seeds.iter().map(|s| seed_location(*s, maps)).min().unwrap()
}

fn read_data(filepath: &str) -> (Vec<u64>, HashMap<&str, RangeMap>) {
    let path = Path::new(filepath);
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    // read seeds
    let mut seeds_header = String::new();
    let _ = reader.read_line(&mut seeds_header);
    let seeds_s = seeds_header.split_once(':').unwrap().1.to_string();

    let seeds: Vec<u64> = seeds_s
        .split_whitespace()
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    // read maps
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

    // let maps = Maps {
    //     seed_to_soil,
    //     soil_to_fert,
    //     fert_to_water,
    //     water_to_light,
    //     light_to_temp,
    //     temp_to_humid,
    //     humid_to_loc,
    // };

    let mut maps: HashMap<&str, RangeMap> = HashMap::new();
    maps.insert("seed-to-soil", seed_to_soil);
    maps.insert("soil-to-fertilizer", soil_to_fert);
    maps.insert("fertilizer-to-water", fert_to_water);
    maps.insert("water-to-light", water_to_light);
    maps.insert("light-to-temperature", light_to_temp);
    maps.insert("temperature-to-humidity", temp_to_humid);
    maps.insert("humidity-to-location", humid_to_loc);

    (seeds, maps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_read_test_file() {
        let (seeds, maps) = read_data("./data/test_part1.txt");
        assert_eq!(seeds, vec![79, 14, 55, 13]);
        let mut keys_found = maps.keys().cloned().collect::<Vec<&str>>();
        let mut keys_expected = vec![
            "seed-to-soil",
            "soil-to-fertilizer",
            "fertilizer-to-water",
            "water-to-light",
            "light-to-temperature",
            "temperature-to-humidity",
            "humidity-to-location",
        ];
        keys_found.sort();
        keys_expected.sort();
        assert_eq!(keys_found, keys_expected);
    }

    #[test]
    fn part1_map_id() {
        let (_, maps) = read_data("./data/test_part1.txt");
        assert_eq!(get_mapped_id(79, "seed-to-soil", &maps), 81);
        assert_eq!(get_mapped_id(14, "seed-to-soil", &maps), 14);
        assert_eq!(get_mapped_id(55, "seed-to-soil", &maps), 57);
        assert_eq!(get_mapped_id(13, "seed-to-soil", &maps), 13);
    }

    #[test]
    fn part1_min_location() {
        let (seeds, maps) = read_data("./data/test_part1.txt");
        assert_eq!(find_min_location(seeds, &maps), 35);
    }
}
