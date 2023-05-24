#[allow(dead_code)]
struct Process {
    active: bool,
    id: isize
}

fn main() {
    let ap = Process {active: true, id: 100};
    println!("My id: {:?}", ap.id);
} 

// demo topics
// =============================
// [x] println macro
// [ ] trait Debug / Display
// [ ] move variable -> shallow copy
// [ ] trait Clone -> deep copy
// [ ] trait Copy
// [ ] implement lifetimes
