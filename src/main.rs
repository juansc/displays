extern crate core_graphics;

use std::fmt::{Debug, Formatter};
use std::ptr;

use core_graphics::display::{
    CGBeginDisplayConfiguration, CGCompleteDisplayConfiguration, CGConfigureDisplayOrigin,
    CGConfigureOption, CGDirectDisplayID, CGDisplayBounds, CGDisplayConfigRef, CGDisplayIsBuiltin,
    CGGetActiveDisplayList,
};
use core_graphics::geometry::CGRect;

const LAPTOP_POSITION: (i32, i32) = (0, 0);

#[derive(Copy, Clone)]
struct DisplayInfo {
    id: CGDirectDisplayID,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

enum Location {
    TopLeft,
    TopRight,
}

impl Debug for DisplayInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DisplayInfo")
            .field("id", &self.id)
            .field("x", &self.x)
            .field("y", &self.y)
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

impl DisplayInfo {
    fn new(id: CGDirectDisplayID, bounds: CGRect) -> Self {
        Self {
            id,
            x: bounds.origin.x as i32,
            y: bounds.origin.y as i32,
            width: bounds.size.width as i32,
            height: bounds.size.height as i32,
        }
    }

    fn new_coordinates(&self, laptop: &DisplayInfo, loc: Location) -> (i32, i32) {
        match loc {
            // To go to the top left, the right edge of the screen must line up with the center of
            // the laptop.
            Location::TopLeft => {
                let new_x = laptop.width / 2 - self.width;
                let new_y = -self.height;
                (new_x, new_y)
            }
            Location::TopRight => {
                let new_x = laptop.width / 2;
                let new_y = -self.height;
                (new_x, new_y)
            }
        }
    }

    fn to_top_left(&self, laptop: &DisplayInfo) -> (i32, i32) {
        self.new_coordinates(laptop, Location::TopLeft)
    }

    fn to_top_right(&self, laptop: &DisplayInfo) -> (i32, i32) {
        self.new_coordinates(laptop, Location::TopRight)
    }
}

fn main() {
    let mut displays = [0; 32];
    let mut display_count = 0;
    unsafe {
        CGGetActiveDisplayList(32, displays.as_mut_ptr(), &mut display_count);
    }
    if display_count != 3 {
        return
    }
    let mut laptop_display_info: Option<DisplayInfo> = None;
    let mut external_monitors = vec![];

    for i in 0..display_count {
        let display_id = displays[i as usize];

        let bounds = unsafe { CGDisplayBounds(display_id) };
        let info = DisplayInfo::new(display_id, bounds);

        if unsafe { CGDisplayIsBuiltin(display_id) } == 1 {
            laptop_display_info = Some(info);
        } else {
            external_monitors.push(info);
        }
    }

    if external_monitors.len() != 2 {
        println!(
            "Only {} monitors detected. Must have exactly 2 external monitors.",
            external_monitors.len()
        );
        return
    }

    external_monitors.sort_by(|a, b| a.x.cmp(&b.x));

    let laptop_display_info = laptop_display_info.unwrap();
    let left_display_info = external_monitors[0];
    let right_display_info = external_monitors[1];

    // Now reset the displays and toggle
    // Grab a context, initialize it, and then set the display configuration.
    let mut config_ctx: CGDisplayConfigRef = ptr::null_mut();
    let out = unsafe { CGBeginDisplayConfiguration(&mut config_ctx) };
    if out != 0 {
        println!("Error: {out}");
        return;
    }
    update_pos(config_ctx, laptop_display_info.id, LAPTOP_POSITION);
    update_pos(
        config_ctx,
        right_display_info.id,
        right_display_info.to_top_left(&laptop_display_info),
    );
    update_pos(
        config_ctx,
        left_display_info.id,
        left_display_info.to_top_right(&laptop_display_info),
    );
    // Commit the configuration.
    let out = unsafe {
        CGCompleteDisplayConfiguration(config_ctx, CGConfigureOption::ConfigurePermanently)
    };
    if out != 0 {
        println!("Error: {out}");
    }
}

fn update_pos(config_ctx: CGDisplayConfigRef, display: CGDirectDisplayID, pos: (i32, i32)) {
    let out = unsafe { CGConfigureDisplayOrigin(config_ctx, display, pos.0, pos.1) };
    if out != 0 {
        println!("Error: {out}");
    }
}

// based off of this from StackOverflow https://stackoverflow.com/a/64126582
// #include <IOKit/graphics/IOGraphicsLib.h>
// #include <ApplicationServices/ApplicationServices.h>
// #include <unistd.h>
// #include <math.h>
// #include <stdio.h>
//
// int main(int argc, const char * argv[]) {
//
//     CGDirectDisplayID screenList[3];
//     CGDirectDisplayID external1;
//     CGDirectDisplayID external2;
//     CGGetOnlineDisplayList(INT_MAX, screenList, NULL);
//
//     if(CGDisplayIsMain(screenList[0])){
//         external1 = screenList[1];
//         external2 = screenList[2];
//     }else if(CGDisplayIsMain(screenList[1])){
//         external1 = screenList[0];
//         external2 = screenList[2];
//     }else{
//         external1 = screenList[1];
//         external2 = screenList[2];
//     }
//
//
//     CGDisplayConfigRef configRef;
//     CGBeginDisplayConfiguration(&configRef);
//
//     CGConfigureDisplayOrigin(configRef, external1, CGDisplayBounds(external2).origin.x, CGDisplayBounds(external2).origin.y);
//     CGConfigureDisplayOrigin(configRef, external2, CGDisplayBounds(external1).origin.x, CGDisplayBounds(external1).origin.y);
//
//     CGCompleteDisplayConfiguration(configRef, kCGConfigurePermanently);
// }
