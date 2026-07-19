// BAD: Passive surface contains business logic
pub struct StatusComponent {
    state: StatusViewModel,
}

impl StatusComponent {
    pub fn render(&self) -> RenderedText {
        if self.state.violations().len() > 10 {
            RenderedText::from("Too many violations")
        } else {
            RenderedText::from("OK")
        }
    }
}
