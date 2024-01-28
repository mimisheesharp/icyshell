use std::fs;
use std::io::Write;
use std::io::{stdin, stdout};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::os::windows::process;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
use std::{env, path};
fn main() {
    read(true);
}

fn read(success: bool) {
    let home_directory = env::home_dir().unwrap();
    let current_directory = env::current_dir().unwrap();

    let current_directory_str = current_directory.to_string_lossy();
    let mut modified_path = current_directory_str.replace(&*home_directory.to_string_lossy(), "~");



    println!("Modified path: {}", modified_path);
    if success == true {
        print!("\x1b[37m{} \x1b[32m@ \x1b[34m", modified_path);
    } else if success == false {
        print!("\x1b[30m{} \x1b[31m* \x1b[34m", modified_path);
    }
    let mut input = String::new();
    stdout().flush().unwrap();
    stdin().read_line(&mut input).expect("error");
    let args: Vec<&str> = input.split(" ").collect();
    parser(args)
}
fn parser(args: Vec<&str>) {
    let maincmd = args[0].to_string(); // maincmd string
    let mut pipe: bool = false;
    let mut pipesplitcmds: Vec<&str> = Vec::new(); // | split!
    let mut option: Vec<&str> = Vec::new(); // option vector
    let mut a: Vec<&str> = Vec::new(); // else vector
    let mut all: Vec<&str> = Vec::new(); //args vector
    let mut h: bool = false; // -h option boolean
    let mut e: bool = false; // -e option boolean
    let mut l: bool = false; // -l option boolean
    let mut p: bool = false;
    let mut r: bool = false;
    let mut i: i32 = 0; // counter
    for va in args.iter() {
        if i > 0 {
            // filter(skip the maincmd)
            all.push(va.trim());
        }
        if va.starts_with("-") {
            if va.starts_with("-h") {
                // println!("the variable boolean h was changed to true");

                h = true;
            }
            if va.starts_with("-e") {
                // println!("the variable boolean e was changed to true");

                e = true;
            }
            if va.starts_with("-l") {
                l = true;
            }
            if va.starts_with("-p") {
                p = true;
            }
            if va.starts_with("-r") {
                r = true;
            }
            option.push(va);
        } else {
            a.push(va);
        }

        i += 1;
    }
    let mut i: i32 = 0;
    if a.len() == 0 {
        read(true); //Which is the best, true or false?
    } else if maincmd.trim() == "exit" {
        // println!("exit if statement");
        exit_exec(args.len().try_into().unwrap(), h, a);
    } else if maincmd.trim() == "cd" {
        if a.len() >= 2 {
            let path: &str = a[1].trim();
            cd_exec(path, args.len().try_into().unwrap(), h, e);
        } else if a.len() == 1 {
            println!("Need path or option!");
            read(false);
        }
        
    } else if maincmd.trim() == "ls" {
        ls_exec(args.len().try_into().unwrap(), a, l);
    } else if maincmd.trim() == "pwd" {
        pwd_exec(args.len().try_into().unwrap(), h);
    } else if maincmd.trim() == "yes" {
        yes_exec(args.len().try_into().unwrap(), h, a);
    } else if maincmd.trim() == "true" {
        true_exec(args.len().try_into().unwrap(), h);
    } else if maincmd.trim() == "false" {
        false_exec(args.len().try_into().unwrap(), h);
    } else if maincmd.trim() == "mkdir" {
        mkdir_exec(args.len().try_into().unwrap(), h, p, a);
    } else if maincmd.trim() == "rm" {
        rm_exec(args.len().try_into().unwrap(), h, r, a);
    } else {
        elsecmd_exec(maincmd, all);
    }
}

fn exit_exec(l: i32, h: bool, a: Vec<&str>) {
    let success: bool = true;
    let alen: i32 = a.len().try_into().unwrap();
    if l == 1 && alen == 0 {
        let code: i32 = 0;
        std::process::exit(code);
    } else if l >= 2 {
        if h == true {
            println!("Usage:");
            println!(r##"exit -[option]"##);
            println!("exit [number](0, 1 and so on.)");
            println!("    exit the shell");
            println!("    -option:");
            println!("        -h  display this help");
            read(success);
        };
        if alen <= 3 {
            if h == false {
                let code: i32 = codestrparse(a[1]);
                std::process::exit(code);
            }
        }
    }
}
fn cd_exec(path: &str, l: i32, h: bool, e: bool) {
    let mut success: bool = true;
    if l == 1 {
    } else if l <= 3 {
        if h == false {
            let root: &Path = Path::new(path.trim());
            let _ = env::set_current_dir(&root);
            if e == true {
                if !env::set_current_dir(&root).is_ok() {
                    success = false;
                    eprintln!("the directory is not found!");
                }
            }
            // println!("Successfully changed working directory to {}!", root.display());
        } else {
            println!("Usage:");
            println!(r##"cd -[option] [path]"##);
            println!("    change directory");
            println!("    -option:");
            println!("        -e  If the directory is not found, the program exits with an error.")
        }
    } else {
        success = false;
        println!("error");
    }
    read(success)
}
fn ls_optformat(file_path: path::PathBuf, _is_dir: bool, meta: std::fs::Metadata, l: bool) {
    let mut lstr: String = String::new();
    let targetname = file_path.display();
    let r: bool = meta.permissions().readonly();
    if l == true {
        // if !is_dir {
        //     lstr = format!("-")
        // } else {
        //     lstr = format!("d")
        // }
        if cfg!(windows) {
            let permission_string = String::from(if meta.is_dir() { "d" } else { "-" });
            if r == true {
                lstr = format!("{}r-- ", permission_string);
            } else if r == false {
                lstr = format!("{}not-ronly", permission_string);
            }
            lstr = format!("{} {}", lstr, targetname);
        }
        #[cfg(target_os = "linux")]
            {
                let permission: fs::Permissions = meta.permissions();
                let mode = permission.mode();
                println!("unix: mdoe");

                let mut permission_string = String::from(if meta.is_dir() { "d" } else { "-" });

                for &shift in &[6, 3, 0] {
                    for &flag in &[4, 2, 1] {
                        if mode & (flag << shift) != 0 {
                            let ch = match flag {
                                4 => "r",
                                2 => "w",
                                1 => "x",
                                _ => unreachable!(),
                            };
                            permission_string.push_str(ch);
                        } else {
                            permission_string.push('-');
                        }
                    }
                }
                lstr = format!("{}{}", lstr, permission_string);
                lstr = format!("{} {}", lstr, targetname);
            }
        println!("{lstr}");
    }
}
fn ls_exec(l: i32, _a: Vec<&str>, lo: bool) {
    let _success: bool = true;
    let _readonly: bool = false;
    let mut target_path= "./".to_string();
    

    if _a.len() == 1 {
        target_path = format!("./");
    } else if _a.len() == 2 {
        let a = _a[1].trim().to_string();
        // let target_ = format!("./{}", a);
        let target_ = format!(r#"{}\"#, a.trim());
        target_path = target_;
        println!("{}", target_path);
        let target: path::PathBuf = path::PathBuf::from(&target_path);
        let files: fs::ReadDir = target.read_dir().expect("このパスは存在しません");
        if lo == false {
            for dir_entry in files {
                let file_path: path::PathBuf = dir_entry.unwrap().path();
                println!("{}", file_path.display());
            }
        }
    }
    let target: path::PathBuf = path::PathBuf::from(&target_path);
        let files: fs::ReadDir = target.read_dir().expect("このパスは存在しません");
    if l == 1 {
        for dir_entry in files {
            let file_path: path::PathBuf = dir_entry.unwrap().path();
            println!("{}", file_path.display());
        }
    } else if l >= 2 {
        for dir_entry in files {
            let file_path: path::PathBuf = dir_entry.unwrap().path();
            let isdir: bool = file_path.is_dir();
            let targetclone = file_path.clone();
            let meta: std::fs::Metadata = std::fs::metadata(file_path).unwrap();
            ls_optformat(targetclone, isdir, meta, lo);
        }
    }
    read(_success);
}
fn pwd_exec(l: i32, h: bool) {
    let home_directory = env::home_dir().unwrap();
    let current_directory = env::current_dir().unwrap();

    let current_directory_str = current_directory.to_string_lossy();
    let modified_path = current_directory_str.replace(&*home_directory.to_string_lossy(), "~");
    if h == false {
        println!("current dir: {}", modified_path);
    } else {
        println!("Usage:");
        println!("pwd -[option]");
        println!("    display the current dir");
        println!("        option:");
        println!("            -h display this help");
    }
    read(true);
}
fn yes_exec(l: i32, h: bool, a: Vec<&str>) {
    let mut success = true;
    if l == 1 {
        loop {
            println!("y");
        }
    } else if l == 2 {
        if h == false {
            loop {
                println!("{}", a[1].trim());
            }
        } else {
            println!("Usage:");
            println!("yes");
            println!("yes [string]");
            println!("yes -[option]");
            println!("    Loop the output of y/[string]");
            println!("        options:");
            println!("            -h display this help");
        }
    } else if l >= 3 {
        success = false;
    }
    read(success)
}
fn elsecmd_exec(maincmd: String, all: Vec<&str>) {
    let args: Vec<&str> = all.clone();
    let success: bool = true;
    Command::new(maincmd.trim())
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output();
}
fn true_exec(l: i32, h: bool) {
    let mut success = true;
    if l == 1 {
        std::process::exit(0);
    } else if l == 2 {
        if h == true {
            println!("Usage:");
            println!("true");
            println!("true -[option]");
            println!("    Always exits with exit code 0");
            println!("    option:");
            println!("        -h display this help");
        } else {
            success = false;
        }
    } else if l >= 3 {
        success = false;
    }
    read(success);
}
fn false_exec(l: i32, h: bool) {
    let mut success = true;
    if l == 1 {
        std::process::exit(1);
    } else if l == 2 {
        if h == true {
            println!("Usage:");
            println!("false");
            println!("false -[option]");
            println!("    Always exits with exit code 1");
            println!("    option:");
            println!("        -h display this help");
        } else {
            success = false;
        }
    } else if l >= 3 {
        success = false;
    }
    read(success);
}
fn mkdir_exec(l: i32, h: bool, p: bool, a: Vec<&str>) {
    let mut success = true;
    println!("mkdir exec!");
    if l == 2 {
        println!("l == 2");
        println!("a[1].trim() == {}", a[1].trim());
        match fs::create_dir(a[1].trim()) {
                Err(why) => println!("! {:?}", why.kind()),
                Ok(_) => {},
        }
    } else if l == 3 {
        println!("l == 3");
                    println!("a[1].trim() == {}", a[1].trim());
        if p == true {
            println!("p == true");
            fs::create_dir_all(a[1].trim()).unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            });
        } else {
            success = false;
        }
    } else {
        success = false;
    }
    read(success);
}
fn rmpathmake(a: String, r: bool) -> String {
    let mut m = String::new();
    if r == true {
        m = format!("{}/", m);
    }
    return m;
}
fn rm_exec(l: i32, h: bool, r: bool, a: Vec<&str>) {
    let mut success = true;
    if l == 2 {
        if r == false {
            println!("a[1].trim() == {}", a[1].trim());
            fs::remove_file(a[1].trim()).unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            });
        } else {
            success = false;
        }
    } else if l >= 3 {
        let mut modifiedpath = rmpathmake(a[1].trim().to_string(), r);
        if r == false {
            fs::remove_file(modifiedpath).unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            });
        } else {
            fs::remove_dir(a[1].trim()).unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            });
        }
    }
    read(success);
}
fn codestrparse(codestr: &str) -> i32 {
    let codei32: i32 = codestr.trim().parse::<i32>().unwrap();
    return codei32;
}