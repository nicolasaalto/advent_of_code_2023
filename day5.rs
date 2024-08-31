use std::ops::Range;
mod data;
fn main() {
    let input = data::DATA;

    let (seeds, stages) = input.split_once("\n\n").unwrap();

    let seeds: Vec<i64> = seeds
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let stages = stages
        .split("\n\n")
        .map(|stage| {
            stage
                .lines()
                .skip(1)
                .map(|line| {
                    let mut parts = line
                        .split_whitespace()
                        .take(3)
                        .map(|c| c.parse::<i64>().unwrap());
                    let start_dest = parts.next().unwrap();
                    let start = parts.next().unwrap();
                    let length = parts.next().unwrap();
                    Mapping {
                        from: start..start + length,
                        offset: start_dest - start,
                    }
                })
                .collect::<Vec<Mapping>>()
        })
        .collect::<Vec<_>>();

    println!("Part 1: {:?}", part_one(&seeds, &stages));
    println!("Part 2: {:?}", part_two(&seeds, &stages));
}

struct Mapping {
    from: Range<i64>,
    offset: i64,
}

fn offset_range(r: Range<i64>, offset: i64) -> Range<i64> {
    (r.start + offset)..(r.end + offset)
}

fn map_number(mappings: &Vec<Mapping>, value: i64) -> i64 {
    for mapping in mappings {
        if mapping.from.contains(&value) {
            return value as i64 + mapping.offset;
        }
    }
    value as i64
}

fn map_range(mapping: &Mapping, i: Range<i64>) -> (Option<Range<i64>>, Vec<Range<i64>>) {
    let mut remainer = Vec::new();
    let mut res = None;
    // no intersecion
    if (i.end <= mapping.from.start) || (i.start >= mapping.from.end) {
        remainer.push(i);
        res = None;
    }
    // fully outside
    else if (i.start < mapping.from.start) && (i.end > mapping.from.end) {
        remainer.push(i.start..mapping.from.start);
        remainer.push(mapping.from.end..i.end);
        res = Some(offset_range(mapping.from.clone(), mapping.offset));
    }
    // fully inside
    else if (i.start >= mapping.from.start) && (i.end <= mapping.from.end) {
        res = Some(offset_range(i, mapping.offset));
    }
    // overlapping start
    else if i.start < mapping.from.start {
        remainer.push(i.start..mapping.from.start);
        res = Some(offset_range(mapping.from.start..i.end, mapping.offset));
    }
    // overlapping end
    else if i.end > mapping.from.end {
        res = Some(offset_range(i.start..mapping.from.end, mapping.offset));
        remainer.push(mapping.from.end..i.end);
    };
    (res, remainer)
}

fn part_one(seeds: &Vec<i64>, stages: &Vec<Vec<Mapping>>) -> i64 {
    seeds
        .iter()
        .cloned()
        .map(|s| {
            stages
                .iter()
                .fold(s, |acc, conversions| map_number(conversions, acc))
        })
        .min()
        .unwrap()
}

fn part_two(seeds: &Vec<i64>, stages: &Vec<Vec<Mapping>>) -> i64 {
    let ranges: Vec<Range<i64>> = seeds
        .chunks(2)
        .map(|pair| pair[0]..pair[0] + pair[1])
        .collect();

    let mut work = ranges.clone();
    for s in stages {
        let mut res = Vec::new();
        for c in s.iter() {
            let mut next_work = vec![];
            for i in work {
                let (result, remaining) = map_range(c, i);
                next_work.extend(remaining);
                if let Some(g) = result {
                    res.push(g);
                }
            }
            work = next_work;
        }
        res.extend(work);
        work = res;
    }

    work.iter().map(|r| r.start).min().unwrap()
}
