// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.



fn main() {
    call_me(26);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}


//appelle la fonction call_me sans lui donner d'argument alors qu'il en attend un u32.
