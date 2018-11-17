macro_rules! one {
    () => { println!("one!"); }
}

macro_rules! two {
    () => {
        one!();
        println!("two!");
    }
}

#[test]
fn some_test() { }

#[cfg(target_os = "windows")]
fn something_windows() { }

#[cfg(target_os = "linux")]
fn something_linux() { }

fn main() {
    two!();
}
