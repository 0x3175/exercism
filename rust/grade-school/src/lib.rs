use std::collections::{BTreeMap, BTreeSet, HashSet};

#[derive(Default)]
pub struct School {
    students: BTreeMap<u32, BTreeSet<String>>,
    names: HashSet<String>,
}

impl School {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if self.names.insert(student.to_owned()) {
            self.students
                .entry(grade)
                .or_default()
                .insert(student.to_owned());
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        self.students.keys().copied().collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.students
            .get(&grade)
            .map(|students| students.iter().cloned().collect())
            .unwrap_or_default()
    }
}
