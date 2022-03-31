extern crate core;
use std::io::Write;
use std::fs::File;
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;
use hex;
use std::str;

fn Prompt (name: &str ) -> String {
    let mut line = String::new();
    println!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("error could not read line");
    return line.trim().to_string()
}

fn Write (line: &str)  {
    println!("{}", line);
    std::io::stdout().flush().unwrap()
}


fn main() {
    let mut entry = String::from("");
    let path = Path::new("diary.txt");
    let pwPath = Path::new("data.txt");
    let display = path.display();

    if pwPath.exists() == false {
        let password = Prompt("> Password");
        let mut pwFile = match File::create(pwPath) {
            Err(why) => panic!("couldn't create {}: {}", pwPath.display(), why),
            Ok(file) => file,
        };

        match pwFile.write_all(hex::encode(password).as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", pwPath.display(), why),
            Ok(_) => println!("successfully wrote to {}", pwPath.display()),
        }
    }

    if pwPath.exists() == true {
        let password = Prompt("> Password?");
        let setPw = fs::read_to_string(pwPath).expect("Something went wrong reading the file");
        let decodedHexPw = match hex::decode(setPw) {
            Err(why) => panic!("error {}", why),
            Ok(v) => v,
        };

        if std::str::from_utf8(&decodedHexPw).unwrap() != password {
            Write("wrong");
            return
        }
        Write("bienvinedos kyle")
    }

    loop {
        let input = Prompt("> ");
        if input == "exit" {
            break
        }
       entry = format!("{}\n{}", entry, input);
    }

    let exists = path.exists();
    if exists {
        let mut file = OpenOptions::new().write(true).append(true)
            .open(path)
            .unwrap();

        if let Err(_e) = writeln!(file, "{}" , hex::encode(entry)) {
            Write("Error.. exiting... ")
        }
        Write("done");
        return;
    }

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(hex::encode(entry).as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
