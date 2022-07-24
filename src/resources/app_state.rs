#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    Paused,
    GameOver,
    Out,
}