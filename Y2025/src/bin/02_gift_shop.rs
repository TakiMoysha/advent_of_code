fn main() {
    todo!()
}

// ===========================================================
// tests
// ===========================================================

#[derive(Debug)]
struct IDRange(u32, u32);

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        "
        11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
        1698522-1698528,446443-446449,38593856-38593862,
        565653-565659,824824821-824824827,2121212118-2121212124
        "
    )]

    fn test_demo_input(#[case] raw_input: &str) {
        // sanitize and clean input
        let raw_input = raw_input.replace("\n", ",");
        let input: Vec<&str> = raw_input
            .split(",")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        // parse input
        let id_ranges: Vec<IDRange> = input
            .iter()
            .map(|s| {
                let id_range: Vec<u32> = s.split("-").map(|s| s.parse().unwrap()).collect();
                IDRange(id_range[0], id_range[1])
            })
            .collect();
        println!("{:?}", id_ranges);
    }
}
