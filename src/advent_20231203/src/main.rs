
fn main() {
    let input = include_str!("input.txt");

    let lines_vec: Vec<&str> = input.lines().collect();
    let total_sum = 0;

    for (index,line) in lines_vec.iter().enumerate() {
        let current_char_number = "";
        for (char_index, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                while line.chars().collect::<Vec<char>>()[char_index +1].is_digit(10) {

                }
            }
        }
    }

}
