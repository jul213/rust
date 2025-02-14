#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn larger_smaller(){
        let larger = Rectangle {
            width: 8,
            height: 9,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };


        assert!(larger.can_hold(&smaller));
    }
}