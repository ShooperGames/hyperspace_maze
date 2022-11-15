//use mazegen::GameState;
use gtk::prelude::*;
use gtk::{gio, Application, ApplicationWindow, Button, Grid, Label};
use gtk::glib::object::Cast;
use std::{fs, io, thread};
use std::time::Duration;

#[macro_use]
extern crate savefile_derive;

const APP_ID: &str = "org.gtk-rs.HyperspaceNav";

pub mod mazegen;

fn main() {
    //println!("Hello, world!");
    let file = fs::OpenOptions::new().write(true).create_new(true).open("data");
    match file {
        Ok(_) => {if mazegen::make_maze(1) {
                println!("Maze Made");
            } else {
                println!("Something went wrong");
            }},
        Err(error) => match error.kind() {
            io::ErrorKind::AlreadyExists => {},
            _ => panic!("Problem with lock file error {:?}", error),
        },
    }
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    
    app.set_accels_for_action("win.restart", &["r"]);
    app.set_accels_for_action("win.forward", &["w", "Up"]);
    app.set_accels_for_action("win.turn_left", &["a", "Left"]);
    app.set_accels_for_action("win.turn_right", &["d", "Right"]);
    app.set_accels_for_action("win.turn_back", &["s", "Down"]);

    app.run();
    //println!("End");
}

fn forward_but_click(my_button: &Button) {
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
        let first_grid = my_button.parent().unwrap()
                                    .parent().unwrap()
                                    .parent().unwrap();
        let mut child_widget = first_grid.first_child().unwrap();
        let state_label = child_widget.downcast_ref::<gtk::Label>().unwrap();
        update_state_label(&state_label);
        let label_grid = first_grid.last_child().unwrap();
        child_widget = label_grid.first_child().unwrap()
                                    .next_sibling().unwrap();
        let mut dimension_counter = 0;
        while dimension_counter < 4 {
            child_widget = child_widget.next_sibling().unwrap();
            let child_label = child_widget.downcast_ref::<gtk::Label>().unwrap();
            update_label((true, dimension_counter), &child_label);
            child_widget = child_widget.next_sibling().unwrap();
            let child_label = child_widget.downcast_ref::<gtk::Label>().unwrap();
            update_label((false, dimension_counter), &child_label);
            dimension_counter += 1;
        }
        //println!("Moved");
    } else {
        //println!("Cannot Move Forward");
    }
    fs::remove_file("data.lock").unwrap();
}

fn restart_game(my_button: &Button) {
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
    if mazegen::make_maze(1) {
        let first_grid = my_button.parent().unwrap()
                                    .parent().unwrap()
                                    .parent().unwrap();
        let mut child_widget = first_grid.first_child().unwrap();
        let state_label = child_widget.downcast_ref::<gtk::Label>().unwrap();
        update_state_label(&state_label);
        let label_grid = first_grid.last_child().unwrap();
        child_widget = label_grid.first_child().unwrap()
                                    .next_sibling().unwrap();
        let mut dimension_counter = 0;
        while dimension_counter < 4 {
            child_widget = child_widget.next_sibling().unwrap();
            let child_label = child_widget.downcast_ref::<gtk::Label>().unwrap();
            update_label((true, dimension_counter), &child_label);
            child_widget = child_widget.next_sibling().unwrap();
            let child_label = child_widget.downcast_ref::<gtk::Label>().unwrap();
            update_label((false, dimension_counter), &child_label);
            dimension_counter += 1;
        }
    } else {
        println!("Something went wrong");
    }
    fs::remove_file("data.lock").unwrap();
}

fn update_label(direction: (bool, u32), text: &Label) {
    let mut result = String::from(".");
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
        result += "+.\n__\n";
    } else {
        result += "-.\n__\n";
    }
    let mut notes = mazegen::hall_dist(direction);
    while notes.len() > 0 {
        result += &("|".to_owned() + &notes.pop().unwrap() + "|\n");
    }
    text.set_label(&result);
    /*if distance >= 0 {
        for _ in 0..distance {
            result += "|  |\n";
        }
        text.set_label(&result);
    }*/
}

fn update_state_label(text: &Label) {
    let state: (u32, u32, (bool, u32), String) = mazegen::get_loc();
    let mut position = state.0;
    if position == (1 << 12) - 1 {
        text.set_label("You Win!");
        return;
    }
    let mut result = String::from("Position: (");
    let direction = state.2;
    result += &(position % 8).to_string();
    position -= position % 8;
    position = position >> 3;
    let mut counter = 1;
    while counter < state.1 {
        result += ", ";
        result += &(position % 8).to_string();
        position -= position % 8;
        position = position >> 3;
        counter += 1;
    }
    result += ") Facing: ";
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
        result += "+ note: ";
    } else {
        result += "- note: ";
    }
    result += &state.3;
    text.set_label(&result);
}

fn left_but_click(my_button: &Button) {
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
    if mazegen::turn_cell(-1) {
        // Update Labels
        let first_grid = my_button.parent().unwrap()
                                    .parent().unwrap()
                                    .parent().unwrap();
        let mut child_widget = first_grid.first_child().unwrap();
        let state_label = child_widget.downcast_ref::<gtk::Label>().unwrap();
        update_state_label(&state_label);
        let label_grid = first_grid.last_child().unwrap();
        child_widget = label_grid.first_child().unwrap()
                                    .next_sibling().unwrap();
        let mut dimension_counter = 0;
        while dimension_counter < 4 {
            child_widget = child_widget.next_sibling().unwrap();
            let child_label = child_widget.downcast_ref::<gtk::Label>().unwrap();
            update_label((true, dimension_counter), &child_label);
            child_widget = child_widget.next_sibling().unwrap();
            let child_label = child_widget.downcast_ref::<gtk::Label>().unwrap();
            update_label((false, dimension_counter), &child_label);
            dimension_counter += 1;
        }
        //println!("Turned Left");
    } else {
        println!("Error turning");
    }
    fs::remove_file("data.lock").unwrap();
}

fn right_but_click(my_button: &Button) {
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
    if mazegen::turn_cell(1) {
        // Update Labels
        let first_grid = my_button.parent().unwrap()
                                    .parent().unwrap()
                                    .parent().unwrap();
        let mut child_widget = first_grid.first_child().unwrap();
        let state_label = child_widget.downcast_ref::<gtk::Label>().unwrap();
        update_state_label(&state_label);
        let label_grid = first_grid.last_child().unwrap();
        child_widget = label_grid.first_child().unwrap()
                                    .next_sibling().unwrap();
        let mut dimension_counter = 0;
        while dimension_counter < 4 {
            child_widget = child_widget.next_sibling().unwrap();
            let child_label = child_widget.downcast_ref::<gtk::Label>().unwrap();
            update_label((true, dimension_counter), &child_label);
            child_widget = child_widget.next_sibling().unwrap();
            let child_label = child_widget.downcast_ref::<gtk::Label>().unwrap();
            update_label((false, dimension_counter), &child_label);
            dimension_counter += 1;
        }
        //println!("Turned Right");
    } else {
        println!("Error turning");
    }
    fs::remove_file("data.lock").unwrap();
}

fn back_but_click(my_button: &Button) {
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
    if mazegen::turn_cell(0) {
        // Update Labels
        let first_grid = my_button.parent().unwrap()
                                    .parent().unwrap()
                                    .parent().unwrap();
        let mut child_widget = first_grid.first_child().unwrap();
        let state_label = child_widget.downcast_ref::<gtk::Label>().unwrap();
        update_state_label(&state_label);
        let label_grid = first_grid.last_child().unwrap();
        child_widget = label_grid.first_child().unwrap()
                                    .next_sibling().unwrap();
        let mut dimension_counter = 0;
        while dimension_counter < 4 {
            child_widget = child_widget.next_sibling().unwrap();
            let child_label = child_widget.downcast_ref::<gtk::Label>().unwrap();
            update_label((true, dimension_counter), &child_label);
            child_widget = child_widget.next_sibling().unwrap();
            let child_label = child_widget.downcast_ref::<gtk::Label>().unwrap();
            update_label((false, dimension_counter), &child_label);
            dimension_counter += 1;
        }
        //println!("Turned Around");
    } else {
        println!("Error turning");
    }
    fs::remove_file("data.lock").unwrap();
}

fn make_note(input_text: &gtk::Entry) {
    if !mazegen::store_note(input_text.buffer().text().clone()) {
        println!("Error Saving Note");
    } //else {
        //println!("Saved");
    //}
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
    left_but.connect_clicked(move |left_but| {left_but_click(left_but)});
    let right_but = Button::builder()
        .label("Next Direction")
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .margin_bottom(12)
        .build();
    right_but.connect_clicked(move |right_but| {right_but_click(right_but)});
    let back_but = Button::builder()
        .label("Rev Direction")
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .margin_bottom(12)
        .build();
    back_but.connect_clicked(move |back_but| {back_but_click(back_but)});
    let text_input = gtk::Entry::new();
    text_input.connect_activate(move |text_input| {make_note(text_input)});

    let button_grid = Grid::builder().build();

    button_grid.attach(&forward_but, 1, 1, 3, 1);
    button_grid.attach(&left_but, 1, 2, 1, 1);
    button_grid.attach(&right_but, 2, 2, 1, 1);
    button_grid.attach(&back_but, 3, 2, 1, 1);
    button_grid.attach(&text_input, 1, 3, 3, 1);

    let text_ph = Label::new(Some("Welcome to hyperspace\nNavigate using the buttons or arrow keys\nPress 'R' to restart\nMake notes with the input at the bottom"));

    let start_pane = Grid::builder().build();
    start_pane.set_row_homogeneous(true);
    start_pane.attach(&text_ph, 1, 1, 1, 1);
    start_pane.attach(&button_grid, 1, 2, 1, 1);

    let text_space1 = Label::new(Some("|\n|\n|\n|\n|\n|\n|\n|\n|\n|"));
    let text_space2 = Label::new(Some("|\n|\n|\n|\n|\n|\n|\n|\n|\n|"));
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
    update_label((true, 0), &text_phx1);
    update_label((false, 0), &text_phx2);
    update_label((true, 1), &text_phy1);
    update_label((false, 1), &text_phy2);
    update_label((true, 2), &text_phz1);
    update_label((false, 2), &text_phz2);
    update_label((true, 3), &text_pha1);
    update_label((false, 3), &text_pha2);
    //text_phx1.set_widget_name("x1");
    //text_phx2.set_widget_name("x2");
    //text_phy1.set_widget_name("y1");
    //text_phy2.set_widget_name("y2");
    //text_phz1.set_widget_name("z1");
    //text_phz2.set_widget_name("z2");
    //text_pha1.set_widget_name("a1");
    //text_pha2.set_widget_name("a2");

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
    end_pane.set_widget_name("end_pane");

    let state_label = Label::new(Some("Position: (0, 0, 0, 0) Facing: x +"));
    update_state_label(&state_label);
    state_label.set_margin_top(10);
    state_label.set_margin_bottom(10);
    state_label.set_widget_name("state");
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

    let restart_act = gio::SimpleAction::new("restart", None);
    let but_clone = forward_but.clone();
    restart_act.connect_activate(move |_,_| {restart_game(&but_clone)});
    window.add_action(&restart_act);
    let forward_act = gio::SimpleAction::new("forward", None);
    forward_act.connect_activate(move |_,_| {forward_but_click(&forward_but)});
    window.add_action(&forward_act);
    let left_act = gio::SimpleAction::new("turn_left", None);
    left_act.connect_activate(move |_,_| {left_but_click(&left_but)});
    window.add_action(&left_act);
    let right_act = gio::SimpleAction::new("turn_right", None);
    right_act.connect_activate(move |_,_| {right_but_click(&right_but)});
    window.add_action(&right_act);
    let back_act = gio::SimpleAction::new("turn_back", None);
    back_act.connect_activate(move |_,_| {back_but_click(&back_but)});
    window.add_action(&back_act);

    window.present();
}