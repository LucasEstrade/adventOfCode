fn main() {
    let input = include_str!("input.txt");

    let result: u32 = input.lines().fold(0, |acc: u32, line: &str| {
        let line_with_replaced_chars = replace_chars_with_nums(line);
        let first = line_with_replaced_chars.chars()
            .find(|c: &char| c.is_digit(10))
            .unwrap_or('0')
            .to_digit(10)
            .unwrap();

        let last = line_with_replaced_chars.chars()
            .rev()
            .find(|c: &char| c.is_digit(10))
            .unwrap_or('0')
            .to_digit(10)
            .unwrap();

        acc + first * 10 + last
    });

    /*
    let result = input.lines().filter_map(|line| {
        let first = line.chars().filter_map(|c: char| c.to_digit(10)).nth(1).unwrap_or_default();
        let first22 = line.chars().find(|c: &char| c.is_digit(10)).unwrap_or('0');
        let last = line.chars().rfind(|c: &char| c.is_digit(10)).unwrap_or('0');

        None
    });
     */

    // Print the total sum
    println!("Total sum of first and last numerical characters: {}", result);
}

// Good answer : 54951
// Good answer part 2 : 55218

fn replace_chars_with_nums (line : &str) -> String{
    line.replace("zero", "0o")
        .replace("one", "o1e")
        .replace("two", "t2")
        .replace("three", "t3e")
        .replace("four", "4")
        .replace("five", "5e")
        .replace("six", "6")
        .replace("seven", "7n")
        .replace("eight", "e8")
        .replace("nine", "9")

}
