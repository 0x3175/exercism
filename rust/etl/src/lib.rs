use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut m: BTreeMap<char, i32> = BTreeMap::new();
    for (&i, v) in h.iter() {
        for &c in v {
            m.insert(c.to_ascii_lowercase(), i);
        }
    }
    m
}
