pub trait WordSplit {
    fn split_by_chars(&self, length: usize) -> Vec<String>;
}

impl WordSplit for String {
    fn split_by_chars(&self, length: usize) -> Vec<String> {
        let words: Vec<&str> = self.split_whitespace().collect();
        let output_capacity = self.len() + self.len() % length + 1;
        let mut lines: Vec<String> = Vec::with_capacity(output_capacity);
        let mut current_line = String::with_capacity(length);
        let (mut chars, mut initialized) = (0, false);
        for word in words {
            if chars + word.len() >= length {
                // if character length met or exceeded
                lines.push(current_line.clone());
                current_line.clear();
                current_line = String::from(word);
                chars = word.len();
            } else if !initialized {
                current_line = String::from(word);
                chars = word.len();
                initialized = true;
            } else {
                current_line = current_line + " " + word;
                chars += word.len() + 1;
            }
        }
        if !current_line.is_empty() { lines.push(current_line); }
        lines
    }
}

#[test]
fn test_split_by_chars() {
    let input = String::from("This is a test string for split_by_chars.");
    let sections = input.split_by_chars(10);
    let mut iter = sections.iter();
    assert_eq!(9, iter.next().unwrap().len());
    assert_eq!(4, iter.next().unwrap().len());
    assert_eq!(10, iter.next().unwrap().len());
    assert_eq!(15, iter.next().unwrap().len());
}
