fn main() {
    println!("Hello, world!");
}

pub(crate) fn check_panic(val: i32) -> i32 {
    //not implemented yet
    panic!("not implemented yet");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_check_panic() {
        assert_eq!(check_panic(1), 1);
    }
}
