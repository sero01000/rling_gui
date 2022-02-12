use std::rc::Rc;

pub fn window_msg(text:&str){
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let layout = Default::default();
    let mut window = Default::default();
    let mut label_donation=nwg::RichLabel::default();

    nwg::Window::builder()
        .size((630, 520))
        .position((300, 300))
        .title("RESULT rling_gui @phones_parts @ubrute")
        .accept_files(true)
        .build(&mut window)
        .unwrap();

    let grid=nwg::GridLayout::builder()
        .parent(&window)
        .spacing(1);

   
    build_richlabel(&mut label_donation, &window,&text);
    grid
        .child(0, 0, &label_donation)
        .build(&layout)
        .unwrap();
    let window = Rc::new(window);
    let events_window = window.clone();

    let handler = nwg::full_bind_event_handler(&window.handle, move |evt, _evt_data, handle| {
        use nwg::Event as E;
        match evt {
            E::OnWindowClose => 
                if &handle == &events_window as &nwg::Window {
                    nwg::stop_thread_dispatch();
                },
            _ => {}
            }
        });
    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler);
}

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