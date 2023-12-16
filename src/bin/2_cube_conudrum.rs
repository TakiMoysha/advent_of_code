use advent_of_code_2023::read_csv;

#[derive(Debug)]
pub struct GameCondition {
    pub red: u8,
    pub blue: u8,
    pub green: u8,
}

#[derive(Debug)]
pub struct GameDescription {
    pub id: usize,
    // pub red: u8,
    // pub blue: u8,
    // pub green: u8,
}

impl GameDescription {
    fn from_string_with_condition<'a, 'b>(
        input: &'a str,
        condition: &'b GameCondition,
    ) -> Result<Self, &'static str> {
        let game: Vec<&str> = input.split(":").collect();

        let game_header: Vec<&str> = game.get(0).unwrap().rsplit(" ").collect();
        let game_id = game_header.get(0).unwrap().parse::<u8>().unwrap();

        let game_rounds: Vec<&str> = game.get(1).unwrap().split(";").map(|s| s.trim()).collect();

        let rounds_validate = game_rounds.iter().for_each(|round| {
            let v = round.split(" ").collect::<Vec<&str>>();
            let num = v[0].parse::<u8>().unwrap();
            let color = v[1];

            match color {
                "red" if condition.red < num => {}
                "blue" if condition.blue < num => {}
                "green" if condition.green < num => { }
                _ => { panic!("invalid color") }
            }
        });

        // let rounds_validate: Vec<&&str> = game_rounds.iter().filter_map(|round| {
        //     let v = round.split(" ").collect::<Vec<&str>>();
        //     let num = v[0].parse::<u8>().unwrap();
        //
        //     if v[1] == "red" && condition.red < num
        //         || v[1] == "blue" && condition.blue < num
        //         || v[1] == "green" && condition.green < num
        //     {
        //     } else {
        //     }
        // }).collect();
        //
        // println!("rv: {rounds_validate:?}");
        todo!();
        // let game_vec = game.get(1).unwrap().split(";").collect::<Vec<&str>>();
        // let game_index = game_vec[0].split(" ").collect::<Vec<&str>>().get(1);
        // todo!()
    }
}

fn main() {
    let game_condition = GameCondition {
        red: 12,
        green: 13,
        blue: 14,
    };
    let dataset: Vec<String> = read_csv("./src/data/2_cube_condrum_dataset.csv");
    let valid_rounds = dataset.iter().for_each(|game| {
        let gd = GameDescription::from_string_with_condition(game, &game_condition);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let games = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let game_condition = GameCondition {
            red: 12,
            green: 13,
            blue: 14,
        };
        games.iter().for_each(|game| {
            let gd = GameDescription::from_string_with_condition(game, &game_condition);
            // gd.unwrap().id
        });
    }
}
