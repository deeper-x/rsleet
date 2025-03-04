mod probl35;

use probl35::solution;

fn main() {
    let a = vec![1,2,3,4,5];
    let b = 3;
    
    println!("{}", solution(a, b));
}
