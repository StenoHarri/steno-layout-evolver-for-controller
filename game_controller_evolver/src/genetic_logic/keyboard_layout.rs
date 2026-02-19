#[derive(Debug, Clone)]
pub(crate) struct KeyboardLayout {
    pub(crate) left_chords: Vec<(String, String)>,
    pub(crate) right_chords: Vec<(String, String)>,
}