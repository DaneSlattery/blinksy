use blinksy::{
    layout::Layout1d,
    layout1d,
    patterns::rainbow::{Rainbow, RainbowParams},
    ControlBuilder,
};
use blinksy_desktop::{
    driver::{Desktop, DesktopError},
    time::elapsed_in_ms,
};
use std::{thread::sleep, time::Duration};

layout1d!(StripLayout, 30);

fn main() {
    Desktop::new_1d::<StripLayout>().start(|driver| {
        // Keep a click receiver before moving the driver into the control.
        let led_clicks = driver.led_clicks();
        let mut control = ControlBuilder::new_1d()
            .with_layout::<StripLayout, { StripLayout::PIXEL_COUNT }>()
            .with_pattern::<Rainbow>(RainbowParams {
                ..Default::default()
            })
            .with_driver(driver)
            .with_frame_buffer_size::<{ StripLayout::PIXEL_COUNT }>()
            .build();

        loop {
            while let Some(led_index) = led_clicks.try_take_led_click() {
                println!("LED {led_index} clicked");
            }

            if let Err(DesktopError::WindowClosed) = control.tick(elapsed_in_ms()) {
                break;
            }

            sleep(Duration::from_millis(16));
        }
    });
}
