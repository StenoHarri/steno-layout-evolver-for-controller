#[derive(Debug, Clone)]
pub(crate) struct KeyboardLayout {
    pub(crate) left_chord_genes: Vec<(String, String)>,
    pub(crate) right_chord_genes: Vec<(String, String)>,
}