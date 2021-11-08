use std::fmt;

pub struct CalcNumber<'game> {
    pub value: u32,
    pub left_element: u32,
    pub right_element: u32,
    pub operation: &'game str,
}

impl<'game> fmt::Debug for CalcNumber<'game> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}={}",
            self.left_element, self.operation, self.right_element, self.value
        )
    }
}

impl<'game> CalcNumber<'game> {
    pub fn generate_number_with_operation(
        left_element: u32,
        right_element: u32,
        operation: &str,
    ) -> Result<CalcNumber, String> {
        match operation {
            "+" => Ok(CalcNumber {
                value: left_element + right_element,
                left_element: left_element,
                right_element: right_element,
                operation: operation,
            }),
            "-" => {
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
