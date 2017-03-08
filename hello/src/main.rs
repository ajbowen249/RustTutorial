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
