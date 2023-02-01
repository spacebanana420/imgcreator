use std::fs::File;
use std::io; use std::io::Write;

fn main() {
    let name = answer("Input file name");
    let size = answer_to_usize("Input the disk image size (in GB)");
    if size > 30 {
        let confirmation = answer("The requested size is over 30GB, do you want to proceed? (y/n)");
        if confirmation != "y" && confirmation != "yes" {
            return;
        }
    }
    create_disk_img(size, &name);
}

fn create_disk_img (img_size:usize, name:&str) {
    let mut filename = name.to_string();
    if filename.contains(".img") == false {
        filename += ".img";
    }
    let img = File::create(filename).unwrap();
    let img_size_bytes:usize = img_size *  1000000000;
    //let v = vec![0; img_size_bytes];
    //img.write(&v);
    img.set_len(img_size_bytes.try_into().unwrap());
}


fn answer(message:&str) -> String {
    println!("{}", &message);
    let mut userinput = String::new();
    io::stdin().read_line(&mut userinput).expect("Failed to read user input");
    return userinput;
}

fn answer_to_usize(message:&str) -> usize {
    println!("{}", &message);
    let mut userinput = String::new();
    io::stdin().read_line(&mut userinput).expect("Failed to read user input");
    let userinput:usize = userinput.trim().parse().expect("Needs to be a number!");
    return userinput;
}
/*
fn answer_to_u8() -> u8 {
    let mut userinput = String::new();
    io::stdin().read_line(&mut userinput).expect("Failed to read user input");
    let userinput:u8 = userinput.trim().parse().expect("Needs to be a number!");
    return userinput;
}*/
