use std::fmt;

/// save the result of a calculation, the operation and the values on the left hand side
pub struct CalcNumber<'game> {
    pub value: u32,
    pub left_element: u32,
    pub right_element: u32,
    pub operation: &'game str,
}

/// implement a debug formatter for the struct
impl<'game> fmt::Debug for CalcNumber<'game> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}={}",
            self.left_element, self.operation, self.right_element, self.value
        )
    }
}

/// implement a simple matching logic and constraints for the operations
impl<'game> CalcNumber<'game> {
    pub fn generate_number_with_operation(
        left_element: u32,
        right_element: u32,
        operation: &str,
    ) -> Result<CalcNumber, String> {
        match operation {
            "+" => {
                // as we use 32 bit unsigned integers check for overflow
                let new_value = left_element.checked_add(right_element);
                if new_value == None {
                    Err(String::from("Overflow on addition"))
                } else {
                    Ok(CalcNumber {
                        value: left_element + right_element,
                        left_element: left_element,
                        right_element: right_element,
                        operation: operation,
                    })
                }
            }
            "-" => {
                // negativ numbers are not allowed
                if right_element > left_element {
                    Err(String::from("Negative numbers are not allowed"))
                } else {
                    Ok(CalcNumber {
                        value: left_element - right_element,
                        left_element: left_element,
                        right_element: right_element,
                        operation: operation,
                    })
                }
            }
            "*" => {
                // multiplication is prone to overflow
                let new_value = left_element.checked_mul(right_element);
                if new_value == None {
                    Err(String::from("Overflow"))
                } else if (left_element == 0) || (right_element == 0) {
                    Err(String::from("Faktor 0 not allowed."))
                } else {
                    Ok(CalcNumber {
                        value: new_value.unwrap(),
                        left_element: left_element,
                        right_element: right_element,
                        operation: operation,
                    })
                }
            }
            "/" => {
                if right_element == 0 {
                    Err(String::from("Division by 0 not allowed"))
                } else if (left_element % right_element) != 0 {
                    // the rules state that float are not allowed
                    // u32 does flooring on division, which is not something we want
                    Err(String::from("Division has a remainder < 1."))
                } else {
                    Ok(CalcNumber {
                        value: left_element / right_element,
                        left_element: left_element,
                        right_element: right_element,
                        operation: operation,
                    })
                }
            }
            _ => Err(String::from("Not possible")),
        }
    }
}
