#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let sublist = first_list.is_empty()
        || second_list
            .windows(first_list.len())
            .any(|window| window == first_list);
    let superlist = second_list.is_empty()
        || first_list
            .windows(second_list.len())
            .any(|window| window == second_list);

    match (sublist, superlist) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}
