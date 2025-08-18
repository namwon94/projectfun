pub fn if_let() {
    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println!("The maxium is configured to be {}", max);
    }
}