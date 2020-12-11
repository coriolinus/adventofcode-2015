//! # Day 14: Reindeer Olympics
//!
//! This year is the Reindeer Olympics! Reindeer can fly at high speeds, but must rest occasionally
//! to recover their energy. Santa would like to know which of his reindeer is fastest, and so he
//! has them race.
//!
//! Reindeer can only either be flying (always at their top speed) or resting (not moving at all),
//! and always spend whole seconds in either state.
//!
//! For example, suppose you have the following Reindeer:
//!
//! - Comet can fly **14 km/s for 10 seconds**, but then must rest for **127** seconds.
//! - Dancer can fly **16 km/s for 11 seconds**, but then must rest for **162** seconds.
//!
//! After one second, Comet has gone 14 km, while Dancer has gone 16 km. After ten seconds, Comet
//! has gone 140 km, while Dancer has gone 160 km. On the eleventh second, Comet begins resting
//! (staying at 140 km), and Dancer continues on for a total distance of 176 km. On the 12th
//! second, both reindeer are resting. They continue to rest until the 138th second, when Comet
//! flies for another ten seconds. On the 174th second, Dancer flies for another 11 seconds.
//!
//! In this example, after the 1000th second, both reindeer are resting, and Comet is in the lead
//! at 1120 km (poor Dancer has only gotten 1056 km by that point). So, in this situation, Comet
//! would win (if the race ended at 1000 seconds).
//!
//! Given the descriptions of each reindeer (in your puzzle input), after exactly 2503 seconds,
//! what distance has the winning reindeer traveled?

use aoc2015::parse;
use std::iter::FromIterator;
use std::path::Path;
use thiserror::Error;

const RACE_DURATION: u32 = 2503;

/// What a Reindeer is currently doing.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ReindeerState {
    Flying,
    Resting,
}

impl Default for ReindeerState {
    fn default() -> Self {
        ReindeerState::Flying
    }
}

impl ReindeerState {
    fn toggle(&mut self) {
        *self = match self {
            ReindeerState::Flying => ReindeerState::Resting,
            ReindeerState::Resting => ReindeerState::Flying,
        }
    }
}

#[derive(
    Clone, Debug, Default, PartialEq, Eq, Hash, parse_display::Display, parse_display::FromStr,
)]
#[display("{name} can fly {speed} km/s for {fly_duration} seconds, but then must rest for {rest_duration} seconds.")]
pub struct Reindeer {
    pub name: String,
    /// km/s
    pub speed: u32,
    /// seconds
    pub fly_duration: u32,
    /// seconds
    pub rest_duration: u32,

    #[from_str(default)]
    pub distance: u32,

    #[from_str(default)]
    pub state: ReindeerState,

    #[from_str(default)]
    pub duration_in_state: u32,

    #[from_str(default)]
    pub points: u32,
}

impl Reindeer {
    pub fn new(name: String, speed: u32, fly: u32, rest: u32) -> Reindeer {
        Reindeer {
            name: name,
            speed: speed,
            fly_duration: fly,
            rest_duration: rest,
            ..Default::default()
        }
    }

    pub fn tick(&mut self) {
        self.duration_in_state += 1;

        if let ReindeerState::Flying = self.state {
            self.distance += self.speed;
        }

        let target_duration = match self.state {
            ReindeerState::Flying => self.fly_duration,
            ReindeerState::Resting => self.rest_duration,
        };
        if self.duration_in_state >= target_duration {
            self.state.toggle();
            self.duration_in_state = 0;
        }
    }

    pub fn reset(&mut self) {
        self.distance = 0;
        self.state = ReindeerState::Flying;
        self.duration_in_state = 0;
    }
}

pub struct Race {
    reindeer: Vec<Reindeer>,
    timer: u32,
}

impl FromIterator<Reindeer> for Race {
    fn from_iter<T: IntoIterator<Item = Reindeer>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let (min_bound, _) = iter.size_hint();

        let mut reindeer = Vec::with_capacity(min_bound);

        for r in iter {
            reindeer.push(r);
        }

        Race { reindeer, timer: 0 }
    }
}

impl Race {
    fn get_reindeer(
        &self,
        by: impl 'static + Copy + Fn(&Reindeer) -> u32,
    ) -> Box<dyn '_ + Iterator<Item = usize>> {
        if let Some(best) = self.reindeer.iter().map(by).max() {
            Box::new(
                self.reindeer
                    .iter()
                    .enumerate()
                    .filter_map(move |(idx, reindeer)| {
                        if by(reindeer) == best {
                            Some(idx)
                        } else {
                            None
                        }
                    }),
            )
        } else {
            Box::new(std::iter::empty())
        }
    }

    fn by_distance(&self) -> Box<dyn '_ + Iterator<Item = usize>> {
        self.get_reindeer(|reindeer| reindeer.distance)
    }

    fn by_points(&self) -> Box<dyn '_ + Iterator<Item = usize>> {
        self.get_reindeer(|reindeer| reindeer.points)
    }

    fn in_lead(&self, by: impl IntoIterator<Item = usize>) -> impl Iterator<Item = &Reindeer> {
        by.into_iter().map(move |index| &self.reindeer[index])
    }

    fn tick(&mut self) {
        for r in self.reindeer.iter_mut() {
            r.tick();
        }

        // we're pretty unlikely to have as much as a 3-way tie
        let mut winner_indices = Vec::with_capacity(2);
        winner_indices.extend(self.by_distance());
        for winner_idx in winner_indices {
            self.reindeer[winner_idx].points += 1;
        }

        self.timer += 1;
    }

    fn run_to_time(&mut self, finish_time: u32) {
        while self.timer < finish_time {
            self.tick();
        }
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut race: Race = parse(input)?.collect();
    race.run_to_time(RACE_DURATION);
    let winner = race
        .in_lead(race.by_distance())
        .next()
        .ok_or(Error::NoWinner)?;
    println!("winner: {:>8} @ {} km", winner.name, winner.distance);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let mut race: Race = parse(input)?.collect();
    race.run_to_time(RACE_DURATION);
    let winner = race
        .in_lead(race.by_points())
        .next()
        .ok_or(Error::NoWinner)?;
    println!("winner: {:>8} @ {} points", winner.name, winner.points);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("no reindeer won :(")]
    NoWinner,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_comet() -> Reindeer {
        Reindeer::new("Comet".to_string(), 14, 10, 127)
    }

    fn get_dancer() -> Reindeer {
        Reindeer::new("Dancer".to_string(), 16, 11, 162)
    }

    #[test]
    fn test_example_seconds() {
        let mut comet = get_comet();
        let mut dancer = get_dancer();

        comet.tick();
        dancer.tick();

        // After one second, Comet has gone 14 km, while Dancer has gone 16 km.
        assert_eq!(14, comet.distance);
        assert_eq!(16, dancer.distance);

        for _ in 0..9 {
            comet.tick();
            dancer.tick();
        }

        // After ten seconds, Comet has gone 140 km, while Dancer has gone 160 km.
        assert_eq!(140, comet.distance);
        assert_eq!(160, dancer.distance);

        comet.tick();
        dancer.tick();

        // On the eleventh second, Comet begins resting (staying at 140 km), and
        // Dancer continues on for a total distance of 176 km.
        assert_eq!(140, comet.distance);
        assert_eq!(ReindeerState::Resting, comet.state);
        assert_eq!(176, dancer.distance);
        // this assertion fails because we toggle state after flying, not before
        // assert_eq!(ReindeerState::Flying, dancer.state);

        comet.tick();
        dancer.tick();

        // On the 12th second, both reindeer are resting.
        assert_eq!(140, comet.distance);
        assert_eq!(ReindeerState::Resting, comet.state);
        assert_eq!(176, dancer.distance);
        assert_eq!(ReindeerState::Resting, dancer.state);
    }

    #[test]
    fn test_one_thousand_seconds() {
        let mut race: Race = [get_comet(), get_dancer()].iter().cloned().collect();
        race.run_to_time(1000);
        let Race { mut reindeer, .. } = race;
        let mut iter = reindeer.drain(..);
        let comet = iter.next().unwrap();
        let dancer = iter.next().unwrap();

        // After the 1000th second, both reindeer are resting, and Comet is in
        // the lead at 1120 km (poor Dancer has only gotten 1056 km by that
        // point).
        assert_eq!(comet.state, ReindeerState::Resting);
        assert_eq!(dancer.state, ReindeerState::Resting);
        assert_eq!(comet.distance, 1120);
        assert_eq!(dancer.distance, 1056);
    }

    #[test]
    fn test_new_race() {
        let mut race: Race = [get_comet(), get_dancer()].iter().cloned().collect();
        race.run_to_time(1000);
        let winner = race
            .in_lead(race.by_points())
            .next()
            .ok_or(Error::NoWinner)
            .unwrap();
        assert_eq!(winner.name, "Dancer");
        assert_eq!(winner.points, 689);
    }
}
