use std::cmp::{max, min};

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

impl BucketStats {
    fn new(moves: u8, goal_bucket: Bucket, other_bucket: u8) -> Self {
        BucketStats {
            moves,
            goal_bucket,
            other_bucket,
        }
    }
}

struct Pail {
    name: Bucket,
    size: u8,
    level: u8,
}

impl Pail {
    fn new(name: Bucket, size: u8) -> Self {
        Pail {
            name,
            size,
            level: 0,
        }
    }

    fn fill(&mut self) {
        self.level = self.size;
    }

    fn empty(&mut self) {
        self.level = 0;
    }

    fn full(&self) -> bool {
        self.level == self.size
    }

    fn is_empty(&self) -> bool {
        self.level == 0
    }

    fn room(&self) -> u8 {
        self.size - self.level
    }

    fn pour_into(&mut self, other: &mut Self) {
        let amount = min(self.level, other.room());
        self.level -= amount;
        other.level += amount;
    }
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    if goal > max(capacity_1, capacity_2) || !goal.is_multiple_of(gcd(capacity_1, capacity_2)) {
        return None;
    }

    let (mut start, mut other) = match start_bucket {
        Bucket::One => (
            Pail::new(Bucket::One, capacity_1),
            Pail::new(Bucket::Two, capacity_2),
        ),
        Bucket::Two => (
            Pail::new(Bucket::Two, capacity_2),
            Pail::new(Bucket::One, capacity_1),
        ),
    };

    if other.size == goal {
        // filling the start bucket + filling the other bucket
        return Some(BucketStats::new(2, other.name, start.size));
    }

    let mut moves: u8 = 0;
    while !&[start.level, other.level].contains(&goal) {
        if start.is_empty() {
            start.fill()
        } else if other.full() {
            other.empty();
        } else {
            start.pour_into(&mut other);
        }
        moves += 1;
    }

    match start.level == goal {
        true => Some(BucketStats::new(moves, start.name, other.level)),
        false => Some(BucketStats::new(moves, other.name, start.level)),
    }
}

fn gcd(a: u8, b: u8) -> u8 {
    if b == 0 { a } else { gcd(b, a % b) }
}
