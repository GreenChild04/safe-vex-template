use crate::drive::Drive;
use safe_vex::prelude::*;

pub struct MyRobot {
    drive: Drive,
}

impl<'a> Bot<'a> for MyRobot {
    #[inline]
    fn new(context: &'a Context) -> Self {
        Self {
            drive: Drive::new(context)
        }
    }

    #[inline]
    fn autonomous(&'a mut self, _: &'a mut Context) {
        // Write your autonomous code here
    }

    #[inline]
    fn opcontrol(&'a mut self, context: &'a mut Context) {
        // Get state of controller joysticks
        let left_stick: i8;
        let right_stick: i8;
        {
            let mut controller = context.controller();
            left_stick = controller.left_stick().clamp(8).y;
            right_stick = controller.right_stick().clamp(8).y;

            // Flush log if x button pressed
            if controller.x() {
                context.flush_logs();
            }
        }

        // Update the drivetrain / motors
        self.drive.run(&context, left_stick, right_stick);
    }

    #[inline]
    fn disabled(&'a mut self, _: &'a mut Context) {
        // This runs when the robot is in disabled mode
    }
}