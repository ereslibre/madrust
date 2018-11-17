#[derive(PartialEq)]
enum ComparisonResult {
  Smaller,
  Equal,
  Bigger,
}

fn compare(a: u32, b: u32) -> ComparisonResult {
  if a < b {
    ComparisonResult::Smaller
  } else if a == b {
    ComparisonResult::Equal
  } else {
    ComparisonResult::Bigger
  }
}

fn main() {
    if compare(2, 3) == ComparisonResult::Smaller {
        println!("Science works!");
    } else {
        println!("Science needs a review!");
    }
}
