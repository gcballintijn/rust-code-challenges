use std::cmp::Ordering;

fn sort_usernames<T: AsRef<str>>(usernames: &mut Vec<T>) {
    usernames.sort_by(|n1, n2| {
        let n1 = n1.as_ref().to_lowercase();
        let n2 = n2.as_ref().to_lowercase();
        if n1 < n2 {
            Ordering::Less
        } else if n1 > n2 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
}

fn main() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];

    println!("unsorted: {:?}", &users);
    sort_usernames(&mut users);
    println!("sorted:   {:?}", &users);
}

#[test]
fn five_users() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];
    let sorted = vec!["alison", "Amy", "Jennifer", "mike99", "Todd"];
    sort_usernames(&mut users);

    assert_eq!(users, sorted);
}
