/// Calculate the total number of times the value appears in a certain value
pub fn count_all(numbers: &Vec<&str>) -> Vec<usize> { 
    let mut array: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    //array.shrink_to_fit();
    
    for num in numbers { 
        let num = if let Ok(v) = num.parse::<usize>() { v } else { continue; };
        if num >= 10 { 
            continue;
        }
        array[num] += 1;
    }

    array
}

/// Statistics the total number of times appears in a certain value
pub fn count_times(numbers: &Vec<&str>) -> Vec<usize> { 
    let mut array: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    //array.shrink_to_fit();
    
    for num in numbers { 
        let num = if let Ok(v) = num.parse::<usize>() { v } else { continue; };
        if num >= 10 { 
            continue;
        }
        array[num] += 1;
    }

    array
}
