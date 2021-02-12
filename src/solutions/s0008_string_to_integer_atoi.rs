use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn my_atoi(string: String) -> i32 {
        let (mut result, mut sign, mut state) = (0i64, 1, 1);

        for input in string.chars() {
            match state {
                1 => match input {
                    ' ' => (),
                    '+' | '-' => {
                        state = 2;
                        sign = (input == '+') as i32 - (input == '-') as i32
                    }
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        state = 3;
                        result += input as i64 - '0' as i64;
                    }
                    _ => {
                        return 0;
                    }
                },
                2 => match input {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        state = 3;
                        result += input as i64 - '0' as i64;
                    }
                    _ => {
                        return 0;
                    }
                },
                3 => match input {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        result = result * 10 + input as i64 - '0' as i64;
                        let diff = result - i32::MAX as i64;
                        if sign == 1 && diff > 0 {
                            return i32::MAX;
                        } else if sign == -1 && diff > 1 {
                            return i32::MIN;
                        }
                    }
                    _ => {
                        break;
                    }
                },
                _ => (),
            }
        }

        return sign * result as i32;
    }
}
