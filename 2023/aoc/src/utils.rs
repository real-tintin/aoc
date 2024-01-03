#[cfg(test)]
pub mod test_utils {
    pub fn get_input_path(day: u32) -> std::path::PathBuf {
        let root = std::env::var("INPUTS_ROOT");

        match root {
            Ok(root) => std::path::PathBuf::from(root).join(format!("day_{:}.txt", day)),
            Err(_) => panic!("INPUTS_ROOT not set"),
        }
    }
}
