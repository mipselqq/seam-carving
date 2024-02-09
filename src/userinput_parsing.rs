pub fn parse_target_dimension(target_dimension: String, original_dimension: u32) -> u32 {
    if target_dimension.contains('%') {
        let percent: f32 = target_dimension.replace('%', "").parse().expect("Failed to parse input in percents");

        return (original_dimension as f32 * percent / 100f32) as u32;
    }

    target_dimension.parse::<u32>().expect("\
        OOPSIE WOOPSIE!! Uwu We made a lucky wucky!!A wittle lucko boingo!\
        The code monkeys at our headquarters are working VEWY HAWD to fix this!
    ")
}
