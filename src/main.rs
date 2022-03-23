fn sum_with_missing(numbers: Vec<Option<i32>>) -> i32 {
    numbers.iter().flatten().fold(0, |acc, val| acc + val)
}

fn main() {
    let numbers = vec![Some(1), None, Some(5), None, Some(4)];
    print!("The sum of {:?}", numbers);
    let sum = sum_with_missing(numbers);
    println!(" is {}.", sum);
}


#[test]
fn empty() {
    let nn = vec![];
    assert_eq!(sum_with_missing(nn), 0);
}

#[test]
fn no_missing() {
    let nn = vec![Some(1), Some(5), Some(4)];
    assert_eq!(sum_with_missing(nn), 10);
}

#[test]
fn some_missing() {
    let nn = vec![None, Some(1), Some(5), Some(4), None, None];
    assert_eq!(sum_with_missing(nn), 10);
}

#[test]
fn all_missing() {
    let nn = vec![None, None, None];
    assert_eq!(sum_with_missing(nn), 0);
}
