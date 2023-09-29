use rand::{rngs::ThreadRng, Rng};

pub fn generate_random_username() -> String {
    let mut rng = rand::thread_rng();
    let random_name_words = vec![
        "long", "strand", "bun", "hebrew", "hammer", "nazna", "hippi", "shwarz", "daik", "essen",
        "sicks", "baby", "snow", "den", "_", "salvo", "angel", "rando", "dax", "obi", "wan", "qui",
        "gon", "jin",
    ];

    let mut random_name: Vec<&str> = Vec::new();
    let name_word_length: u8 = rng.gen_range(1..5);
    for n in 0..name_word_length {
        let random_word_index = rng.gen_range(0..random_name_words.len());
        let random_word = random_name_words[random_word_index];
        random_name.push(random_word);
    }

    let to_return = random_name.concat();
    to_return
}
