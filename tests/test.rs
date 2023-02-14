use rust_command_line_calculator as rclc;

#[test]
fn test_tests_are_loaded() {
    assert_eq!("AA", "AA");
}

#[test]
fn test_main_sum_simple() {
    let my_expression: rclc::Expression = 
        rclc::Expression::new(
            String::from("40 + 33"),
            String::from("40 + 33"),
            rclc::Task::None,
            0);
    assert_eq!(my_expression.process().unwrap(), 73.0);
}

#[test]
fn test_main_sum_chain() {
    let my_expression: rclc::Expression = 
        rclc::Expression::new(
            String::from("20340 + 32424 + 24 + 23"),
            String::from("20340 + 32424 + 24 + 23"),
            rclc::Task::None,
            0);
    assert_eq!(my_expression.process().unwrap(), 20340.0 + 32424.0 + 24.0 + 23.0);
}

#[test]
fn test_main_difference_simple() {
    let my_expression: rclc::Expression = 
        rclc::Expression::new(
            String::from("33-13"),
            String::from("33-13"),
            rclc::Task::None,
            0);
    assert_eq!(my_expression.process().unwrap(), 33.0 - 13.0);
}

#[test]
fn test_main_difference_chain() {
    let my_expression: rclc::Expression = 
        rclc::Expression::new(
            String::from("353535 - 2405 - 33 - 13 - 4"),
            String::from("353535 - 2405 - 33 - 13 - 4"),
            rclc::Task::None,
            0);
    assert_eq!(my_expression.process().unwrap(), 353535.0 - 2405.0 - 33.0 - 13.0 - 4.0);
}

#[test]
fn test_main_product_simple() {
    let my_expression: rclc::Expression = 
        rclc::Expression::new(
            String::from("353* 13"),
            String::from("353* 13"),
            rclc::Task::None,
            0);
    assert_eq!(my_expression.process().unwrap(), 353.0 * 13.0);
}

#[test]
fn test_main_procuct_chain() {
    let my_expression: rclc::Expression = 
        rclc::Expression::new(
            String::from("353535 * 2405 * 33 * 13 * 4"),
            String::from("353535 * 2405 * 33 * 13 * 4"),
            rclc::Task::None,
            0);
    assert_eq!(my_expression.process().unwrap(), 353535.0 * 2405.0 * 33.0 * 13.0 * 4.0);
}

#[test]
fn test_main_quotient_simple() {
    let my_expression: rclc::Expression = 
        rclc::Expression::new(
            String::from("353 / 13"),
            String::from("353 / 13"),
            rclc::Task::None,
            0);
    assert_eq!(my_expression.process().unwrap(), 353.0 / 13.0);
}

#[test]
fn test_main_quotient_chain() {
    let my_expression: rclc::Expression = 
        rclc::Expression::new(
            String::from("353535 / 2405 / 33 / 13 / 4"),
            String::from("353535 / 2405 / 33 / 13 / 4"),
            rclc::Task::None,
            0);
    assert_eq!(my_expression.process().unwrap(), 353535.0 / 2405.0 / 33.0 / 13.0 / 4.0);
}
