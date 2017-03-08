fn main() {
    let nums = vec![4, 8, 15, 16, 23, 42];

    match find_max(&nums) {
        Ok(max) => println!("max: {}", max),
        Err(err) => println!("Error: {}", err),
    }

    match find_max(&Vec::<i32>::new()) {
        Ok(max) => println!("max: {}", max),
        Err(err) => println!("Error: {}", err),
    }

    match first_even(&nums) {
        Some(first) => println!("first even number in set: {}", first),
        None => println!("All numbers in the set are odd."),
    }

    let odd_nums = vec![1, 3, 5, 7, 9, 11];
    match first_even(&odd_nums) {
        Some(first) => println!("first even number in set: {}", first),
        None => println!("All numbers in the set are odd."),
    }
}

fn find_max(nums: &Vec<i32>) -> Result<i32, &str> {
    if nums.len() == 0 {
        return Err("Vector is empty!");
    }

    let mut max = std::i32::MIN;

    for num in nums {
        if *num > max {
            max = *num
        }
    }

    Ok(max)
}

fn first_even(nums: &Vec<i32>) -> Option<i32> {
    for num in nums {
        if *num % 2 == 0 {
            return Some(*num);
        }
    }

    None
}