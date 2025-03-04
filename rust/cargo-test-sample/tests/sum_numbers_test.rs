use math_stuffs::sum_numbers;

#[test]
fn test_sum_numbers() -> Result<(), String> {
    let result = sum_numbers(3, 5);
    if result != 7 {
        Err("Answer is not 7".to_string())
    } else {
        Ok(())
    }
}