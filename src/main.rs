extern crate ncurses;

//Standard libary stuff
use std::char;       //Used to ensure chars are chars
use std::env;        //Used to collect env variables
use std::path::Path; //File handling
use std::fs;         //File handling
use std::io::Read;   //File handling

//TUI library
use ncurses::*;

static QUIT: i32 = 113;

//Start up ncurses and set all the settings I want
fn ncurses_init() {
    //Start ncurses
    initscr();

    //Allow an extended keyboard
    keypad(stdscr(), true);

    //When I type, this will prevent the
    //terminal from displaying that
    noecho();

    //Update the screen
    refresh();
}

//Clear screen and display welcome message again
fn reset() {

    clear();
    
    //Welcome message 
    addstr("Welcome to Papyrus!\n");

    refresh();
}

fn file_load(fp: &str) -> Result<String, std::io::Error>{
    let mut reader = fs::File::open(Path::new(&fp))
        .ok()
        .expect("Unable to open file");

    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() {

    //Collect CLI args
    let args: Vec<_> = env::args().collect();

    //Must be exactly 2 (program name, file name)
    //Later on, users will be able to go to a
    //directory view by entering no args
    if args.len() != 2 {
        println!("ARGUMENT ERROR:");
        println!("\tUsage:   papyrus <file>");
        println!("\tExample: papyrus data.txt");
        panic!("Exiting due to argument error");
    }

    let buffer = if let Ok(x) = file_load(&args[1]) {
        x
    } else {
        "Error loading contents...".to_string()
    };

    ncurses_init();
    // Get the screen bounds as well as cursors position
    let mut max_x = 0;
    let mut max_y = 0;
    let mut cur_x = 0;
    let mut cur_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    //Print the contents of the file to the screen
    for ch in buffer.chars() {
        getyx(stdscr(), &mut cur_y, &mut cur_x);

        if cur_y >= (max_y-1) {
            addstr("Press a key to continue");
            getch();
            clear();
            mv(0, 0);
        }

        addch(ch as chtype);
    }
    refresh();
   
    //Wait for a button press to end
    addstr("At the end of the file, press \'q\' to exit");
    let mut ch = getch();
    while ch != QUIT {
        ch = getch();
    }
    endwin();
}
