use std::path::PathBuf;

use rand::{Rng, distr::Alphanumeric};

use crate::clean_up;

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Busy,
    Done,
}

pub struct Session {
    code: String,
    pub state: RunningState,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            code: Self::randomize_code(),
            state: RunningState::default(),
        }
    }
}

impl Session {
    const TEMP_NAME: &str = "firefly_temp";

    fn randomize_code() -> String {
        let rng = rand::rng();
        let rand_string: String = rng
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();

        rand_string
    }

    pub fn get_code(&self) -> String {
        self.code.clone()
    }

    pub fn get_temp(&self) -> PathBuf {
        PathBuf::from(format!("{}_{}.flac", Self::TEMP_NAME, self.code))
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        clean_up(self).unwrap();
    }
}
