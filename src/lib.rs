pub fn square_of_sum(n: u32) -> u32 {
    let sum_of_n: u32 = (1..=n).sum();
    let final_result = sum_of_n * sum_of_n;
    final_result
}

pub fn sum_of_squares(n: u32) -> u32 {
    let numbers = (1..=n);
    let final_result_2: u32 = numbers.map(|x| x * x).sum();
    final_result_2
}

pub fn difference(n: u32) -> u32 {
    let square_of_sum = square_of_sum(n);
    let sum_of_squares = sum_of_squares(n);
    square_of_sum - sum_of_squares

}
