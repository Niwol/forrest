use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    max: i32,
    current: i32,
}

impl Health {
    pub fn new(max: i32, current: i32) -> Self {
        Self { max, current }
    }

    pub fn new_full(max: i32) -> Self {
        Self { max, current: max }
    }

    pub fn set_max(&mut self, max: i32) {
        self.max = max;
    }

    pub fn set_current(&mut self, current: i32) {
        self.current = i32::clamp(current, 0, self.max);
    }

    pub fn max(&self) -> i32 {
        self.max
    }

    pub fn current(&self) -> i32 {
        self.current
    }

    pub fn change(&mut self, amount: i32) {
        self.current = i32::clamp(self.current + amount, 0, self.max);
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0
    }

    pub fn is_alive(&self) -> bool {
        !self.is_dead()
    }
}
