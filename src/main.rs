use gethostname;
use std::process::Command;
use sys_info;
fn has(pkg_manager: &str) -> bool {
    return Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {}", pkg_manager))
        .output()
        .unwrap()
        .stdout
        .len()
        > 0;
}
fn get_kernel() -> Vec<u8> {
    let kernel_version = Command::new("sh")
        .arg("-c")
        .arg("uname -r")
        .output()
        .unwrap();
    return kernel_version.stdout;
}
fn get_packages() -> Vec<u8> {
    if sys_info::os_type().unwrap() == "Linux" {
        /*
            2 ways to detect packages, lookup, and rety.
            #1.
                Lookup
                    Looks up the distrobution name and sees if the Package manager is known
            #2.
                Retry
                    executes a bunch of commands know to return the amount of packages,
                    and hope that one of them returns a valid value.
        */
        if has("dpkg") {
            let lines = Command::new("sh")
                .arg("-c")
                .arg("dpkg --list | wc --lines")
                .output();
            return lines.unwrap().stdout;
        }
if has("apk"){
let lines = Command::new("sh")
.arg("-c")
.arg("apk info | wc -l")
.output();
return lines.unwrap().stdout; 
}
    }
    return vec![255];
    // return "Could not detect Package manager";
}
fn get_ascii() -> &'static str {
    if sys_info::linux_os_release().unwrap().id.unwrap() == "ubuntu" {
        return "
        ---(_)   
     _/  ---  \\   
     (_) |   |     
      \\  --- _/   
        ---(_)  
                 ";
    } else if sys_info::linux_os_release().unwrap().id.unwrap() == "alpine" {
        return "
           /\\ /\\
        // \\  \\
       //   \\  \\
      ///    \\  \\
      //      \\  \\
               \\
               ";
    }
    return "";
}
fn main() {
    let ascii: &str = get_ascii();
    let ascii_h: usize = ascii.as_bytes().iter().filter(|&&c| c == b'\n').count() + 1;
    let ascii_w = ascii
        .lines()
        .max_by(|x, y| {
            //https://www.reddit.com/r/rust/comments/2q4kq5/help_rust_program_to_find_the_longest_line_in_a/
            return x.len().cmp(&y.len());
        })
        .unwrap()
        .len();
    let padding = format!("\x1B[{}C", ascii_w);
    println!("{}", ascii);
    println!("\x1B[{}A", ascii_h);
    //hostname
    println!(
        "{} {}@{}",
        padding,
        "accusitive",
        gethostname::gethostname().to_str().unwrap()
    );
    //os
    println!(
        "{} os {}",
        padding,
        sys_info::linux_os_release().unwrap().pretty_name.unwrap()
    );
    print!(
        "{} pkgs {}",
        padding,
        std::str::from_utf8(&get_packages()).unwrap()
    ); // I think that it has a trailing newline, so its more elegant just to use print!.
    print!(
        "{} kernel {}",
        padding,
        std::str::from_utf8(&get_kernel()).unwrap()
    );
    let used = sys_info::mem_info().unwrap().total
        - sys_info::mem_info().unwrap().free
        - sys_info::mem_info().unwrap().cached;
    println!(
        "{} memory {}/{}",
        padding,
        used / 1024,
        sys_info::mem_info().unwrap().total / 1024
    );
    // println!("{} {}@{}", padding, env::consts::OS);

    println!("\n\n");
    #[cfg(debug_assertions)]
    println!(
        "DEBUG: {}",
        sys_info::linux_os_release().unwrap().id.unwrap()
    );
}
