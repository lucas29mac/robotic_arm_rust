use std::io::{Write, Read};
use std::time::Duration;
use std::thread;

#[derive(Copy, Clone)]
pub enum Joint {
    Base = 0,
    Shoulder = 1,
    Elbow = 2,
    WristPitch = 3,
    WristRoll = 4,
    Gripper = 5,
    Aux = 6,
}

pub struct Arm {
    port: Box<dyn serialport::SerialPort>,
    servos: [u8; 7],
}

impl Arm {
    pub fn new(port: Box<dyn serialport::SerialPort>) -> Self {
        Arm {
            port,
            servos: [90; 7],
        }
    }

    // =========================
    // 🔹 BAIXO NÍVEL
    // =========================

    pub fn set_angle(&mut self, id: u8, angle: u8) {
        if id as usize >= self.servos.len() {
            return;
        }

        self.servos[id as usize] = angle;

        let comando = format!("{}:{}\n", id, angle);
        self.port.write_all(comando.as_bytes()).unwrap();

        let mut buffer = [0u8; 32];
        let _ = self.port.read(&mut buffer);
    }

    // =========================
    // 🔹 API COM ENUM (LIMPA)
    // =========================

    pub fn set_joint(&mut self, joint: Joint, angle: u8) {
        self.set_angle(joint as u8, angle);
    }

    // =========================
    // 🔹 MOVIMENTO SIMPLES
    // =========================

    pub fn move_group(&mut self, targets: &[(u8, u8)]) {
        let mut done = false;

        while !done {
            done = true;

            for (id, target) in targets {
                let idx = *id as usize;

                if idx >= self.servos.len() {
                    continue;
                }

                let current = self.servos[idx];
                let target = *target;

                if current < target {
                    self.set_angle(*id, current + 1);
                    done = false;
                } else if current > target {
                    self.set_angle(*id, current - 1);
                    done = false;
                }
            }

            thread::sleep(Duration::from_millis(20));
        }
    }

    // =========================
    // 🔥 MOVIMENTO SUAVE + BATCH
    // =========================

    pub fn move_group_smooth(&mut self, targets: &[(u8, u8)]) {
        let mut done = false;

        while !done {
            done = true;

            let mut comando = String::new();

            for (i, (id, target)) in targets.iter().enumerate() {
                let idx = *id as usize;

                if idx >= self.servos.len() {
                    continue;
                }

                let current = self.servos[idx];
                let target = *target;

                let next = if current < target {
                    done = false;
                    current + 1
                } else if current > target {
                    done = false;
                    current - 1
                } else {
                    current
                };

                self.servos[idx] = next;

                if i > 0 {
                    comando.push(',');
                }

                comando.push_str(&format!("{}:{}", id, next));
            }

            comando.push('\n');

            self.port.write_all(comando.as_bytes()).unwrap();

            let mut buffer = [0u8; 32];
            let _ = self.port.read(&mut buffer);

            thread::sleep(Duration::from_millis(20));
        }
    }

    // =========================
    // 🔹 API COM ENUM (SUAVE)
    // =========================

    pub fn move_joints_smooth(&mut self, targets: &[(Joint, u8)]) {
        let converted: Vec<(u8, u8)> = targets
            .iter()
            .map(|(j, a)| (*j as u8, *a))
            .collect();

        self.move_group_smooth(&converted);
    }

    // =========================
    // 🚀 TRAJETÓRIA
    // =========================

    pub fn execute_path(&mut self, path: Vec<Vec<(u8, u8)>>) {
        for point in path {
            self.move_group_smooth(&point);
            thread::sleep(Duration::from_millis(300));
        }
    }

    // 🔥 versão com enum (melhor ainda)
    pub fn execute_path_joints(&mut self, path: Vec<Vec<(Joint, u8)>>) {
        for point in path {
            self.move_joints_smooth(&point);
            thread::sleep(Duration::from_millis(300));
        }
    }
}