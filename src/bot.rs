use crate::drive::Drive;
use safe_vex::prelude::*;

pub struct MyRobot {
    drive: Drive,
}

impl Bot for MyRobot {
    const TICK_SPEED: u64 = 50;
    
    #[inline]
    fn new(_: &Peripherals, port_manager: &mut PortManager) -> Self {
        Self {
            drive: Drive::new(port_manager),
        }
    }
    
    #[inline]
    fn disabled(&mut self, _context: Context) -> bool {
        // your disabled code here
        // (code that runs while the robot is disabled)

        true // if it has completed no not
    }

    #[inline]
    fn opcontrol(&mut self, context: Context) -> bool {
        // (code that runs during the driver-control period)

        // get controller info
        let left_stick = context.controller.left_stick.y;
        let right_stick = context.controller.right_stick.y;

        // update drive-train
        self.drive.run(left_stick, right_stick);
        
        false // if it has completed or not
    }

    #[inline]
    fn autonomous(&mut self, _context: Context) -> bool {
        // your autonomous code here
        // (code that runs during the autonomous period)

        true // if it has completed no not
    }
}
