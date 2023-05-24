#[cfg(test)]

mod tests {
    use std::path::Path;
    use crate::hide_unhide::*;

    #[test]
    fn test_hide() {
        let directory = String::from("C:/Users/Mehedi Hasan Sizar/Desktop/Goal");
        let directory = directory.trim().trim_matches('"'); // remove leading/trailing whitespace and quotes
        let file_path = Path::new(directory);  // Inspect the path from the directory provided
        let directory = file_path.to_path_buf(); // Convert the Path to an owned PathBuf.
        
        let hidden = hide_directory(&directory);
        assert!(hidden.is_ok());
    }

    #[test]
    fn test_unhide() {
        let directory = String::from("C:/Users/Mehedi Hasan Sizar/Desktop/Goal");
        let directory = directory.trim().trim_matches('"'); // remove leading/trailing whitespace and quotes
        let file_path = Path::new(directory);  // Inspect the path from the directory provided
        let directory = file_path.to_path_buf(); // Convert the Path to an owned PathBuf.
        
        let unhidden = unhide_directory(&directory);
        assert!(unhidden.is_ok());
    }

}
