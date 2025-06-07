use device_query::{DeviceQuery, DeviceState, Keycode};
use std::{thread, time::Duration};
use vigem_client::{Client, TargetId, XButtons, XGamepad, Xbox360Wired};

use winapi::shared::windef::RECT;

use winapi::um::winuser::{ClipCursor, GetClientRect, GetDesktopWindow};
use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN, SetCursorPos};

/*use winapi::um::winuser::ShowCursor;
use winapi::shared::minwindef::{TRUE, FALSE};  // Importa TRUE y FALSE desde aquí*/
fn main() {
    /*unsafe {
        ShowCursor(TRUE); // Muestra el cursor otra vez
    }*/
    let client = Client::connect().expect("No se pudo conectar con ViGEmBus");
    let mut target = Xbox360Wired::new(client, TargetId::XBOX360_WIRED);
    target
        .plugin()
        .expect("No se pudo conectar el mando virtual");
    target.wait_ready().expect("El mando virtual no está listo");

    let state = DeviceState::new();
    let screen_center_x = unsafe { GetSystemMetrics(SM_CXSCREEN) / 2 };
    let screen_center_y = unsafe { GetSystemMetrics(SM_CYSCREEN) / 2 };

    unsafe {
        SetCursorPos(screen_center_x, screen_center_y);

        // Confinar cursor a toda la pantalla (evita doble clic, evita cursor visible)
        let mut rect: RECT = std::mem::zeroed();
        GetClientRect(GetDesktopWindow(), &mut rect);
        ClipCursor(&rect);
    }

    let mut last_mouse_pos = (screen_center_x, screen_center_y);
    let mut cam_x: f32 = 0.0;
    let mut cam_y: f32 = 0.0;

    loop {
        let keys = state.get_keys();
        let mouse = state.get_mouse();

        if keys.contains(&Keycode::LControl) && keys.contains(&Keycode::C) {
            println!("Ctrl + C detectado. Saliendo...");
            break;
        }

        let current_mouse_pos = mouse.coords;
        let delta_x = current_mouse_pos.0 - last_mouse_pos.0;
        let delta_y = current_mouse_pos.1 - last_mouse_pos.1;

        // Recentrar cursor para evitar que toque bordes
        unsafe {
            SetCursorPos(screen_center_x, screen_center_y);
        }
        last_mouse_pos = (screen_center_x, screen_center_y);

        // Sensibilidad variable: apuntando o no
        let aiming = mouse.button_pressed[2];
        let friction = if aiming { 0.95 } else { 0.95 };
        let current_sensitivity = if aiming { 3250.0 } else { 3200.0 };

        // Movimiento de cámara
        cam_x += delta_x as f32 * current_sensitivity;
        cam_y += delta_y as f32 * current_sensitivity;

        cam_x *= friction;
        cam_y *= friction;

     let rx = cam_x.clamp(-32768.0, 32767.0) as i16;
let ry = (-cam_y).clamp(-32768.0, 32767.0) as i16;

        let mut gamepad = XGamepad::default();
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

        // Clicks del mouse → LT y RT
        if mouse.button_pressed[2] {
            gamepad.left_trigger = 255;
        }
        if mouse.button_pressed[1] {
            gamepad.right_trigger = 255;
        }

        // Joystick derecho (movimiento de cámara)

        // Botones
        let mut button_flags = vec![];

        if keys.contains(&Keycode::Space) {
            button_flags.push("A");
        }
        if keys.contains(&Keycode::F) {
            button_flags.push("B");
        }
        if keys.contains(&Keycode::E) {
            button_flags.push("X");
        }
        if keys.contains(&Keycode::T) {
            button_flags.push("Y");
        }
        if keys.contains(&Keycode::Q) {
            button_flags.push("LB");
        }
        if keys.contains(&Keycode::R) {
            button_flags.push("RB");
        }
        if keys.contains(&Keycode::B) {
            button_flags.push("START");
        }
        if keys.contains(&Keycode::Escape) {
            button_flags.push("BACK");
        }
        if keys.contains(&Keycode::Key4) {
            button_flags.push("UP");
        }
        if keys.contains(&Keycode::Key3) {
            button_flags.push("DOWN");
        }
        if keys.contains(&Keycode::Key2) {
            button_flags.push("LEFT");
        }
        if keys.contains(&Keycode::Key1) {
            button_flags.push("RIGHT");
        }


        let buttons = match button_flags.len() {
            0 => XButtons!(),
            _ => {
                let joined = button_flags.join(" | ");
                match joined.as_str() {
                    "UP" => XButtons!(UP),
                    "DOWN" => XButtons!(DOWN),
                    "LEFT" => XButtons!(LEFT),
                    "RIGHT" => XButtons!(RIGHT),
                    "A" => XButtons!(A),
                    "B" => XButtons!(B),
                    "X" => XButtons!(X),
                    "Y" => XButtons!(Y),
                    "LB" => XButtons!(LB),
                    "RB" => XButtons!(RB),
                    "START" => XButtons!(START),
                    "BACK" => XButtons!(BACK),
                    // Agrega más combinaciones si las usas frecuentemente
                    _ => {
                        println!("Combinación no reconocida: {}", joined);
                        XButtons!()
                    }
                }
            }
        };
        gamepad.thumb_rx = rx;
        gamepad.thumb_ry = ry;
        gamepad.buttons = buttons;
        target.update(&gamepad).expect("Error al enviar estado");

        thread::sleep(Duration::from_millis(1)); // Menor delay = más fluidez
    }
}
