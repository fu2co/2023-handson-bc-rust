use std::fmt::{Debug, Display};

#[allow(dead_code)]
#[derive(Debug,Clone)]
struct Process {
    active: bool,
    id: isize
}

impl Display for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.active {
            false => write!(f, "Process not active"),
            true => write!(f, "id: {}", self.id),
        }
    }
}

fn main() {
    let ap = Process {active: true, id: 100};
    let aq = ap.clone();
    println!("My process: {:?}, {:?}", ap, aq);
    println!("My process: {}, {}", ap, aq);
    println!("pointers: {:p} {:p}", &ap, &aq);
} 

// demo topics
// =============================
// [x] println macro
// [x] trait Debug / Display
// [x] move variable -> shallow copy
// [x] trait Clone -> deep copy
// [ ] trait Copy
// [ ] implement lifetimes
