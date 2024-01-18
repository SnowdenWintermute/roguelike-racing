use rand::Rng;

pub fn generate_random_monster_name() -> String {
    let mut rng = rand::thread_rng();
    let random_name_words = vec![
        "basilisk",
        "toad",
        "beetle",
        "demon",
        "skeleton",
        "diablo",
        "devil",
        "snake",
        "scavenger",
        "slime",
        "worm",
        "wurm",
        "drake",
        "dragon",
        "rock",
        "ghost",
        "spirit",
        "zombie",
        "spider",
        "elemental",
        "fire",
        "water",
        "ice",
        "dark",
        "lightning",
    ];

    let mut random_name: Vec<&str> = Vec::new();
    let name_word_length: u8 = rng.gen_range(1..=1);
    for n in 0..=name_word_length {
        let random_word_index = rng.gen_range(0..random_name_words.len());
        let random_word = random_name_words[random_word_index];
        random_name.push(random_word);
        if n != name_word_length {
            random_name.push("_")
        }
    }

    let to_return = random_name.concat();
    to_return
}

pub fn generate_random_username() -> String {
    let mut rng = rand::thread_rng();
    let random_name_words = vec![
        "long", "strand", "bun", "hebrew", "hammer", "nazna", "hippi", "shwarz", "sicks", "baby",
        "snowden", "obi-wan", "qui-gon", "jin", "mcmom", "sandy", "cambie", "tj", "boxer", "mini",
        "flash", "action", "bisu", "sharp",
    ];

    let mut random_name: Vec<&str> = Vec::new();
    let name_word_length: u8 = rng.gen_range(1..=2);
    for n in 0..=name_word_length {
        let random_word_index = rng.gen_range(0..random_name_words.len());
        let random_word = random_name_words[random_word_index];
        random_name.push(random_word);
        if n != name_word_length {
            random_name.push("_")
        }
    }

    let to_return = random_name.concat();
    to_return
}

pub fn calculate_number_of_pages(page_size: usize, num_items: usize) -> usize {
    let full_pages = num_items / page_size;
    let remaining_items = num_items % page_size;

    if remaining_items > 0 {
        full_pages + 1
    } else {
        full_pages
    }
}

pub fn add_i16_to_u16_and_clamp_to_max(u16: u16, i16: i16, max: u16) -> u16 {
    let value = u16 as i16 + i16;
    let value = if value < 0 { 0 } else { value as u16 };
    if value > max {
        max
    } else {
        value
    }
}

pub fn vec_shift<T>(vec: &mut Vec<T>) -> Option<T> {
    if vec.first().is_some() {
        Some(vec.remove(0))
    } else {
        None
    }
}
