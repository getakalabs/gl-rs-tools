use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn get_rand(limit: i32) -> i32 {
    let mut rng = thread_rng();
    let mut nums: Vec<i32> = (1..=9).collect();
    nums.shuffle(&mut rng);

    let numbers = crate::numbers::iter_numbers(nums, limit, &mut String::default());
    let result = numbers.parse::<i32>();
    if result.is_err() {
        return 0;
    }

    result.unwrap()
}