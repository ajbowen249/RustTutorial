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

    let tuple = (1, 'a', "tuple", 15.35);
    println!("tuple 0: {}", tuple.0);
    println!("tuple 1: {}", tuple.1);
    println!("tuple 2: {}", tuple.2);
    println!("tuple 3: {}", tuple.3);

    let (int, character, float) = (12, 'b', 1.23);
    println!("int: {}", int);
    println!("character: {}", character);
    println!("float: {}", float);

    let (value, index) = first_even_with_index(&nums);
    println!("first even: {} at index {}", value, index);

    match nums.iter().find(|x| *x % 2 == 0) {
        Some(first) => println!("first even number in set: {}", first),
        None => println!("All numbers in the set are odd."),
    }

    match find_nth_even(&nums, 3) {
        Some(val) => println!("4th even number in set: {}", val),
        None => println!("No 4th even"),
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

fn first_even_with_index(nums: &Vec<i32>) -> (i32, usize) {
    for i in 0..nums.len() {
        if nums[i] % 2 == 0 {
            return (nums[i], i);
        }
    }

    (0, 0)
}

fn find_nth_even(nums: &Vec<i32>, n: i32) -> Option<&i32> {
    let mut num_found = -1;
    let found = nums.iter().find(|x| {
        if **x % 2 == 0 {
            num_found = num_found + 1;
        }

        num_found == n
    });

    println!("num_found: {} n: {}", num_found, n);

    found
}