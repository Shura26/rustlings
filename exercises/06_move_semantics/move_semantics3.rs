// move_semantics3.rs
//
// Make me compile without adding new lines -- just changing existing lines! (no
// lines with multiple semicolons necessary!)
//
// Execute `rustlings hint move_semantics3` or use the `hint` watch subcommand
// for a hint.



#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

<<<<<<< HEAD:exercises/06_move_semantics/move_semantics3.rs
    let vec1 = fill_vec(vec0);
=======
    let mut vec1 = fill_vec(vec0.clone());
>>>>>>> recov-branch:exercises/move_semantics/move_semantics3.rs

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);

    vec
}
