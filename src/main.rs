use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, Box, Orientation, Label};
use sysinfo::{System, SystemExt, ComponentExt};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let application = Application::new(
        Some("com.example.fan_control"),
        Default::default(),
    );

    application.connect_activate(build_ui);

    application.run();
}

fn build_ui(application: &gtk4::Application) {
    let window = ApplicationWindow::new(application);
    window.set_title(Some("Fan Control"));
    window.set_default_size(350, 70);

    let vbox = Box::new(Orientation::Vertical, 5);
    let temp_label = Label::new(Some("Temp: "));
    let update_button = Button::with_label("Update");
    let fan_button = Button::with_label("Set Fan Speed");

    vbox.append(&temp_label);
    vbox.append(&update_button);
    vbox.append(&fan_button);

    window.set_child(Some(&vbox));

    let system = Rc::new(RefCell::new(System::new_all()));

    {
        let system = system.clone();
        update_button.connect_clicked(move |_| {
            let mut system = system.borrow_mut();
            system.refresh_components();
            let temp = system.components()[0].temperature();
            temp_label.set_text(&format!("Temp: {:.2}°C", temp));
        });
    }

    fan_button.connect_clicked(move |_| {
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg("echo 255 > /sys/class/hwmon/hwmon0/pwm1") // Exemplo de comando, ajustar conforme necessário
            .output()
            .expect("failed to execute process");

        if output.status.success() {
            println!("Fan speed set successfully");
        } else {
            eprintln!("Error setting fan speed: {}", String::from_utf8_lossy(&output.stderr));
        }
    });

    window.present();
}
