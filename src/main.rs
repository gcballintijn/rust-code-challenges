// use std::collections::BTreeSet;
// fn unique(a: Vec<i32>) -> Vec<i32> {
//     let value_set = a.into_iter().collect::<BTreeSet<i32>>();
//     value_set.into_iter().collect()
// }

// advanced 1: use generic types
// use std::collections::BTreeSet;
// fn unique<T>(a: Vec<T>) -> Vec<T>
//     where T: Ord
// {
//     let value_set = a.into_iter().collect::<BTreeSet<T>>();
//     value_set.into_iter().collect::<Vec<T>>()
// }

// advanced 2: keep items in order
// fn unique(a: Iterable<T>) -> Vec<T> {
//     todo!();
// }
// use std::collections::BTreeSet;
// fn unique<S, T>(a: S) -> Vec<T>
//     where S: IntoIterator<Item = T>,
//           T: Ord + Copy,
// {
//     let mut duplicates: BTreeSet<T> = BTreeSet::new();
//     a.into_iter().filter(|x| if !duplicates.contains(x) {
//         duplicates.insert(*x);
//         true
//     } else {
//         false
//     }).collect::<Vec<T>>()
// }

// advanced 3: use iterators
// fn unique(a: Iterable<T>) -> Vec<T> {
//     todo!();
// }
use std::collections::BTreeSet;
fn unique<S, T>(a: S) -> Vec<T>
    where S: IntoIterator<Item = T>,
          T: Ord,
{
    let value_set = a.into_iter().collect::<BTreeSet<T>>();
    value_set.into_iter().collect::<Vec<T>>()
}

fn main() {
    let input: Vec<i32> = vec![2, 1, 1];
    let answer: Vec<i32> = unique(input);
    println!("unique items -> {:?}", answer);
}

#[test]
fn empty_list() {
    let input: Vec<i32> = vec![];
    let expected_output = vec![];
    // let actual_output = unique(input);
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1, 4, 5];
    let expected_output = vec![1, 4, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1, 5, 2];
    let expected_output = vec![1, 2, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}


#[test]
fn unsorted_list_with_duplicates() {
    let input = vec![1, 5, 2, 2, 1];
    let expected_output = vec![1, 2, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list_with_duplicates() {
    let mut input = vec![1, 5, 2, 2, 1];
    input.sort_by(|x,y| x.partial_cmp(y).unwrap());
    let expected_output = vec![1, 2, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}
