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
    fn autonomous(&'a mut self, _: &'a Context) {
        // Write your autonomous code here
    }

    #[inline]
    fn opcontrol(&'a mut self, context: &'a Context) {
        // Get state of controller joysticks
        let left_stick = context.controller().left_stick().clamp(8).y;
        let right_stick = context.controller().right_stick().clamp(8).y;

        // Flush log if x button pressed
        if context.controller().x() {
            context.flush_logs();
        }

        // Update the drivetrain / motors
        self.drive.run(context, left_stick, right_stick);
    }

    // note: there's no disabled runtime unlike in `vex-rt` (notify me if you want this feature)
}
