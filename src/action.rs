use crate::components::Component;

pub enum Action {
    None,
    ChangeComponent(Box<dyn Component + Send>),
    Quit,
}