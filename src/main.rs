mod probl121;
mod probl125;
mod probl20;
mod probl205;
mod probl21;
mod probl217;
mod probl35;
mod probl409;
mod probl58;
mod probl88;

mod another_mod {
    pub mod something;
}

use another_mod::something::hello;

fn main() {
    hello();

    let nums = vec![1, 2, 3, 4, 5];
    let target = 3;
    println!("Problem #35 - Result: {}", probl35::solution(nums, target));

    let in_str_125 = " ".to_string();
    println!("Problem #125 - Result: {}", probl125::solution(in_str_125));

    let in_str_409 = "abccccdd".to_string();
    println!("Problem #409 - Result: {}", probl409::solution(in_str_409));

    let in_str_20 = "{()[]}".to_string();
    println!("Problem #20 - Result: {}", probl20::solution(in_str_20));

    let in_str_58 = "Hello World".to_string();
    println!("Problem #58 - Result: {}", probl58::solution(in_str_58));

    let in_prices = vec![2, 5, 4, 7, 9, 10];
    println!("Problem #121 - Result: {}", probl121::solution(in_prices));

    let mut in_nums1_88 = vec![1, 2, 3, 0, 0, 0];
    let mut in_nums2_88 = vec![2, 5, 6];

    probl88::solution(&mut in_nums1_88, 3, &mut in_nums2_88, 3);
    println!("Problem #88 - Result: inline");

    let in_list_217 = vec![1, 6, 5, 4, 3, 4, 3, 2, 5, 4];
    println!("Problem #217 - Result: {}", probl217::solution(in_list_217));

    let in_s_205 = "foo".to_string();
    let in_t_205 = "bar".to_string();

    println!(
        "Problem #205 - Result {}",
        probl205::solution(in_s_205, in_t_205)
    );

    probl21::solution();
}
