const STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

fn plant(c: char) -> &'static str {
    match c {
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        'V' => "violets",
        _ => unreachable!("invalid plant code"),
    }
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let index = STUDENTS
        .iter()
        .position(|&s| s == student)
        .expect("unknown student")
        * 2;

    diagram
        .lines()
        .flat_map(|line| line.chars().skip(index).take(2).map(plant))
        .collect()
}
