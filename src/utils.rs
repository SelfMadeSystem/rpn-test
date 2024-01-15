/// Adds whitespace between numbers and operators or parentheses
pub fn normalize_whitespace(s: &str) -> String {
    let mut result = String::new();
    let mut last_was_number = false;
    let mut last_was_operator = false;
    let mut last_was_parenthesis = false;
    for c in s.chars() {
        if c.is_whitespace() {
            result.push(' ');
            continue;
        }
        if c.is_digit(10) || c == '.' {
            if last_was_operator || last_was_parenthesis {
                result.push(' ');
            }
            result.push(c);
            last_was_number = true;
            last_was_operator = false;
            last_was_parenthesis = false;
        } else if c == '(' || c == ')' {
            if last_was_number | last_was_operator | last_was_parenthesis {
                result.push(' ');
            }
            result.push(c);
            last_was_number = false;
            last_was_operator = false;
            last_was_parenthesis = true;
        } else {
            if last_was_number || last_was_parenthesis {
                result.push(' ');
            }
            result.push(c);
            last_was_number = false;
            last_was_operator = true;
            last_was_parenthesis = false;
        }
    }
    result
}

// logs and returns a singular value
#[macro_export]
macro_rules! log {
    ($e:expr) => {{
        let e = $e;
        println!("{} = {:?}", stringify!($e), e);
        e
    }};
}
