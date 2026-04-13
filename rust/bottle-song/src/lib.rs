pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let say_number = [
        "No", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
    ];
    fn say_bottle(n: u32) -> String {
        match n {
            1 => "bottle",
            _ => "bottles",
        }
        .to_owned()
    }

    (start_bottles - take_down + 1..=start_bottles)
        .rev()
        .map(|i| {
            format!(
                "{0} green {1} hanging on the wall,\n\
                {0} green {1} hanging on the wall,\n\
                And if one green bottle should accidentally fall,\n\
                There'll be {2} green {3} hanging on the wall.",
                say_number[i as usize],
                say_bottle(i),
                say_number[i as usize - 1].to_lowercase(),
                say_bottle(i - 1)
            )
        })
        .collect::<Vec<String>>()
        .join("\n\n")
}
