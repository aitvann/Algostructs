use rand::{distributions::Standard, prelude::*};

#[test]
fn random_data() {
    let rng = rand::thread_rng();
    let data: Vec<i64> = rng.sample_iter(Standard).take(1024).collect();

    let mut to_test = data.clone();
    merge_sort::sort(&mut to_test);

    let mut tested = data.clone();
    tested.sort();

    assert_eq!(to_test, tested);
}

#[test]
fn empty_data() {
    let empty_vec = Vec::<i64>::new();

    let mut to_test = empty_vec.clone();
    merge_sort::sort(&mut to_test);

    let mut tested = empty_vec.clone();
    tested.sort();

    assert_eq!(to_test, tested);
}
