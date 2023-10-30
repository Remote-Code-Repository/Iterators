use std::io;

fn main() {
    let mut nums: Vec<i32> = Vec::new();
    nums.push(33);
    nums.push(36);
    nums.push(130);
    nums.push(290);

    let mut target = String::new();

    println!("Please provide a target >");

    io::stdin().read_line(&mut target).unwrap();

    match target.trim().parse::<i32>() {
        Ok(target) => find_target(nums, target),
        Err(e) => panic!("Failed to find match: {}\n", e),
    };
}

fn find_target(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    for i in 0..nums.iter().len() {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                let start: i32 = i.to_string().parse::<i32>().unwrap();
                let end: i32 = j.to_string().parse::<i32>().unwrap();
                output.push(start);
                output.push(end);
                break;
            }
        }
    }
    print!("Output: {:?}", output);
    output
}
