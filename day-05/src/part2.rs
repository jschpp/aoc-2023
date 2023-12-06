use super::{my_parser::*, part1::mapping};

pub fn process(input: &str) -> anyhow::Result<String> {
    let (_, almanac) = almanac_parser(input).unwrap();
    let result = almanac
        .seeds
        .iter()
        .array_chunks::<2>()
        .map(|arr| (arr[0].to_owned()..arr[0].to_owned() + arr[1].to_owned()).into_iter())
        .flatten()
        .map(|seed| mapping(seed, &almanac.seed_to_soil))
        .map(|soil| mapping(soil, &almanac.soil_to_fertilizer))
        .map(|fertilizer| mapping(fertilizer, &almanac.fertilizer_to_water))
        .map(|water| mapping(water, &almanac.water_to_light))
        .map(|light| mapping(light, &almanac.light_to_temperature))
        .map(|temperature| mapping(temperature, &almanac.temperature_to_humidity))
        .map(|humidity| mapping(humidity, &almanac.humidity_to_location))
        .min()
        .expect("slice should not be empty");
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!("46", process(input)?);
        Ok(())
    }
}
