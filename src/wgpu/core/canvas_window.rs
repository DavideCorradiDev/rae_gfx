use rae_app::window::Window;

use super::Surface;

#[derive(Debug)]
pub struct CanvasWindow {
    window: Window,
    surface: Surface,
}
