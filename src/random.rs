use rand;

/// all letters
const ALL_LETTERS: &str = "abcdefghijklmnopqrstuvwxyz0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ALL_NUMBERS: &str = "0123456789";

/// random string
pub fn rand_str(length: usize) -> String { 
    let mut target = String::from("");
    let chars :Vec<char> = ALL_LETTERS.chars().collect();
    for _ in 0..length { 
        let rand_num = rand::random::<usize>();
        target.push(chars[rand_num % 62]);
    }   
    target
}

/// Get random numberndom number
#[inline]
pub fn rand_number(max: usize) -> usize { 
    rand::random::<usize>() % max
}

/// Random acquisition
pub fn rand_members(max: usize, arr: &[&str]) -> String { 
    let mut target = String::from("");
    let num = rand_number(max);
    for i in 0..num { 
        if i > 0 { 
            target.push(',');
        }
        target.push_str(arr[i]);
    }
    target
}

/// Get a random string
pub fn rand_numbers(length: usize) -> String { 
    let mut target = String::from("");
    let chars :Vec<char> = ALL_NUMBERS.chars().collect();
    for i in 0..length { 
        if i > 0 { 
            target.push(',');
        }
        let rand_num = rand::random::<usize>();
        target.push(chars[rand_num % 10]);
    }   
    target
}

/// Get the strings that goes heavy
pub fn rand_dedup_numbers(length: usize) -> String { 
    let mut target = String::from("");
    let mut chars :Vec<char> = ALL_NUMBERS.chars().collect();
    for i in 0..length { 
        if i > 0 { 
            target.push(',');
        }
        let rand_num = rand::random::<usize>();
        let chars_len = chars.len();
        target.push(chars.remove(rand_num % chars_len));
    }
    target
}

/// Get the heavy string, and does not include the specified character
pub fn rand_dedup_numbers_exp(length: usize, nums: &Vec<&str>) -> String { 
    let mut target = String::from("");
    let mut chars :Vec<char> = ALL_NUMBERS.chars().collect();
    let mut i = 0;
    loop {
        let rand_num = rand::random::<usize>();
        let chars_len = chars.len();
        let ch = &format!("{}", chars.remove(rand_num % chars_len));
        if !nums.contains(&ch.as_str()) { 
            if i > 0 { 
                target.push(',');
            }
            target.push_str(ch);
            i += 1;
        }
        if i >= length { 
            break;
        }
    }
    target
}
