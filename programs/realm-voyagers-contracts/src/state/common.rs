use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace)]
pub struct Rect {
    pub top_left: Position,
    pub bottom_right: Position,
}

impl Position {
    pub fn distance_squared(&self, other: &Position) -> i32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    pub fn distance(&self, other: &Position) -> i32 {
        (self.distance_squared(other) as f64).sqrt() as i32
    }
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Rect {
    pub fn contains(&self, position: &Position) -> bool {
        position.x >= self.top_left.x
            && position.x <= self.bottom_right.x
            && position.y >= self.top_left.y
            && position.y <= self.bottom_right.y
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        self.top_left.x <= other.bottom_right.x
            && self.bottom_right.x >= other.top_left.x
            && self.top_left.y <= other.bottom_right.y
            && self.bottom_right.y >= other.top_left.y
    }

    pub fn area(&self) -> i32 {
        (self.bottom_right.x - self.top_left.x) * (self.bottom_right.y - self.top_left.y)
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            top_left: Position::default(),
            bottom_right: Position::default(),
        }
    }
}
