fn print_binary(num: i32) {
    for i in 0..32 {
        print!("{}", if num & (1 << (31 - i)) == 0 { 0 } else { 1 });
    }
    println!("");
}

fn return_true() -> bool {
    println!("enter return_true");
    true
}

fn return_false() -> bool {
    println!("enter return_false");
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_binary() {
        print_binary(10);
        print_binary(10 << 3);
        print_binary(10 >> 3);

        let mut test = return_true() | return_false();
        println!("test: {}", test);

        test = return_true() || return_false();
        println!("test: {}", test);

        test = return_false() & return_true();
        println!("test: {}", test);

        test = return_false() && return_true();
        println!("test: {}", test);
    }
}
