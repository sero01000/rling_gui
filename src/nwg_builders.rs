pub fn build_checkbox(checkbox: &mut nwg::CheckBox, window: &nwg::Window, text:&str, state:nwg::CheckBoxState) {
    nwg::CheckBox::builder()
        .text(text)
        .flags(nwg::CheckBoxFlags::VISIBLE)
        .parent(window)
        .check_state(state)
        .build(checkbox).unwrap_or(());
}

pub fn build_button(button: &mut nwg::Button, window: &nwg::Window, text:&str,flags:nwg::ButtonFlags) {
    nwg::Button::builder()
        .text(text)
        .flags(flags)
        .parent(window)
        .build(button).unwrap_or(());
}

pub fn build_textinput(tbox: &mut nwg::TextInput, window: &nwg::Window, text:&str) {
    nwg::TextInput::builder()
        .text(text)
        .parent(window)
        .build(tbox).unwrap_or(());
}

pub fn file_dialogue_layout(dialog: &mut nwg::FileDialog,action:nwg::FileDialogAction,text:&str) {
    nwg::FileDialog::builder()
        .title(text)
        .action(action)
        .multiselect(false)
        //.filters("a(*.txt)|b(*.*)")//
        .build(dialog).unwrap_or(());
}

pub fn build_label(label: &mut nwg::Label, window: &nwg::Window, text:&str) {
    nwg::Label::builder()
        .text(text)
        .parent(window)
        .build(label).unwrap_or(());
}

pub fn build_richlabel(label: &mut nwg::RichLabel, window: &nwg::Window, text:&str) {
    let mut font = nwg::Font::default();

    nwg::Font::builder()
        .size(16)
        .family("Segoe UI")
        .build(&mut font).unwrap_or(());

    nwg::RichLabel::builder()
        .text(text)
        .font(Some(&font))
        .flags(nwg::RichLabelFlags::VISIBLE|nwg::RichLabelFlags::MULTI_LINE)
        .parent(window)
        .build(label).unwrap_or(());
}