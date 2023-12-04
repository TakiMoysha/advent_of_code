use advent_of_code_2023::read_csv;

pub struct GameDescription {
    id: usize,
    red: u8,
    blue: u8,
    green: u8,
}

impl GameDescription {
    fn from_string(input: &str) -> Result<Self, &str> {
        let game: Vec<&str> = input.split(":").collect();

        let game_header: Vec<&str> = game.get(0).unwrap().rsplit(" ").collect();
        let game_id = game_header.get(0).unwrap().parse::<u8>().unwrap();

        let game_rounds: Vec<&str> = game.get(1).unwrap().split(";").map(|s| s.trim()).collect();
        let v = game_rounds.iter().map(|round| {
            todo!()
        });
        let colors = vec!["red", "blue", "green"];

        // let game_vec = game.get(1).unwrap().split(";").collect::<Vec<&str>>();
        // let game_index = game_vec[0].split(" ").collect::<Vec<&str>>().get(1);
        todo!()
    }
}

fn main() {
    let dataset = read_csv("./src/data/2_cube_condrum_dataset.csv");
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::*;

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red")]
    #[case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red")]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green")]
    fn examples(#[case] input: &str) {
        let game_data = GameDescription::from_string(input);
    }
}
