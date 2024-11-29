extern crate rsnum2persian;

fn main() {
    let mut level = 0;
    println!("{}", rsnum2persian::num_to_persian(5678, &mut level, false));
    println!("{}", rsnum2persian::num_to_persian(5678, &mut level, true));
}
