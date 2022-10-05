#![allow(dead_code, unused)]

use std::env;
use std::fs::File;
use std::io;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::io::ErrorKind;
use std::io::Write;
use chrono::{Datelike, Timelike, Utc};
use rand::Rng;

pub fn main(){
    let args : Vec<String> = env::args().collect();

    //Three ways to do the same thing
    let username = if args.len() == 2 {
        args[1].clone()
    }
    else {
        String::from("Gianluca (Default user)")
    };

    let username = match args.get(1) {
        Some(i) => { 
            args[1].clone() 
        }
        None => String::from("Gianluca (Default user)")
    };

    let username = if let Some(i) = args.get(1){
        args[1].clone()
    }
    else{
        String::from("Gianluca (Default user)")
    };

    println!("Ciao {:?}", username);

    loop {
        let choice = mainmenu();
        println!("you selected: {}", choice);
        match choice{
            0 => break,
            1 => noteshandler(&username[..]),
            2 => magicnumber(),
            _ => println!("boooo"),
        }
    }
}

fn mainmenu() -> i32{
    let mut usr_in : String = String::new();

    //print all the functions in the main menu and return a selected value
    println!("\n---MAIN MENU---");
    println!("1) Notes menu;");
    println!("2) Magic number;");
    println!("0) exit");
    io::stdin().read_line(&mut usr_in).expect("You did not enter a correct string");

    match usr_in.is_empty() {
        true => {
            println!("you inserted an empty string, retry!");
            return mainmenu();
        },
        false => {
            //when you get a string from std_in it contains new line trail
            //you have to trim it with trim_end!!!!!!
            // let a = usr_in.trim_end().parse::<i32>().unwrap();

            match usr_in.trim_end().parse::<i32>() {
                Ok(n) =>{ return n;},
                Err(e) => { 
                    println!("couldn't parse the string {}", e);
                    return mainmenu();
                }
            }
        },
    }
}

fn magicnumber(){
    let mut rng = rand::thread_rng();
    println!("Your magic number is: {}", rng.gen_range(0..1000)%100)
}

fn noteshandler(user: &str) {
    loop {
        match notesmenu(){
            0 => break,
            1 => showallnotes(),
            2 => newnote(user),
            _ => println!("booooooo"),
        }
    }
}

fn notesmenu() -> i32 {
    let mut usr_in: String = String::new();
    println!("\n---NOTES MENU---");
    println!("1) Show all notes");
    println!("2) Publish new note");
    println!("0) exit");
    io::stdin().read_line(&mut usr_in).expect("couldn't read!");
    //let selection = usr_in.trim_end().parse::<i32>().expect("couldn't parse");

    if let Ok(n) = usr_in.trim_end().parse::<i32>() {
        return n;
    }
    else{
        println!("parsing error, try again");
        return notesmenu();
    }

}

fn showallnotes() {
    //prints notes one by one
    const NOTESPATH: &str = "../notes/notes.txt";

    let mut f = match fs::File::open(NOTESPATH) {
        Ok(file) => file,
        Err(e) => match e.kind(){
            ErrorKind::NotFound => fs::File::create(NOTESPATH).unwrap(),
            _ => panic!("problems opening the file"),
        },
    };
    let i: i32 = 0; 
    let reader = BufReader::new(f);
    for line in reader.lines(){
        if let Ok(note) = line{
            
            //handle the csv
            let mut split = note.split(";");

            //get author
            let author = if let Some(aut) = split.next(){
                aut
            }
            else{
                panic!("wrong format");
            };
            // equal to: let author = split.next().unwrap();

            //get text
            let text = if let Some(t) = split.next(){
                t
            }
            else{
                panic!("wrong format");
            };
            // equal to: let text = split.next().unwrap();

            let date = if let Some(t) = split.next(){
                t
            }
            else{
                panic!("wrong format");
            };

            println!("{} published: {}. ({})", author, text, date);

        }
        else{
            break;
        }
    }
 //files are closed automatically when they go out of scope
}

fn newnote(author: &str) {
    const NOTESPATH: &str = "../notes/notes.txt";
    // println!("{}", author);
    //gets current date and time
    //gets note description
    println!("Insert note text and enter!");
    println!("Your note cannot contain semicolons -> ;");

    let mut file = fs::OpenOptions::new().write(true).append(true).open(NOTESPATH);
    let mut file = match file{
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => fs::File::create(NOTESPATH).unwrap(),
            _ => panic!("cannot open file"),
        },
    };

    let mut usr_in = String::new();
    loop{
        io::stdin().read_line(&mut usr_in).expect("couldn't read input");

        match usr_in.find(';') {
            None => break,
            Some(x) =>{
                println!("Your note cannot contain semicolons -> ;");
                usr_in.clear();
            },
        }
    }
    
    let mut app = String::new();
    app.push_str(author);
    app.push(';');
    app.push_str(&usr_in[..].trim_end());
    app.push(';');
    let now = Utc::now();
    let mut date = now.format("%a %b %e %T %Y").to_string();
    app.push_str(&date[..]);
    app.push('\n');
    write!(file, "{}", app);

    println!("Note added successfully!");
}
