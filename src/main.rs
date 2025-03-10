mod probl35;
mod probl125;
mod probl88;


fn main() {
    let nums = vec![1,2,3,4,5];
    let target = 3;
    println!("Problem #35 - Result: {}", probl35::solution(nums, target));

    let in_str = " ".to_string();
    println!("Problem #125 - Result: {}", probl125::solution(in_str));

    probl88::solution(&mut vec![1,2,3,0,0], 5, &mut vec![1,2], 3);

}
