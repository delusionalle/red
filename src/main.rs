use crate::editor::Editor;

mod editor;
mod terminal;
mod document;
mod row;

fn main() {
    Editor::default().run()
}
