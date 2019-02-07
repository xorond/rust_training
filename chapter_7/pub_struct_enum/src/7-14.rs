mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // body goes here
        }
    }
}

use self::sound::instrument;

fn main() {
    instrument::clarinet();
    instrument::clarinet();
    instrument::clarinet();
}
