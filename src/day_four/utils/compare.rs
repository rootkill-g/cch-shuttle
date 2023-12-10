use super::super::routes::Reindeer;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ContestResult {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

pub fn compare(contest_reindeers: Vec<Reindeer>) -> ContestResult {
    let fastest = contest_reindeers
        .iter()
        .max_by(|r1, r2| r1.speed.total_cmp(&r2.speed))
        .unwrap();

    let tallest = contest_reindeers.iter().max_by_key(|r| r.height).unwrap();

    let magician = contest_reindeers
        .iter()
        .max_by_key(|r| r.snow_magic_power)
        .unwrap();

    let consumer = contest_reindeers
        .iter()
        .max_by_key(|r| r.candies_eaten_yesterday)
        .unwrap();

    ContestResult {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.strength, fastest.name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.name, tallest.antler_width
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician.name, magician.snow_magic_power
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            consumer.name, consumer.favorite_food
        ),
    }
}
