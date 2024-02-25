use chrono::Local;

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

pub fn server_log(message: &str) {
    let current_time = Local::now();

    // Format the current time into a human-readable string
    let formatted_time = current_time.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("{}: {message}", formatted_time)
}
