extern crate ncurses;

use std::char;
use ncurses::*;

static QUIT: i32 = 113;

//Start up ncurses and set all the settings I want
fn init() {
    //Start ncurses
    initscr();

    //Allow an extended keyboard
    keypad(stdscr(), true);

    //When I type, this will prevent the
    //terminal from displaying that
    noecho();

    //Welcome message 
    addstr("Welcome to Papyrus!\n");
    
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

fn main() {

    init();

    loop {

        //Get a character
        let ch = getch();
        reset();

        //If it is not the quit condition
        if ch != QUIT {
            
            //Set it to bold and blink
            attron(A_BOLD() | A_BLINK());
            addstr(
                format!("{}", char::from_u32(ch as u32)
                    .expect("Invalid char"))
                    .as_ref()
            );
            //Turn off bold and blink
            attroff(A_BOLD() | A_BLINK());
            addstr(" pressed\n");

        //Else quit
        } else {
            endwin();
            return;
        }
        //Draw screen
        refresh();
    }
}
