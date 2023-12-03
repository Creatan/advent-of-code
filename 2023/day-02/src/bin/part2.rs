fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);

    println!("{}", output);
}

#[derive(Debug)]
struct Game {
    _id: u32,
    red: Vec<u32>,
    green: Vec<u32>,
    blue: Vec<u32>
}

fn process(input: &str) -> u32 {
    return input.lines().map(line_to_game).map(|game|{
        let max_red = game.red.iter().max().expect("red should not be empty");
        let max_green = game.green.iter().max().expect("green should not be empty");
        let max_blue = game.blue.iter().max().expect("blue should not be empty");
        return max_red * max_green * max_blue;
    })
    // .inspect(|x| println!("{:?}",x))
    .sum();
    
}

fn line_to_game(line: &str) -> Game {
    let mut red: Vec<u32> = Vec::new();
    let mut green: Vec<u32> = Vec::new();
    let mut blue: Vec<u32> = Vec::new();
    let game_info = line.split(':').collect::<Vec<_>>();
    let game_num = game_info[0].replace("Game ","").replace(":","");
    for (_, set) in game_info[1].split(';').enumerate() {
       for(_, cube) in set.split(',').enumerate() {
                let c = cube.trim();
                if c.ends_with("red") {
                    let value: u32 = c.replace("red", "").trim().parse().expect("red should be valid number");
                    red.push(value);
                }
                if c.ends_with("green") {
                    let value: u32 = c.replace("green", "").trim().parse().expect("green should be valid number");
                    green.push(value);
                }

                if c.ends_with("blue") {
                    let value: u32 = c.replace("blue", "").trim().parse().expect("blue should be valid number");
                    blue.push(value);
                }
        } 
    }


    return Game { 
        _id: game_num.parse::<u32>().expect("Game should be valid number"), 
        red, 
        green, 
        blue 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       let result = process("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
); 
        assert_eq!(result, 2286)
    }
}
