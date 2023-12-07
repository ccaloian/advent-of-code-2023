use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Range, RangeTo};
use std::path::Path;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let (seeds, maps) = read_data("./data/input.txt");
    let min_loc = find_min_location(&seeds, &maps);
    println!("Day 5, Part 1: {}", min_loc);

    // let seed_ranges = get_seed_ranges(&seeds);
    // let min_loc_with_ranges = find_min_location_with_ranges(seed_ranges, &maps);
    // println!("Day 5, Part 2: {}", min_loc_with_ranges);

    let duration = start.elapsed();
    println!("Time elapsed in find_min_location() is: {:?}", duration);
}

type RangeMap = HashMap<Range<u64>, Range<u64>>;

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

fn find_min_location(seeds: &[u64], maps: &HashMap<&str, RangeMap>) -> u64 {
    seeds.iter().map(|s| seed_location(*s, maps)).min().unwrap()
}

fn get_seed_ranges(seeds: &[u64]) -> Vec<Range<u64>> {
    let mut seed_ranges: Vec<Range<u64>> = Vec::new();
    assert_eq!(seeds.len() % 2, 0);
    for i in (0..seeds.len()).step_by(2) {
        seed_ranges.push(Range {
            start: seeds[i],
            end: seeds[i] + seeds[i + 1],
        })
    }
    seed_ranges
}

fn get_max_seed_id(seeds: &[Range<u64>]) -> u64 {
    seeds.iter().map(|s| s.end).max().unwrap()
}

fn filter_seed_ranges_by_input(
    input_ranges: &[Range<u64>],
    seed_ranges: &[Range<u64>],
) -> Vec<Range<u64>> {
    // let mut ranges_filt: Vec<Range<u64>> = Vec::new();
    let ranges_filt = input_ranges.iter().flat_map(|src| {
        seed_ranges
            .iter()
            .filter(|dst| overlap(src, dst))
            .map(|dst| Range {
                start: max(src.start, dst.start),
                end: min(src.end, dst.end),
            })
            .collect::<Vec<Range<u64>>>()
    });
    ranges_filt.collect()
}

// fn find_min_location_for_range(seed_range: Range<u64>, maps: &HashMap<&str, RangeMap>) -> u64 {
//     seed_range.map(|s| seed_location(s, maps)).min().unwrap()
// }

// fn find_min_location_with_ranges(
//     seed_ranges: Vec<Range<u64>>,
//     maps: &HashMap<&str, RangeMap>,
// ) -> u64 {
//     seed_ranges
//         .into_iter()
//         .map(|r| find_min_location_for_range(r, maps))
//         .min()
//         .unwrap()
// }

fn overlap(src: &Range<u64>, dst: &Range<u64>) -> bool {
    (src.start <= dst.end) && (src.end >= dst.start)
}

/// Return a tuple (src_left, src_overlap, src_right).
fn extract_overlap(src: &Range<u64>, dst: &Range<u64>) -> Range<u64> {
    // ) -> (Option<Range<u64>>, Range<u64>, Option<Range<u64>>) {
    let overlap = Range {
        start: max(src.start, dst.start),
        end: min(src.end, dst.end),
    };

    let mut src_left = None;
    if src.start < dst.start {
        src_left = Some(Range {
            start: src.start,
            end: dst.start,
        });
    }

    let mut src_right = None;
    if src.end > dst.end {
        src_right = Some(Range {
            start: dst.end,
            end: src.end,
        });
    }

    // (src_left, overlap, src_right)
    overlap
}

fn get_one_to_one_ranges(src: &[Range<u64>], dst: &[Range<u64>], range_end: u64) -> RangeMap {
    let mut src_clone = src.iter().map(|s| s.clone()).collect::<Vec<Range<u64>>>();
    let mut dst_clone = dst.iter().map(|d| d.clone()).collect::<Vec<Range<u64>>>();

    let mut one_to_one: RangeMap = HashMap::new();
    src_clone.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());
    dst_clone.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());

    // Add one_to_one ranges that map 1-1
    for i in 0..src_clone.len() - 1 {
        one_to_one.insert(
            Range {
                start: src_clone[i].end,
                end: src_clone[i + 1].start,
            },
            Range {
                start: dst_clone[i].end,
                end: dst_clone[i + 1].start,
            },
        );
    }

    // Add 1-1 range from 0 to the start of the first interval
    if src_clone[0].start > 0 {
        one_to_one.insert(
            Range {
                start: 0,
                end: src_clone[0].start,
            },
            Range {
                start: 0,
                end: dst_clone[0].start,
            },
        );
    }

    // Add 1-1 range from the end of the last interval until range_end
    if src_clone.last().unwrap().end < range_end {
        one_to_one.insert(
            Range {
                start: src_clone.last().unwrap().end,
                end: range_end,
            },
            Range {
                start: dst_clone.last().unwrap().end,
                end: range_end,
            },
        );
    }

    one_to_one
}

fn complete_map_ranges(rmap: RangeMap, max_range: u64) -> RangeMap {
    let src = rmap.keys().cloned().collect::<Vec<Range<u64>>>();
    let dst = rmap.values().cloned().collect::<Vec<Range<u64>>>();

    let mut complete: RangeMap = get_one_to_one_ranges(&src, &dst, max_range);
    let _ = rmap.into_iter().map(|(s, d)| complete.insert(s, d));

    complete
}

fn map_input_to_seeds(inputs: Vec<Range<u64>>, seeds: Vec<Range<u64>>) -> Vec<Range<u64>> {
    // filter seed ranges by inputs (to further filter soil ranges,..., to filter locations)
    let seed_ranges: Vec<Range<u64>> = Vec::new();
    seed_ranges
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
        assert_eq!(find_min_location(&seeds, &maps), 35);
    }

    #[test]
    fn part2_seed_ranges() {
        let (seeds, _) = read_data("./data/test_part1.txt");
        let seed_ranges = get_seed_ranges(&seeds);
        assert_eq!(
            seed_ranges,
            vec![
                Range {
                    start: 79,
                    end: 79 + 14
                },
                Range {
                    start: 55,
                    end: 55 + 13
                }
            ]
        );
    }

    // #[test]
    // fn part2_min_location() {
    //     let (seeds, maps) = read_data("./data/test_part2.txt");
    //     let seed_ranges = get_seed_ranges(&seeds);
    //     let min_loc_with_ranges = find_min_location_with_ranges(seed_ranges, &maps);
    //     assert_eq!(min_loc_with_ranges, 46);
    // }

    #[test]
    fn filter_seed_ranges() {
        let inputs = vec![
            Range { start: 2, end: 8 },
            Range { start: 12, end: 22 },
            Range { start: 34, end: 49 },
            Range { start: 55, end: 65 },
        ];
        let seeds = vec![
            Range { start: 0, end: 5 },
            Range { start: 5, end: 16 },
            Range { start: 16, end: 19 },
            Range { start: 19, end: 30 },
            Range { start: 30, end: 38 },
            Range { start: 38, end: 42 },
            Range { start: 42, end: 60 },
            Range { start: 60, end: 65 }, // test for getting this 1-1 interval
        ];
        let expected_ranges: Vec<Range<u64>> = vec![
            Range { start: 2, end: 5 },
            Range { start: 5, end: 8 },
            Range { start: 12, end: 16 },
            Range { start: 16, end: 19 },
            Range { start: 19, end: 22 },
            Range { start: 34, end: 38 },
            Range { start: 38, end: 42 },
            Range { start: 42, end: 49 },
            Range { start: 55, end: 60 },
            Range { start: 60, end: 65 },
        ];
        let filtered_ranges = filter_seed_ranges_by_input(&inputs, &seeds);
        assert_eq!(filtered_ranges, expected_ranges);
    }
}
