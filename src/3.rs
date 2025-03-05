// A function that returns a string with a random color hex code
fn get_random_color() -> String {
    // Create an array of all possible color hex codes
    let colors = ["#ff0000", "#00ff00", "#0000ff", "#ffff00", "#ff00ff"];
    // Get a random index from the array
    let random_index = rand::thread_rng().gen_range(0..colors.len());
    // Return the color hex code at the random index
    return colors[random_index].to_string();
}
