#![windows_subsystem="windows"]
extern crate native_windows_gui as nwg;
use std::rc::Rc;
mod cmd_execute;
mod nwg_builders;
pub use crate::cmd_execute::*;
pub use crate::nwg_builders::*;

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let mut window = Default::default();
    let mut label_file1 = nwg::Label::default();
    let mut label_file2 = nwg::Label::default();

    let mut text_file1 = nwg::TextInput::default();
    let mut text_file2 = nwg::TextInput::default();

    let mut button_opef_file1=Default::default();
    let mut button_opef_file2=Default::default();

    let mut label_uniq = nwg::Label::default();
    //let mut label_doubles = nwg::Label::default();

    let mut text_uniq = nwg::TextInput::default();
    //let mut text_doubles = nwg::TextInput::default();

    let mut label_settings = nwg::Label::default();
    let mut check_hash_search = nwg::CheckBox::default();
    let mut check_binary_search = nwg::CheckBox::default();
    let mut check_use_files_instead_of_memory = nwg::CheckBox::default();//Use files instead of memory
    //let mut check_save_doubles = nwg::CheckBox::default();//save doubles not working in rling, idk why

    let mut button_start=Default::default();
    let mut file1_dialog = nwg::FileDialog::default();
    let mut ap_path_dialog = nwg::FileDialog::default();
    let mut label_donation=nwg::RichLabel::default();

    nwg::Window::builder()
        .size((545, 220))
        .position((300, 300))
        .title("rling_gui @phones_parts @ubrute")
        .accept_files(true)
        .build(&mut window)
        .unwrap();

    let grid=nwg::GridLayout::builder()
        .parent(&window)
        .spacing(1);
    let layout = Default::default();

    build_label(&mut label_file1, &window, "file original:");
    build_label(&mut label_file2, &window, "uniqs folder:");

    build_textinput(&mut text_file1, &window,"original.txt");
    build_textinput(&mut text_file2, &window,"uniqs");
    build_button(&mut button_opef_file1,&window, "open file",nwg::ButtonFlags::VISIBLE);
    build_button(&mut button_opef_file2,&window, "open uniqs folder",nwg::ButtonFlags::VISIBLE);

    build_label(&mut label_uniq, &window, "save uniq path:");
    //build_label(&mut label_doubles, &window, "save doubles path:");

    build_textinput(&mut text_uniq, &window,"uniq_rling.txt");
    //build_textinput(&mut text_doubles, &window,"doubles.txt");

    build_label(&mut label_settings, &window, "settings :");
    build_checkbox(&mut check_hash_search, &window, "hash search",nwg::CheckBoxState::Checked);
    build_checkbox(&mut check_binary_search, &window, "binary search",nwg::CheckBoxState::Unchecked);
    build_checkbox(&mut check_use_files_instead_of_memory, &window, "use files instead of memory",nwg::CheckBoxState::Unchecked);
    //build_checkbox(&mut check_save_doubles, &window, "save doubles",nwg::CheckBoxState::Unchecked);
    let text_donat="BTC:1HXYcDDZed3ei9Ndm9Au65ZfoCqubEEvku\nLTC:MRGWKyuPV6YQ42Jh4BVgNGHBuSQrXsmkfA\nETH/BSC:0x3d5Dbc0E218D0084Ce755803724F614ea70191F6\nXMR:8C1CpeKGzwhF9pAL3HamhoCLWReLwntNjELJKAqWg4pXgUDW8GLFnW2VdWz9NJXPSVQUjf1CvpcC1gtNVWf74RoW5AUjZiH";
    build_richlabel(&mut label_donation, &window,&text_donat);
    build_button(&mut button_start,&window, "start",nwg::ButtonFlags::VISIBLE);
    file_dialogue_layout(&mut file1_dialog,nwg::FileDialogAction::Open,"Select file-original");
    file_dialogue_layout(&mut ap_path_dialog,nwg::FileDialogAction::OpenDirectory,"Select ap folder");
    
    grid
        .child(0, 0, &label_file1)
        .child(1, 0, &label_file2)
        .child_item(nwg::GridLayoutItem::new(&text_file1, 0, 1, 1, 1))
        .child_item(nwg::GridLayoutItem::new(&text_file2, 1, 1, 1, 1))

        .child_item(nwg::GridLayoutItem::new(&button_opef_file1, 0, 2, 1, 1))
        .child_item(nwg::GridLayoutItem::new(&button_opef_file2, 1, 2, 1, 1))

        .child_item(nwg::GridLayoutItem::new(&label_uniq, 0, 3, 1, 1))
        //.child_item(nwg::GridLayoutItem::new(&label_doubles, 1, 3, 1, 1))

        .child_item(nwg::GridLayoutItem::new(&text_uniq, 0, 4, 1, 1))
        //.child_item(nwg::GridLayoutItem::new(&text_doubles, 1, 4, 1, 1))

        .child_item(nwg::GridLayoutItem::new(&label_settings, 2, 0, 1, 1))
        .child_item(nwg::GridLayoutItem::new(&check_hash_search, 2, 1, 1, 1))
        .child_item(nwg::GridLayoutItem::new(&check_binary_search, 2, 2, 1, 1))
        .child_item(nwg::GridLayoutItem::new(&check_use_files_instead_of_memory, 2, 3, 1, 1))
        //.child_item(nwg::GridLayoutItem::new(&check_save_doubles, 3, 2, 1, 1))
        .child_item(nwg::GridLayoutItem::new(&label_donation,1,4,2,2))

        .child_item(nwg::GridLayoutItem::new(&button_start, 0, 5, 1, 1))
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
            nwg::Event::OnFileDrop=>{
                let drop = _evt_data.on_file_drop();
                let path_drop = &drop.files()[0];//path_text
                let path=std::path::Path::new(path_drop);
                if path.is_file(){
                    text_file1.set_text(path_drop);
                    match path.file_name() {
                        Some(filename) => {
                            let ap_path=text_file2.text();
                            let text_uniq_path=format!(
                                "{}\\{}.uniq_rling.txt",
                                ap_path.as_str(),
                                filename.to_str().unwrap_or("Error")
                                );
                            text_uniq.set_text(&text_uniq_path);
                        },
                        None => (),
                    }
                } else if path.is_dir() {
                    let text_file1_1=&text_file1.text();
                    let path_file1=std::path::Path::new(&text_file1_1);
                    let filename_file1=path_file1.file_name().unwrap_or(std::ffi::OsStr::new("uniq_rling.txt")).to_str().unwrap_or("uniq_rling.txt");
                    text_file2.set_text(path_drop);
                    let text_uniq_path=format!(
                        "{}\\{}.uniq_rling.txt",
                        path_drop,
                        filename_file1,
                        );
                    text_uniq.set_text(&text_uniq_path);
                }
            },
            nwg::Event::OnButtonClick => 

                if &handle == &button_start {//start
                    let mut args=vec![
                        //"&pause".to_string(),
                        "/C".to_string(),
                        "start".to_string(),
                        "".to_string(),
                        "rling.exe".to_string(),
                        text_file1.text().to_string(),
                        text_uniq.text().to_string(),
                        ];
                    let dir_or_file_selected=text_file2.text();
                    let path=std::path::Path::new(&dir_or_file_selected);
                    if path.is_file(){//is file
                        args.push(dir_or_file_selected);
                    } else if path.is_dir() {//is dir
                        for i in check_ap_path(&dir_or_file_selected){
                            println!("{:?}", i);
                            args.push(i);
                        }
                    } else{
                        println!("NOT_path NOT_dir {:?}",path);
                    }

                    match check_binary_search.check_state(){
                        nwg::CheckBoxState::Checked=>args.push("-b".to_string()),
                        _=>(),
                    }
                    match check_use_files_instead_of_memory.check_state(){
                        nwg::CheckBoxState::Checked=>args.push("-f".to_string()),
                        _=>(),
                    }
                    
                    std::thread::spawn(move || {
                        let cmd=cmd_execute(args);
                        println!("{:?}", cmd);
                    });
                } else if &handle == &button_opef_file1{//file1
                    file1_dialog.run(Some(&button_opef_file1));
                    if let Ok(directory) = file1_dialog.get_selected_item() {
                        let dir_selected=&directory.into_string().unwrap_or("Error".to_string());
                        let splited: Vec<&str> =dir_selected.split("\\").collect();
                        println!("{:?}", splited);
                        let filename=splited.last();

                        text_file1.set_text(&dir_selected);
                        text_uniq.set_text(&format!("{}.uniq_rling.txt", filename.unwrap_or(&"Error_...")))
                    }

                } else if &handle == &button_opef_file2{//uniq path
                    ap_path_dialog.run(Some(&button_opef_file2));
                    if let Ok(directory) = ap_path_dialog.get_selected_item() {
                        let dir_selected=directory.into_string().unwrap_or("Error".to_string());
                        text_file2.set_text(&dir_selected);
                        check_ap_path(&dir_selected);
                    }
                } else if &handle == &check_hash_search{//hash
                    check_hash_search.set_check_state(nwg::CheckBoxState::Checked);
                    check_binary_search.set_check_state(nwg::CheckBoxState::Unchecked);
                } else if &handle == &check_binary_search{//binary
                    check_binary_search.set_check_state(nwg::CheckBoxState::Checked);
                    check_hash_search.set_check_state(nwg::CheckBoxState::Unchecked);
                }
                // } else if &handle == &check_save_doubles{//save doubles
                //     println!("check_save_doubles");
                // },
            _ => {}
        }
    });
    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler);
}
