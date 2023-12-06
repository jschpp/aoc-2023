use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::{preceded, terminated},
    IResult, Parser,
};

fn seed_parser(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(tag("seeds: "), separated_list1(tag(" "), complete::u64))(input)
}

#[derive(Debug, Clone)]
pub struct MapData {
    pub destination_start: u64,
    pub source_start: u64,
    pub len: u64,
}

#[derive(Debug, Clone)]
pub struct Almanac {
    pub seeds: Vec<u64>,
    pub seed_to_soil: Vec<MapData>,
    pub soil_to_fertilizer: Vec<MapData>,
    pub fertilizer_to_water: Vec<MapData>,
    pub water_to_light: Vec<MapData>,
    pub light_to_temperature: Vec<MapData>,
    pub temperature_to_humidity: Vec<MapData>,
    pub humidity_to_location: Vec<MapData>,
}

fn map_parser(input: &str) -> IResult<&str, Vec<MapData>> {
    separated_list1(
        line_ending,
        separated_list1(tag(" "), complete::u64).map({
            |x| {
                assert_eq!(x.len(), 3);
                MapData {
                    destination_start: x[0],
                    source_start: x[1],
                    len: x[2],
                }
            }
        }),
    )(input)
}

pub fn almanac_parser(input: &str) -> IResult<&str, Almanac> {
    let (input, seeds) = terminated(seed_parser, line_ending)(input)?;

    let (input, _) = line_ending(input)?;
    let (input, _) = terminated(tag("seed-to-soil map:"), line_ending)(input)?;
    let (input, seed_to_soil) = terminated(map_parser, line_ending)(input)?;

    let (input, _) = line_ending(input)?;
    let (input, _) = terminated(tag("soil-to-fertilizer map:"), line_ending)(input)?;
    let (input, soil_to_fertilizer) = terminated(map_parser, line_ending)(input)?;

    let (input, _) = line_ending(input)?;
    let (input, _) = terminated(tag("fertilizer-to-water map:"), line_ending)(input)?;
    let (input, fertilizer_to_water) = terminated(map_parser, line_ending)(input)?;

    let (input, _) = line_ending(input)?;
    let (input, _) = terminated(tag("water-to-light map:"), line_ending)(input)?;
    let (input, water_to_light) = terminated(map_parser, line_ending)(input)?;

    let (input, _) = line_ending(input)?;
    let (input, _) = terminated(tag("light-to-temperature map:"), line_ending)(input)?;
    let (input, light_to_temperature) = terminated(map_parser, line_ending)(input)?;

    let (input, _) = line_ending(input)?;
    let (input, _) = terminated(tag("temperature-to-humidity map:"), line_ending)(input)?;
    let (input, temperature_to_humidity) = terminated(map_parser, line_ending)(input)?;

    let (input, _) = line_ending(input)?;
    let (input, _) = terminated(tag("humidity-to-location map:"), line_ending)(input)?;
    let (input, humidity_to_location) = map_parser(input)?;

    Ok((
        input,
        Almanac {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        },
    ))
}
