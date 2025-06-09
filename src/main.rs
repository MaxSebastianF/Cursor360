use device_query::{DeviceQuery, DeviceState, Keycode};

use vigem_client::{Client, TargetId, XButtons, XGamepad, Xbox360Wired};
use windows::Win32::Foundation::RECT;
use windows::Win32::UI::WindowsAndMessaging::SetCursorPos;
use windows::Win32::UI::WindowsAndMessaging::{
    ClipCursor, GetClientRect, GetDesktopWindow, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN,
};

fn main() {
    // Connect to ViGEmBus and set up a virtual Xbox 360 controller
    let client = Client::connect().expect("Couldn't connect to ViGEmBus");
    let mut target = Xbox360Wired::new(client, TargetId::XBOX360_WIRED);
    target
        .plugin()
        .expect("Failed to plug in virtual controller");
    target.wait_ready().expect("Controller not ready");

    let state = DeviceState::new();

    // Get screen center coords
    let screen_center_x = unsafe { GetSystemMetrics(SM_CXSCREEN) / 2 };
    let screen_center_y = unsafe { GetSystemMetrics(SM_CYSCREEN) / 2 };

    // Center the cursor and lock it to the screen
    unsafe {
        SetCursorPos(screen_center_x, screen_center_y);
        let mut rect = RECT::default();
        GetClientRect(GetDesktopWindow(), &mut rect);
        ClipCursor(Some(&rect));
    }

    let mut last_mouse_pos = (screen_center_x, screen_center_y);
    let mut cam_x: f32 = 0.0;
    let mut cam_y: f32 = 0.0;

    loop {
        let keys = state.get_keys();
        let mouse = state.get_mouse();

        // Exit if Ctrl + C is pressed
        if keys.contains(&Keycode::LControl) && keys.contains(&Keycode::C) {
            println!("Ctrl + C detected. Exiting...");
            break;
        }

        let current_mouse_pos = mouse.coords;
        let delta_x = current_mouse_pos.0 - last_mouse_pos.0;
        let delta_y = current_mouse_pos.1 - last_mouse_pos.1;

        // Re-center cursor after reading movement
        unsafe {
            SetCursorPos(screen_center_x, screen_center_y);
        }
        last_mouse_pos = (screen_center_x, screen_center_y);

        // Right mouse button = aiming
        let aiming = mouse.button_pressed[2];
        let current_sensitivity = if aiming { 18000.0 } else { 22000.0 };
        let smoothing_factor = if aiming { 0.12 } else { 0.2 };

        // Smooth camera movement
        cam_x = cam_x * (1.0 - smoothing_factor)
            + (delta_x as f32 * current_sensitivity) * smoothing_factor;
        cam_y = cam_y * (1.0 - smoothing_factor)
            + (delta_y as f32 * current_sensitivity) * smoothing_factor;

        let rx = cam_x.clamp(-32768.0, 32767.0) as i16;
        let ry = (-cam_y).clamp(-32768.0, 32767.0) as i16;

        let mut gamepad = XGamepad::default();

        // WASD movement + shift for slower speed
        let slow_mode = keys.contains(&Keycode::LShift) || keys.contains(&Keycode::RShift);
        let speed_factor = if slow_mode { 0.5 } else { 1.0 };

        if keys.contains(&Keycode::W) {
            gamepad.thumb_ly = (32767.0 * speed_factor) as i16;
        }
        if keys.contains(&Keycode::S) {
            gamepad.thumb_ly = (-32768.0 * speed_factor) as i16;
        }
        if keys.contains(&Keycode::A) {
            gamepad.thumb_lx = (-32768.0 * speed_factor) as i16;
        }
        if keys.contains(&Keycode::D) {
            gamepad.thumb_lx = (32767.0 * speed_factor) as i16;
        }

        // Triggers: Left = aim, Right = shoot
        if mouse.button_pressed[1] {
            gamepad.right_trigger = 255;
        }
        if mouse.button_pressed[2] {
            gamepad.left_trigger = 255;
        }

        // Map keyboard keys to Xbox buttons
        let mut buttons = XButtons(0);

        if keys.contains(&Keycode::Space) {
            buttons = XButtons!(A);
        }
        if keys.contains(&Keycode::F) {
            buttons = XButtons!(B);
        }
        if keys.contains(&Keycode::E) {
            buttons = XButtons!(X);
        }
        if keys.contains(&Keycode::T) {
            buttons = XButtons!(Y);
        }
        if keys.contains(&Keycode::Q) {
            buttons = XButtons!(LB);
        }
        if keys.contains(&Keycode::R) {
            buttons = XButtons!(RB);
        }
        if keys.contains(&Keycode::B) {
            buttons = XButtons!(START);
        }
        if keys.contains(&Keycode::Escape) {
            buttons = XButtons!(BACK);
        }
        if keys.contains(&Keycode::Key4) {
            buttons = XButtons!(UP);
        }
        if keys.contains(&Keycode::Key3) {
            buttons = XButtons!(DOWN);
        }
        if keys.contains(&Keycode::Key2) {
            buttons = XButtons!(LEFT);
        }
        if keys.contains(&Keycode::Key1) {
            buttons = XButtons!(RIGHT);
        }
        if mouse.button_pressed.get(3) == Some(&true) {
            buttons = XButtons!(RTHUMB);
        }

        gamepad.buttons = buttons;
        gamepad.thumb_rx = rx;
        gamepad.thumb_ry = ry;

        // Send updated state to the virtual controller
        target.update(&gamepad).expect("Failed to update gamepad");

        // Keep the loop tight
        std::hint::spin_loop();
    }
}
