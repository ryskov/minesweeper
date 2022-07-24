use bevy::{prelude::*, core::Stopwatch};

#[derive(Deref, DerefMut)]
pub struct GameTime(Stopwatch);

impl GameTime {
    pub fn to_string(&self) -> String {
        let duration = self.elapsed();
        let minutes = (duration.as_secs() as f32 / 60.).floor() as u16;
        let seconds = (duration.as_secs() % 60) as u16;
        let minute_string = if minutes < 10 {
            format!("0{}", minutes)
        } else {
            format!("{}", minutes)
        };
        let second_string = if seconds < 10 {
            format!("0{}", seconds)
        } else {
            format!("{}", seconds)
        };

        format!("{}:{}", minute_string, second_string)
    }

    pub fn new_paused() -> Self {
        let mut game_time_watch = Stopwatch::new();
        game_time_watch.pause();
        Self(game_time_watch)
    }

    pub fn reset(&mut self) {
        self.0.reset();
    } 
}