use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::path::Path;
use std::time::Instant;

fn main() {
    let start_part1 = Instant::now();

    let (seeds, seed_ranges, maps) = read_data("./data/input.txt");
    let min_loc = find_min_location(&seeds, &maps);
    println!("Day 5, Part 1: {}", min_loc);

    let duration = start_part1.elapsed();
    println!("Time elapsed in find_min_location() is: {:?}", duration);

    let start_part2 = Instant::now();

    let min_loc_with_ranges = find_min_location_for_ranges(seed_ranges, &maps);
    println!("Day 5, Part 2: {}", min_loc_with_ranges);

    let duration = start_part2.elapsed();
    println!(
        "Time elapsed in find_min_location_for_ranges() is: {:?}",
        duration
    );
}

type RangeMap = HashMap<Range<u64>, Range<u64>>;

fn get_mapped_id(id: u64, name: &str, maps: &HashMap<&str, RangeMap>) -> u64 {
    let src_ranges: Vec<(&Range<u64>, &Range<u64>)> =
        maps[name].iter().filter(|(k, _)| k.contains(&id)).collect();
    if src_ranges.is_empty() {
        return id;
    }
    let (src_range, dst_range) = src_ranges[0];
    let shift_by = id - src_range.start;
    dst_range.start + shift_by
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

/// Return the minimum location id for a vector of query seed ranges. `maps` is assumed to have
/// complete coverage.
fn find_min_location_for_ranges(
    range_queries: Vec<Range<u64>>,
    maps: &HashMap<&str, RangeMap>,
) -> u64 {
    let soil_range_chunk = get_mapped_range(&range_queries, "seed-to-soil", maps);
    let fert_range_chunk = get_mapped_range(&soil_range_chunk, "soil-to-fertilizer", maps);
    let water_range_chunk = get_mapped_range(&fert_range_chunk, "fertilizer-to-water", maps);
    let light_range_chunk = get_mapped_range(&water_range_chunk, "water-to-light", maps);
    let temp_range_chunk = get_mapped_range(&light_range_chunk, "light-to-temperature", maps);
    let humid_range_chunk = get_mapped_range(&temp_range_chunk, "temperature-to-humidity", maps);
    let loc_range_chunk = get_mapped_range(&humid_range_chunk, "humidity-to-location", maps);

    loc_range_chunk.iter().map(|r| r.start).min().unwrap()
}

/// Return the query seed ids as ranges.
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

/// Return the largest seed id value from the query seed ranges.
fn get_max_seed_id(seeds: &[Range<u64>]) -> u64 {
    seeds.iter().map(|s| s.end).max().unwrap()
}

/// Given the query seed ranges, filter the "completed" input seed ranges and return only the the
/// vector of sub-ranges that cover the query.
fn filter_ranges_by_query(
    query_ranges: &[Range<u64>],
    seed_ranges: &[Range<u64>],
) -> Vec<Range<u64>> {
    // let mut ranges_filt: Vec<Range<u64>> = Vec::new();
    let ranges_filt = query_ranges.iter().flat_map(|src| {
        seed_ranges
            .iter()
            .filter(|dst| overlap(src, dst))
            .map(|dst| extract_overlap(src, dst))
            .collect::<Vec<Range<u64>>>()
    });
    ranges_filt.collect()
}

/// Given a range as input, return the corresponding mapped values as a range.
/// This is similar to mapping an id to another id, just for ranges.
fn get_mapped_range(
    query_ranges: &[Range<u64>],
    name: &str,
    maps: &HashMap<&str, RangeMap>,
) -> Vec<Range<u64>> {
    // break down source ranges into chunks
    let src_ranges = &maps[name].keys().cloned().collect::<Vec<Range<u64>>>();

    // keep only relevant chunks of seed ranges from the input
    let src_range_chunks = filter_ranges_by_query(query_ranges, &src_ranges);

    // for each chunk, find it's mapping
    let mut mapped_chunks: Vec<Range<u64>> = Vec::new();
    for src_chunk in &src_range_chunks {
        for (src_range, dst_range) in &maps[name] {
            if overlap(src_chunk, src_range) {
                // assert!(src_chunk.start >= src_range.start);
                let shift_by = src_chunk.start - src_range.start;
                let range_len = src_chunk.end - src_chunk.start;

                mapped_chunks.push(Range {
                    start: dst_range.start + shift_by,
                    end: dst_range.start + shift_by + range_len,
                });
            }
        }
    }
    mapped_chunks
}

/// Return true if src range overlaps the dst range, false otherwise.
fn overlap(src: &Range<u64>, dst: &Range<u64>) -> bool {
    (src.start < dst.end) && (src.end > dst.start)
}

/// Return a range of the overlapping region of src range onto dst range.
fn extract_overlap(src: &Range<u64>, dst: &Range<u64>) -> Range<u64> {
    Range {
        start: max(src.start, dst.start),
        end: min(src.end, dst.end),
    }
}

/// Return all the 1-1 mappings that are missing from the input. This will be used to "cover" the
/// ranges axes, from  0..max_value, where the max_value comes from the seeds query vector.
fn get_one_to_one_ranges(src: &[Range<u64>], max_id: u64) -> RangeMap {
    // only source ranges (keys) are needed since the maps are 1-1
    let mut src_clone = src.iter().map(|s| s.clone()).collect::<Vec<Range<u64>>>();
    // let mut dst_clone = dst.iter().map(|d| d.clone()).collect::<Vec<Range<u64>>>();
    src_clone.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());
    // dst_clone.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());

    let mut one_to_one: RangeMap = HashMap::new();

    // Add one_to_one ranges that map 1-1
    for i in 0..src_clone.len() - 1 {
        // if there is a gap between ranges
        if src_clone[i].end < src_clone[i + 1].start {
            one_to_one.insert(
                Range {
                    start: src_clone[i].end,
                    end: src_clone[i + 1].start,
                },
                Range {
                    start: src_clone[i].end,
                    end: src_clone[i + 1].start,
                },
            );
        }
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
                end: src_clone[0].start,
            },
        );
    }

    // Add 1-1 range from the end of the last interval until range_end
    if src_clone.last().unwrap().end < max_id {
        one_to_one.insert(
            Range {
                start: src_clone.last().unwrap().end,
                end: max_id,
            },
            Range {
                start: src_clone.last().unwrap().end,
                end: max_id,
            },
        );
    }

    one_to_one
}

/// Given an input range map, _e.g._ `seeds-to-soil`, return a new range map with "full coverage",
/// _i.e._ no gaps between the input ranges. The `max_id` is needed for the case in which
/// the query seed ranges go beyond the given input seed ranges.
fn complete_map_ranges(rmap: RangeMap, max_id: u64) -> RangeMap {
    let src = rmap.keys().cloned().collect::<Vec<Range<u64>>>();
    // let dst = rmap.values().cloned().collect::<Vec<Range<u64>>>();

    let mut complete: RangeMap = get_one_to_one_ranges(&src, max_id);
    for (s, d) in rmap {
        complete.insert(s, d);
    }

    complete
}

/// Read input data and return a vector of query seeds, and a map of map names to mapped ranges.
fn read_data(filepath: &str) -> (Vec<u64>, Vec<Range<u64>>, HashMap<&str, RangeMap>) {
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
            // destination range start is first (e.g. soil in seed-to-soil map)
            let dest_range = Range {
                start: data[0],
                end: data[0] + data[2],
            };
            // source range start is first (e.g. seed in seed-to-soil map)
            let src_range = Range {
                start: data[1],
                end: data[1] + data[2],
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

    let seed_ranges = get_seed_ranges(&seeds);
    let max_seed_id = get_max_seed_id(&seed_ranges);

    let mut maps: HashMap<&str, RangeMap> = HashMap::new();
    maps.insert(
        "seed-to-soil",
        complete_map_ranges(seed_to_soil, max_seed_id),
    );
    maps.insert(
        "soil-to-fertilizer",
        complete_map_ranges(soil_to_fert, max_seed_id),
    );
    maps.insert(
        "fertilizer-to-water",
        complete_map_ranges(fert_to_water, max_seed_id),
    );
    maps.insert(
        "water-to-light",
        complete_map_ranges(water_to_light, max_seed_id),
    );
    maps.insert(
        "light-to-temperature",
        complete_map_ranges(light_to_temp, max_seed_id),
    );
    maps.insert(
        "temperature-to-humidity",
        complete_map_ranges(temp_to_humid, max_seed_id),
    );
    maps.insert(
        "humidity-to-location",
        complete_map_ranges(humid_to_loc, max_seed_id),
    );

    (seeds, seed_ranges, maps)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn part1_read_test_file() {
    //     let (seeds, _, maps) = read_data("./data/test_part1.txt");
    //     assert_eq!(seeds, vec![79, 14, 55, 13]);
    //     let mut keys_found = maps.keys().cloned().collect::<Vec<&str>>();
    //     let mut keys_expected = vec![
    //         "seed-to-soil",
    //         "soil-to-fertilizer",
    //         "fertilizer-to-water",
    //         "water-to-light",
    //         "light-to-temperature",
    //         "temperature-to-humidity",
    //         "humidity-to-location",
    //     ];
    //     keys_found.sort();
    //     keys_expected.sort();
    //     assert_eq!(keys_found, keys_expected);
    // }

    // #[test]
    // fn part1_map_id() {
    //     let (_, _, maps) = read_data("./data/test_part1.txt");
    //     assert_eq!(get_mapped_id(79, "seed-to-soil", &maps), 81);
    //     assert_eq!(get_mapped_id(14, "seed-to-soil", &maps), 14);
    //     assert_eq!(get_mapped_id(55, "seed-to-soil", &maps), 57);
    //     assert_eq!(get_mapped_id(13, "seed-to-soil", &maps), 13);
    // }

    // #[test]
    // fn part1_min_location() {
    //     let (seeds, _, maps) = read_data("./data/test_part1.txt");
    //     assert_eq!(find_min_location(&seeds, &maps), 35);
    // }

    // #[test]
    // fn part2_seed_ranges() {
    //     let (seeds, _, _) = read_data("./data/test_part1.txt");
    //     let seed_ranges = get_seed_ranges(&seeds);
    //     assert_eq!(
    //         seed_ranges,
    //         vec![
    //             Range {
    //                 start: 79,
    //                 end: 79 + 14
    //             },
    //             Range {
    //                 start: 55,
    //                 end: 55 + 13
    //             }
    //         ]
    //     );
    // }

    #[test]
    fn part2_complete_ranges() {
        let mut input_map: HashMap<Range<u64>, Range<u64>> = HashMap::new();
        input_map.insert(
            Range { start: 50, end: 52 },
            Range {
                start: 98,
                end: 100,
            },
        );
        input_map.insert(
            Range {
                start: 60,
                end: 100,
            },
            Range { start: 50, end: 90 },
        );
        let mut expected: HashMap<Range<u64>, Range<u64>> = HashMap::new();

        expected.insert(Range { start: 0, end: 50 }, Range { start: 0, end: 50 });
        expected.insert(
            Range { start: 50, end: 52 },
            Range {
                start: 98,
                end: 100,
            },
        );
        expected.insert(Range { start: 52, end: 60 }, Range { start: 52, end: 60 });
        expected.insert(
            Range {
                start: 60,
                end: 100,
            },
            Range { start: 50, end: 90 },
        );
        expected.insert(
            Range {
                start: 100,
                end: 120,
            },
            Range {
                start: 100,
                end: 120,
            },
        );
        let complete = complete_map_ranges(input_map, 120);
        assert_eq!(complete, expected);
    }

    #[test]
    fn part2_filter_seed_ranges() {
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
            Range { start: 60, end: 65 },
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
        let filtered_ranges = filter_ranges_by_query(&inputs, &seeds);
        assert_eq!(filtered_ranges, expected_ranges);
    }

    #[test]
    fn part2_map_ranges() {
        let mut rmap: RangeMap = HashMap::new();
        rmap.insert(Range { start: 0, end: 50 }, Range { start: 0, end: 50 });
        rmap.insert(
            Range { start: 50, end: 52 },
            Range {
                start: 98,
                end: 100,
            },
        );
        rmap.insert(Range { start: 52, end: 60 }, Range { start: 90, end: 98 });
        rmap.insert(
            Range {
                start: 60,
                end: 100,
            },
            Range { start: 50, end: 90 },
        );
        rmap.insert(
            Range {
                start: 100,
                end: 120,
            },
            Range {
                start: 100,
                end: 120,
            },
        );

        let mut rmaps: HashMap<&str, RangeMap> = HashMap::new();
        rmaps.insert("seeds-to-soil", rmap);

        let seed_ranges_query: Vec<Range<u64>> = vec![
            Range { start: 30, end: 50 },
            Range { start: 50, end: 52 },
            Range { start: 52, end: 60 },
            Range { start: 60, end: 70 },
            Range {
                start: 80,
                end: 100,
            },
            Range {
                start: 100,
                end: 110,
            },
        ];

        let expected: Vec<Range<u64>> = vec![
            Range { start: 30, end: 50 },
            Range {
                start: 98,
                end: 100,
            },
            Range { start: 90, end: 98 },
            Range { start: 50, end: 60 },
            Range { start: 70, end: 90 },
            Range {
                start: 100,
                end: 110,
            },
        ];

        let mapped_ranges = get_mapped_range(&seed_ranges_query, "seeds-to-soil", &rmaps);
        assert_eq!(mapped_ranges, expected);
    }
}
