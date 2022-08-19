use super::window::Window;

pub struct Context<'w> {
    pub window: &'w mut Window,
    pub game_time: f64,
}
