
fn fact(n: u32) -> u32 {
    match n {
        0 => 1,
        _ => n * fact(n - 1)
    }
}

#[cfg(test)]
mod tests {
    use fact;

    #[test]
    fn test1() {
        assert_eq!(fact(0), 1)
    }

    #[test]
    fn test2() {
        assert_eq!(fact(3), 5)
    }
}