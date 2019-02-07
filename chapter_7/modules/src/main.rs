mod sound {
    pub mod instrument {
        pub fn clarinet() {
            super::breathe_in()
            }
        }
}

fn breathe_in() {
    // Function body goes here
}

fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();
}
