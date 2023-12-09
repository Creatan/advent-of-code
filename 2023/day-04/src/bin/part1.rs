fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);

    println!("{}", output);
}

fn process(input: &str) -> u32 {
    return input.lines().map(get_score).sum();
    
}

fn get_score(line: &str) -> u32 {
    let card_data: Vec<_> = line.split(':')
        .nth(1).expect("Card should contain data").trim().split(" | ").collect();
    let winning_numbers: Vec<_> = card_data[0].split(' ').filter(|n| !n.is_empty()).collect();
    return card_data
        .get(1)
        .expect("Card data should have numbers").split(' ')
        .filter(|n| {
            return !n.is_empty() && winning_numbers.contains(n)
        })
        .fold(0, |acc,_| {
            // there is propably better way to do this but at least this works
            if acc == 0 {
                return 1
            }else {
                return acc * 2 
            }

        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       let result = process("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"); 
        assert_eq!(result, 13)
    }
}
