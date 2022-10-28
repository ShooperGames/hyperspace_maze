use mazegen::GameState;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Grid, Label};
use std::{fs, io, thread};
use std::time::Duration;

#[macro_use]
extern crate savefile_derive;

const APP_ID: &str = "org.gtk-rs.HyperspaceNav";

pub mod mazegen;

fn main() {
    println!("Hello, world!");
    if mazegen::make_maze(1) {
        println!("Maze Made");
    } else {
        println!("Something went wrong");
    }
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
    println!("End");
}

fn forward_but_click(forward_but: Button) {
    //Lock File Loop
    let mut counter = 0;
    while counter < 10 {
        counter += 1;
        let file = fs::OpenOptions::new().write(true).create_new(true).open("data.lock");
        match file {
            Ok(_) => break,
            Err(error) => match error.kind() {
                io::ErrorKind::AlreadyExists => println!("Game Locked"),
                _ => panic!("Problem with lock file error {:?}", error),
            },
        }
        thread::sleep(Duration::from_secs(1));
    }
    if counter > 9 {
        return;
    }
    if mazegen::move_cell() {
        // Update labels
        println!("Moved");
    } else {
        println!("Cannot Move Forward");
    }
    fs::remove_file("data.lock").unwrap();
}

fn update_label(direction: (bool, u32), text: Label) {
    let mut result = String::from(".  ");
    result += match direction.1 {
        0 => "x ",
        1 => "y ",
        2 => "z ",
        3 => "a ",
        _ => {
            println!("Something went wrong: {:?}", direction);
            return;
        }
    };
    if direction.0 {
        result += "+  .\n";
    } else {
        result += "-  .\n";
    }
    // Label Update
}

fn left_but_click () {
    //Testing Button Edit
}

fn build_ui(app: &Application) {
    let forward_but = Button::builder()
        .label("Move Forward")
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .margin_bottom(12)
        .build();
    forward_but.connect_clicked(move |forward_but| {forward_but_click(forward_but)});
    let left_but = Button::builder()
        .label("Prev Direction")
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .margin_bottom(12)
        .build();
    let right_but = Button::builder()
        .label("Next Direction")
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .margin_bottom(12)
        .build();
    let back_but = Button::builder()
        .label("Rev Direction")
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .margin_bottom(12)
        .build();

    let button_grid = Grid::builder().build();

    button_grid.attach(&forward_but, 1, 1, 3, 1);
    button_grid.attach(&left_but, 1, 2, 1, 1);
    button_grid.attach(&right_but, 2, 2, 1, 1);
    button_grid.attach(&back_but, 3, 2, 1, 1);

    let text_ph = Label::new(Some("Text Here"));

    let start_pane = Grid::builder().build();
    start_pane.set_row_homogeneous(true);
    start_pane.attach(&text_ph, 1, 1, 1, 1);
    start_pane.attach(&button_grid, 1, 2, 1, 1);

    let text_space1 = Label::new(Some("|\n|\n|\n|\n|\n|\n|\n|"));
    let text_space2 = Label::new(Some("|\n|\n|\n|\n|\n|\n|\n|"));
    let text_phx1 = Label::new(Some(".  x +  ."));
    let text_phx2 = Label::new(Some(".  x -  ."));
    let text_phy1 = Label::new(Some(".       ."));
    let text_phy2 = Label::new(Some(".       ."));
    let text_phz1 = Label::new(Some(".       ."));
    let text_phz2 = Label::new(Some(".       ."));
    let text_pha1 = Label::new(Some(".       ."));
    text_pha1.set_margin_end(20);
    let text_pha2 = Label::new(Some(".       ."));
    text_pha2.set_margin_end(20);

    let end_pane = Grid::builder().build();
    end_pane.set_row_homogeneous(true);
    end_pane.set_column_spacing(20);
    end_pane.attach(&text_space1, 1, 1, 1, 1);
    end_pane.attach(&text_space2, 1, 2, 1, 1);
    end_pane.attach(&text_phx1, 2, 1, 1, 1);
    end_pane.attach(&text_phx2, 2, 2, 1, 1);
    end_pane.attach(&text_phy1, 3, 1, 1, 1);
    end_pane.attach(&text_phy2, 3, 2, 1, 1);
    end_pane.attach(&text_phz1, 4, 1, 1, 1);
    end_pane.attach(&text_phz2, 4, 2, 1, 1);
    end_pane.attach(&text_pha1, 5, 1, 1, 1);
    end_pane.attach(&text_pha2, 5, 2, 1, 1);

    let state_label = Label::new(Some("Position: (0, 0, 0, 0) Facing: x +"));
    state_label.set_margin_top(10);
    state_label.set_margin_bottom(10);
    start_pane.set_margin_bottom(10);
    end_pane.set_margin_bottom(10);

    let outer_pane = Grid::builder().build();
    outer_pane.attach(&state_label, 1, 1, 2, 1);
    outer_pane.attach(&start_pane, 1, 2, 1, 1);
    outer_pane.attach(&end_pane, 2, 2, 1, 1);
    
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hyperspace Navigation Training")
        .child(&outer_pane)
        .build();
    window.present();
}