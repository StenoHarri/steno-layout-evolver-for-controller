use crate::genetic_logic::keyboard_layout::KeyboardLayout;

#[derive(Debug, Clone)]
pub(crate) struct MeasuredKeyboardLayout {
    pub(crate) layout: KeyboardLayout,
    pub(crate) fitness: f64,
}