// Copyright (C) 2018 David A Holmes Jr
//
// This file is part of ToyCalc.
//
// ToyCalc is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// ToyCalc is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with ToyCalcCpp.  If not, see <http://www.gnu.org/licenses/>.

extern crate toycalc;
extern crate gtk;
extern crate gio;

use toycalc::*;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;
use std::rc::Rc;
use std::cell::RefCell;

// See http://gtk-rs.org/tuto/closures
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

fn configure_button(button: &gtk::Button, label: &str) {
    button.set_label(""); // Create the child label widget so we can get it

    match button.get_child() {
        Some(ref child) => match child.clone().downcast::<gtk::Label>() {
            Ok(ref button_label) =>
                button_label.set_markup(&*format!("<span font='24'>{}</span>", label)),
            Err(_) => panic!("gtk::Button child wasn't a gtk::Label (should never happen)"),
        },
        None => panic!("gtk::Button didn't have a child (should never happen)"),
    }

    // GTK lays out widgets according to their expand and size request
    // properties. I'm overriding the defaults here to cause them to expand
    // and to prevent them from being too small.
    button.set_hexpand(true);
    button.set_vexpand(true);
    button.set_size_request(50, 50);
}

fn new_digit_button(calc: &mut Rc<RefCell<Calculator>>, digit: i8) -> gtk::Button {
    let button = gtk::Button::new();
    configure_button(&button, &digit.to_string());
    button.connect_clicked(clone!(calc => move |_| { calc.borrow_mut().press_digit(digit)}));
    button
}

fn new_op_button(calc: &mut Rc<RefCell<Calculator>>, op: Operation) -> gtk::Button {
    let button = gtk::Button::new();
    configure_button(&button, op.to_str());
    button.connect_clicked(clone!(calc => move |_| { calc.borrow_mut().press_operation(op)}));
    button
}

fn new_eq_button(calc: &mut Rc<RefCell<Calculator>>) -> gtk::Button {
    let button = gtk::Button::new();
    configure_button(&button, "=");
    button.connect_clicked(clone!(calc => move |_| { calc.borrow_mut().press_equals()}));
    button
}

fn create_window(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("gtk-rs Calculator");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.connect_delete_event(clone!(window => move |_, _| {
        window.destroy();
        Inhibit(false)
    }));

    let mut calc = Rc::new(RefCell::new(Calculator::new()));

    let main_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    window.add(&main_box);

    let display_label = gtk::Label::new("");
    display_label.set_markup("<span font='32'>0</span>");
    display_label.set_halign(gtk::Align::End);
    main_box.add(&display_label);
    calc.borrow_mut().add_display_listener(Box::new(
        move |str| display_label.set_markup(&*format!("<span font='32'>{}</span>", str))));

    let button_grid = gtk::Grid::new();
    main_box.add(&button_grid);

    let mut digit_buttons: Vec<gtk::Button> = vec![];
    for i in 0..10 {
        digit_buttons.push(new_digit_button(&mut calc, i as i8));
    }

    button_grid.attach(&digit_buttons[0], 0, 3, 2, 1);
    button_grid.attach(&digit_buttons[1], 0, 2, 1, 1);
    button_grid.attach(&digit_buttons[2], 1, 2, 1, 1);
    button_grid.attach(&digit_buttons[3], 2, 2, 1, 1);
    button_grid.attach(&digit_buttons[4], 0, 1, 1, 1);
    button_grid.attach(&digit_buttons[5], 1, 1, 1, 1);
    button_grid.attach(&digit_buttons[6], 2, 1, 1, 1);
    button_grid.attach(&digit_buttons[7], 0, 0, 1, 1);
    button_grid.attach(&digit_buttons[8], 1, 0, 1, 1);
    button_grid.attach(&digit_buttons[9], 2, 0, 1, 1);

    let add_button = new_op_button(&mut calc, Operation::Add);
    let sub_button = new_op_button(&mut calc, Operation::Subtract);
    let mul_button = new_op_button(&mut calc, Operation::Multiply);
    let div_button = new_op_button(&mut calc, Operation::Divide);
    let eq_button = new_eq_button(&mut calc);

    button_grid.attach(&add_button, 3, 0, 1, 1);
    button_grid.attach(&sub_button, 3, 1, 1, 1);
    button_grid.attach(&mul_button, 3, 2, 1, 1);
    button_grid.attach(&div_button, 3, 3, 1, 1);
    button_grid.attach(&eq_button, 2, 3, 1, 1);

    window.show_all();
}

fn main() {
    let application = gtk::Application::new("us.dholmes.rust_calculator",
                                            gio::ApplicationFlags::empty())
                                       .expect("Initialization failed...");

    application.connect_activate(|app| {
        create_window(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
