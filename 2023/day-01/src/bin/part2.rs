use regex::Regex;
fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);

    println!("{}", output);
}


fn process(input: &str) -> usize {
    return input.lines().map(|line|{
        //convert one to nine as numbers
        let mut index = 0;
        let line_iter = std::iter::from_fn(move || {
            let w = &line[index..];
            let result = if w.starts_with("one"){
                Some('1')
            } else if w.starts_with("two"){
                Some('2')
            } else if w.starts_with("three"){
                Some('3')
            } else if w.starts_with("four"){
                Some('4')
            } else if w.starts_with("five"){
                Some('5')
            } else if w.starts_with("six"){
                Some('6')
            } else if w.starts_with("seven"){
                Some('7')
            } else if w.starts_with("eight"){
                Some('8')
            } else if w.starts_with("nine"){
                Some('9')
            } else {
                w.chars().next()
            };
            index += 1;
            result
        });
        return line_iter.collect::<String>()
    }).map(|line|{
        
        let re = Regex::new(r"[^0-9]").unwrap();
        let decimals = re.replace_all(&line, "");

        let mut chars = decimals.chars();
        let first = chars.nth(0).expect("should be a number");
        let last = chars.nth_back(0);

        if let None = last {
            return format!("{first}{first}").parse::<usize>().unwrap();
        }
        let last_value = last.unwrap();
        return format!("{first}{last_value}").parse::<usize>().unwrap();
    }).sum();
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       let result = process("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
); 
        assert_eq!(result, 281)
    }
}
