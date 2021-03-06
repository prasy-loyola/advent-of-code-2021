use std::io;

mod inputs;
fn read_input_from_file() -> Result<String, io::Error> {
    inputs::get_input("day1")
}

fn sum_of_n_terms(nums: &Vec<u32>, start_idx: usize, no_of_terms: usize) -> u32 {
    assert_eq!(true, start_idx + no_of_terms <= nums.len());

    let mut sum = 0;
    for i in start_idx..(start_idx + no_of_terms) {
        sum += nums[i];
    }
    return sum;
}

fn main() {
    let depths: Vec<u32> = read_input_from_file()
        .expect("Input file should be present")
        .lines()
        .map(|l| {
            l.parse()
                .expect(&format!("Depths need to be a number but found: {}", l))
        })
        .collect();

    let mut prev_depth = 0;
    let mut increase_count = 0;
    for i in 0..(depths.len() - 2) {
        let curr_depth = sum_of_n_terms(&depths, i, 3);
        // println!("Sum of 3 terms starting from {}: {}", i, curr_depth) ;
        if prev_depth != 0 && curr_depth > prev_depth {
            increase_count += 1;
        }

        prev_depth = curr_depth;
    }
    println!(
        "No. of times the depth has increased is: {:?}",
        increase_count
    );
}
