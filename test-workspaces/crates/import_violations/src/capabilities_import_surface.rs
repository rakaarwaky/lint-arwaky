// Fixture: AES201 — capabilities importing surface (forbidden layer).
use surface::command::SurfaceCommand;
use taxonomy::vo::OrderVO;

pub fn process() -> OrderVO {
    let _cmd = SurfaceCommand::new();
    OrderVO::new()
}
