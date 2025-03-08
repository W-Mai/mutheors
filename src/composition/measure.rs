#[derive(Clone, Copy)]
pub struct Measure {}

impl Measure {
    pub fn new() -> Self {
        Measure {}
    }

    pub fn test(&mut self) {
        // Test function to demonstrate usage
        println!("Measure test function called");
    }
}
