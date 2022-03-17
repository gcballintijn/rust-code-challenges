use std::cmp::Ordering;

#[derive(Clone, Copy)]
struct MyF32(f32);

impl Ord for MyF32 {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 < other.0 {
            Ordering::Less
        } else if self.0 > other.0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl Eq for MyF32 {}

impl PartialOrd for MyF32 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl PartialEq for MyF32 {
    fn eq(&self, other: &MyF32) -> bool {
        self.0 == other.0
    }
}

fn median(a: Vec<f32>) -> Option<f32> {
    if a.is_empty() {
        return None;
    }

    let mut a: Vec<MyF32> = a.iter().map(|f| MyF32(*f)).collect();
    a.sort();

    let middle = a.len() / 2;
    if a.len() % 2 == 0 {
        let first = a[middle - 1];
        let second = a[middle];
        Some((first.0 + second.0) / 2.0)
    } else {
        Some(a[middle].0)
    }
}

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
