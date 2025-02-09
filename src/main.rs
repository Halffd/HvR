use rdev::{listen, Event, EventType, Key, simulate, SimulateError};
use std::time::{Duration, Instant};
use std::process::Command;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;

struct WindowInfo {
    title: String,
    class: String,
}

impl WindowInfo {
    fn get_active_window() -> Option<WindowInfo> {
        // Connect to X server
        let (conn, screen_num) = x11rb::connect(None).ok()?;
        
        // Get the root window
        let setup = conn.setup();
        let screen = &setup.roots[screen_num];
        
        // Get active window
        let active_window = conn.get_property(
            false,
            screen.root,
            conn.intern_atom(false, b"_NET_ACTIVE_WINDOW").ok()?.reply().ok()?.atom,
            AtomEnum::WINDOW,
            0,
            1,
        ).ok()?.reply().ok()?;

        if active_window.value.len() != std::mem::size_of::<u32>() {
            return None;
        }

        let window_id = u32::from_ne_bytes(active_window.value[..4].try_into().ok()?);
        
        // Get window title
        let title_reply = conn.get_property(
            false,
            window_id,
            conn.intern_atom(false, b"_NET_WM_NAME").ok()?.reply().ok()?.atom,
            AtomEnum::STRING,
            0,
            1024,
        ).ok()?.reply().ok()?;
        
        let title = String::from_utf8_lossy(&title_reply.value).into_owned();

        // Get window class
        let class_reply = conn.get_property(
            false,
            window_id,
            AtomEnum::WM_CLASS,
            AtomEnum::STRING,
            0,
            1024,
        ).ok()?.reply().ok()?;
        
        let class = String::from_utf8_lossy(&class_reply.value).into_owned();

        Some(WindowInfo {
            title,
            class,
        })
    }
}

fn send_key(key: Key) -> Result<(), SimulateError> {
    simulate(&EventType::KeyPress(key))?;
    std::thread::sleep(Duration::from_millis(20));
    simulate(&EventType::KeyRelease(key))?;
    Ok(())
}

fn send_media_command(cmd: &str) {
    match cmd {
        "vol_up" => { Command::new("pactl").args(["set-sink-volume", "@DEFAULT_SINK@", "+5%"]).spawn().ok(); }
        "vol_down" => { Command::new("pactl").args(["set-sink-volume", "@DEFAULT_SINK@", "-5%"]).spawn().ok(); }
        "vol_mute" => { Command::new("pactl").args(["set-sink-mute", "@DEFAULT_SINK@", "toggle"]).spawn().ok(); }
        "play_pause" => { Command::new("playerctl").arg("play-pause").spawn().ok(); }
        _ => {}
    }
}

fn handle_event(event: Event) {
    // Get active window info
    let window_info = WindowInfo::get_active_window();
    
    // Check if we should disable remapping for the current window
    let should_disable_remapping = if let Some(window_info) = &window_info {
        window_info.class.contains("terminal")
            || window_info.title.contains("vim")
            || window_info.title.contains("code")
            || window_info.title.contains("emacs")
    } else {
        false
    };

    if should_disable_remapping {
        return;
    }

    match event.event_type {
        EventType::KeyPress(key) => {
            match key {
                // WASD to Arrow keys
                Key::KeyW => { let _ = send_key(Key::UpArrow); }
                Key::KeyA => { let _ = send_key(Key::LeftArrow); }
                Key::KeyS => { let _ = send_key(Key::DownArrow); }
                Key::KeyD => { let _ = send_key(Key::RightArrow); }
                
                // Numpad to Volume controls
                Key::KpPlus => send_media_command("vol_up"),
                Key::KpMinus => send_media_command("vol_down"),
                Key::KpDivide => send_media_command("vol_mute"),
                
                // Play/Pause key
                Key::F6 => send_media_command("play_pause"),
                
                _ => {}
            }
        }
        _ => {}
    }
}

fn main() {
    println!("Starting key remapper...");
    println!("WASD -> Arrow keys");
    println!("Numpad +/- -> Volume Up/Down");
    println!("Numpad / -> Volume Mute");
    println!("F6 -> Play/Pause");
    
    if let Err(error) = listen(handle_event) {
        println!("Error: {:?}", error);
    }
}
