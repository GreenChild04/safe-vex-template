use safe_vex::{maybe::Maybe, motor::Motor, port::PortManager};
use alloc::boxed::Box;

pub struct Drive {
    pub left_motor1: Maybe<Motor>,
    pub right_motor1: Maybe<Motor>,
    
    pub left_motor2: Maybe<Motor>,
    pub right_motor2: Maybe<Motor>,
}

impl Drive {
    #[inline]
    pub fn new(port_manager: &mut PortManager) -> Self {
        // reserve the ports in the port manager
        let _ = port_manager.take(1);
        let _ = port_manager.take(2);
        let _ = port_manager.take(3);
        let _ = port_manager.take(4);
        
        // construct the drive-train
        Self {
            left_motor1: Maybe::new(Box::new(|| build_motor(1, true))),
            left_motor2: Maybe::new(Box::new(|| build_motor(2, true))),

            right_motor1: Maybe::new(Box::new(|| build_motor(3, true))),
            right_motor2: Maybe::new(Box::new(|| build_motor(4, true))),
        }
    }

    #[inline]
    pub fn run(&mut self, left: i8, right: i8) {
        self.left_motor1.get().map(|motor| motor.move_i8(left));
        self.left_motor2.get().map(|motor| motor.move_i8(left));

        self.right_motor1.get().map(|motor| motor.move_i8(right));
        self.right_motor2.get().map(|motor| motor.move_i8(right));
    }
}

#[inline]
fn build_motor(port: u8, reverse: bool) -> Option<Motor> {
    // we know what we're doing (there won't be more than one mutable reference to the motor)
    unsafe { Motor::new(
        port,
        safe_vex::vex_rt::motor::Gearset::ThirtySixToOne,
        safe_vex::vex_rt::motor::EncoderUnits::Rotations,
        reverse,
    ) }.ok()
}
