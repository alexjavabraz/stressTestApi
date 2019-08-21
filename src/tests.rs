#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1(){
        let formatted_number = format!("{:.*}", 2, 1.234567);
        assert_eq!("1.23", formatted_number)
    }

        #[test]
    fn test2(){
        let formatted_number = format!("1,{}", 134567);
        assert_eq!("1,134567", formatted_number)
    }

}