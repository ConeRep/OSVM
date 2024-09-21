use std::{env, f32::NAN, fs::File, io::Read, ops::{Range, RangeBounds}, os::unix::process::CommandExt, path::PathBuf, process::{exit, Command}};

pub struct Preprocessor {}

impl Preprocessor {
    fn remove_line_by_sstr(self: &Self, starts_with: &str,  source: String, index: usize) -> String {
        let mut new_source = String::new();
        for line in source.lines() {
            if line.starts_with(starts_with) {
                continue;
            }
            
            new_source.push_str(line);
            new_source.push('\n');
        }
        
        new_source
    }
    
    fn get_string(self: &Self, line: &str) -> String {
        let mut string = String::new();
        let mut found = false;
        for c in line.chars() {
            if c == '"' {
                if found != true {
                    found = true;
                } else {
                    found = false;
                }
                continue;
            }
            
            if found == true {
                string.push(c);
            }
        }
        
        string
    }
    
    fn get_cindex(self: &Self, index: usize, source: &str) -> usize {
        let mut lindex = 0;
        let mut cindex = 0;
        for c in source.chars() {
            if lindex == index {
                break;
            }
            
            if c != '\n' {
                cindex += 1;
            } else {
                cindex += 1;
                lindex += 1;
            }
        }
        
        cindex
    }
    
    pub fn process_includes(self: &Self, file_path: String, mut source: String) -> String {
        println!("[Preprocessor] => includes => {}", file_path);
        let mut index = 0;
        for mut line in source.clone().lines() {
            line.trim();
            if line.starts_with("%") {
                if line.replace("%", "").starts_with("include") {
                    let path = self.get_string(line);
                    let mut file;
                    match env::var("OSVM_LIBS_DIR") {
                        Ok(_) => {}
                        Err(_) => {
                            eprintln!("[Error]: OSVM_LIBS_DIR NOT SET!");
                            exit(1);
                        }
                    }
                    
                    match File::open(PathBuf::from(env::var("OSVM_LIBS_DIR").unwrap()).join(path.clone())) {
                        Ok(_) => file = File::open(PathBuf::from(env::var("OSVM_LIBS_DIR").unwrap()).join(path)).unwrap(),
                        Err(_) => file = File::open(PathBuf::from(file_path.clone()).parent().unwrap().join(path)).unwrap(),
                    }
                    
                    let mut include_source = String::new();
                    let _ = file.read_to_string(&mut include_source);
                    
                    let mut isource = include_source + "\n";
                    source.insert_str(self.get_cindex(index + 1, source.as_str()), isource.as_str());
                }
            }
            
            index += 1;
        }
        
        source = self.remove_line_by_sstr("%include", source, index);
        source
    }
    
    pub fn process_source(self: &Self, file_path: String, mut source: String) -> String {
        println!("[Preprocessor] => all => {}", file_path);
        let mut index = 0;
        
        let mut macros = Vec::<(&str, &str)>::new();
        for mut line in source.lines() {
            line.trim();
            if line.starts_with("%") {
                if line.replace("%", "").starts_with("define") {
                    let splitted: Vec<&str> = line.trim().split_whitespace().collect();
                    macros.push((splitted[1], splitted[2]));
                }
            }
            
            index += 1;
        }
        
        let mut source = self.remove_line_by_sstr("%", source.clone(), index);
        for _macro in macros.clone() {
            source = source.replace(_macro.0, _macro.1);
        }
        
        source
    }
}