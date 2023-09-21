use std::collections::HashMap;

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School {
    students_in_grade: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        Self {
            students_in_grade: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        match self.students_in_grade.get_mut(&grade) {
            Some(grade) => grade.push(student.to_string()),
            None => {
                self.students_in_grade
                    .insert(grade, vec![student.to_string()]);
            }
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades: Vec<u32> = self.students_in_grade.keys().copied().collect();

        grades.sort_unstable();
        grades
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.students_in_grade.get(&grade) {
            Some(grade) => {
                let mut students = grade.clone();
                students.sort_unstable();
                students
            }
            None => vec![],
        }
    }
}
