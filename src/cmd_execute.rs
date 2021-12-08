use encoding_rs::{IBM866};
use std::io::Write;
use std::fs::File;

//Run cmd with arguments
//return result string(IBM866 encoding in windows cmd)
pub fn cmd_execute(args:Vec<String>)->Result<String,std::io::Error>{
    let output=std::process::Command::new("cmd")
        .args(args)
        .output()
        .expect("failed to execute process");
    let (cow, _encoding_used, had_errors) = IBM866.decode(&output.stdout);
    match had_errors {
        true => Err(std::io::Error::new(std::io::ErrorKind::Other, "Encoding error")),
        false => Ok(cow.into_owned()),
    }
}

pub fn check_files_in_subfolder_log(dir_selected:&str)->Result<usize,std::io::Error>{
    let mut f = File::create("LOG_rling.txt")?;
    let subfolders_count=check_files_in_subfolder(dir_selected,&mut 0,&mut f)?;
    Ok(subfolders_count)
}

//Check subfolders
//return depth of lying files
//if the folder does not contain file - skip subfolders
fn check_files_in_subfolder(dir_selected:&str,num_folders_with_files:&mut usize,file_log:&mut File)->Result<usize,std::io::Error>{
    let mut file_exist=false;
    let mut not_ascii=false;
    let mut count_max=0;
    for path in std::fs::read_dir(&dir_selected).unwrap() {
        match path{
            Ok(path_ok)=>{
                match path_ok.file_type() {
                    Ok(file_type_ok) => {
                        match file_type_ok.is_file() {
                            true => {
                                match file_exist{
                                    true=>(),
                                    false=>{
                                        file_exist=true
                                    },
                                }
                            },
                            false => (),
                        }
                    },
                    Err(e) => println!("{:?}", e),
                }
            },
            Err(e)=>println!("{:?}", e)
        }
    }
    match file_exist{
        true=>{
            *num_folders_with_files=1;
            for path in std::fs::read_dir(&dir_selected).unwrap() {
                match path{
                    Ok(path_ok)=>{
                        match path_ok.file_type() {
                            Ok(file_type_ok)=>{
                                match file_type_ok.is_dir(){
                                    true => {
                                        match if_ascii(&path_ok.path().to_str().unwrap()){
                                            true=>{
                                            },
                                            false=>{
                                                not_ascii=true;
                                                file_log.write_all(format!("NOT_ASCII(rename to english):{:?}\r\n", &path_ok.path().to_str().unwrap()).as_bytes()).unwrap_or(());
                                            },
                                        }
                                        match check_files_in_subfolder(&path_ok.path().to_str().unwrap(),num_folders_with_files,file_log){
                                            Ok(_i)=>{
                                                *num_folders_with_files+=1;
                                            },
                                            Err(_e)=>not_ascii=true,
                                        }
                                    },
                                    false => {
                                        match if_ascii(&path_ok.path().to_str().unwrap()){
                                            true=>{ 
                                            },
                                            false=>{
                                                not_ascii=true;
                                                file_log.write_all(format!("NOT_ASCII(rename to english):{:?}\r\n", &path_ok.path().to_str().unwrap()).as_bytes()).unwrap_or(());
                                            },
                                        }
                                    },
                                }
                            },
                            Err(e)=>println!("{:?}", e)
                        }
                    },
                    Err(e)=>println!("{:?}", e)
                }
                if num_folders_with_files>&mut count_max{
                    count_max=*num_folders_with_files;
                } 
            }
        },
        false=>()
    }
    match not_ascii {
        true => Err(std::io::Error::new(std::io::ErrorKind::Other, "Not ascii, all files or folders writed to LOG_rling.txt")),
        false => Ok(count_max),
    }
    
}

pub fn if_ascii(string:&str)->bool{
    string.bytes().all(|c| c.is_ascii())
}

pub fn check_ap_path(dir_selected:&str)->Vec<String>{
    match check_files_in_subfolder_log(&dir_selected){
        Ok(subdirs_with_files) => {
            let mut vec_sub_dirs:Vec<String>=Vec::new();
            if subdirs_with_files>0{
                for i in 1..subdirs_with_files+1{
                    let vec1=vec!["/*";i];
                    let str_dirs: String = vec1.into_iter().map(|i| i.to_string()).collect::<String>();
                    vec_sub_dirs.push(str_dirs);
                }
            }
            else{
                nwg::error_message("Error","0 files in dir");
            }
            let str_subdirs: Vec<String> = vec_sub_dirs.into_iter().map(|i| format!("{}{}",dir_selected,i)).collect::<Vec<String>>();
            str_subdirs
        },
        Err(_e) => {
            nwg::error_message("Error","Folder or file contains non ASCII symbols.\nRead LOG_rling.txt");
            Vec::new()
        },
    }
}