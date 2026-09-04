mod control;

use control::{Arm, Joint};
use std::time::Duration;
use std::thread;

fn main() {
    let port = serialport::new("/dev/ttyUSB0", 9600)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("Erro ao abrir porta");

    // espera o ESP32 reiniciar
    thread::sleep(Duration::from_millis(3000));

    let mut arm = Arm::new(port);

    arm.execute_path_joints(vec![
        vec![
            (Joint::Base, 30),
            (Joint::Shoulder, 40),
            (Joint::Elbow, 50),
        ],
        vec![
            (Joint::Base, 90),
            (Joint::Shoulder, 80),
            (Joint::Elbow, 120),
        ],
    ]);
}