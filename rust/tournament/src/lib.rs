use std::cmp::Reverse;
use std::collections::HashMap;

#[derive(Default)]
struct Stats {
    mp: u32,
    w: u32,
    d: u32,
    l: u32,
    p: u32,
}

impl Stats {
    fn win(&mut self) {
        self.mp += 1;
        self.w += 1;
        self.p += 3;
    }
    fn lose(&mut self) {
        self.mp += 1;
        self.l += 1;
    }
    fn draw(&mut self) {
        self.mp += 1;
        self.d += 1;
        self.p += 1;
    }
    fn row(&self, name: &str) -> String {
        format!(
            "{name:<30} |{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
            self.mp, self.w, self.d, self.l, self.p
        )
    }
}

pub fn tally(match_results: &str) -> String {
    let mut map: HashMap<&str, Stats> = HashMap::new();
    for line in match_results.lines() {
        let mut it = line.split(';');
        let (Some(home), Some(away), Some(res)) = (it.next(), it.next(), it.next()) else {
            continue;
        };
        match res {
            "win" => {
                map.entry(home).or_default().win();
                map.entry(away).or_default().lose();
            }
            "loss" => {
                map.entry(home).or_default().lose();
                map.entry(away).or_default().win();
            }
            "draw" => {
                map.entry(home).or_default().draw();
                map.entry(away).or_default().draw();
            }
            _ => {}
        }
    }

    let mut teams: Vec<(&str, &Stats)> = map.iter().map(|(&name, stats)| (name, stats)).collect();
    teams.sort_by_key(|(name, s)| (Reverse(s.p), *name));

    let header = format!(
        "{:<30} |{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
        "Team", "MP", "W", "D", "L", "P"
    );

    let mut out = String::with_capacity(header.len() * (1 + teams.len()) + teams.len());
    out.push_str(&header);
    for team in teams {
        out.push('\n');
        out.push_str(&team.1.row(team.0));
    }
    out
}
