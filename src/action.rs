use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum Action {
    MoveRight,
    MoveLeft,
    MoveDown,
    RotateClockwise,
    RotateCounterClockwise,
    HardDrop,
    SoftDrop,
    HoldPiece,

    FallPiece,
    LockPiece,
    GameOver,
}
