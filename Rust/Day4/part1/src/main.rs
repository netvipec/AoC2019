fn get_digits(n : i32) -> Vec<u8> {
    let mut digits : Vec<u8> = vec![0; 6];

    digits[0] = ( n / 100000) as u8;
    digits[1] = ((n /  10000) % 10) as u8;
    digits[2] = ((n /   1000) % 10) as u8;
    digits[3] = ((n /   100)  % 10) as u8;
    digits[4] = ((n /    10)  % 10) as u8;
    digits[5] = ( n           % 10) as u8;

    return digits;
}

fn valid_password(n : i32) -> bool {
    let digits = get_digits(n);

    let mut valid = false;
    for i in 0..digits.len()-1 {
        if digits[i] == digits[i+1] {
            valid = true;
        }
        if digits[i] > digits[i+1] {
            valid = false;
            break;
        }
    }

    return valid;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_password_works() {
        assert!( valid_password(122345));
        assert!( valid_password(111123));
        assert!(!valid_password(135679));
                
        assert!( valid_password(111111));
        assert!(!valid_password(223450));
        assert!(!valid_password(123789));

        assert!( valid_password(112233));
        assert!( valid_password(123444));
        assert!( valid_password(111122));
    }
}

fn main() {
    let lower_bound  = 284639;
    let higher_bound = 748759;

    let mut counter = 0;
    for i in (lower_bound + 1)..higher_bound {
        if valid_password(i) {
            counter += 1;
        }
    }

    println!("Solution: {}", counter);
}
