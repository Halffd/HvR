use keyberon::layout::{Layout, Event};
use keyberon::matrix::Matrix;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::protocol::Event as X11Event;
use std::time::Duration;
use std::time::Instant;
use embedded_hal::digital::v2::{InputPin, OutputPin};

mod layout;
use crate::layout::LAYERS;

// Define pin types for the matrix
#[derive(Copy, Clone)]
pub struct InputPins;

#[derive(Copy, Clone)]
pub struct OutputPins;

// Implement required traits for the pins
impl InputPin for InputPins {
    type Error = ();
    fn is_high(&self) -> Result<bool, Self::Error> { Ok(false) }
    fn is_low(&self) -> Result<bool, Self::Error> { Ok(true) }
}

impl OutputPin for OutputPins {
    type Error = ();
    fn set_high(&mut self) -> Result<(), Self::Error> { Ok(()) }
    fn set_low(&mut self) -> Result<(), Self::Error> { Ok(()) }
}

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

// Custom matrix implementation for software simulation
struct KeyboardMatrix {
    state: [[bool; 12]; 4],
}

impl KeyboardMatrix {
    fn new() -> Self {
        Self {
            state: [[false; 12]; 4],
        }
    }

    fn get_events(&mut self) -> Vec<Event> {
        // Simulate key events here
        Vec::new()
    }
}

struct Keyboard {
    layout: Layout,
    matrix: KeyboardMatrix,
    window_info: Option<WindowInfo>,
    last_update: Instant,
}

impl Keyboard {
    fn new() -> Self {
        Self {
            layout: Layout::new(&LAYERS),
            matrix: KeyboardMatrix::new(),
            window_info: None,
            last_update: Instant::now(),
        }
    }

    fn update(&mut self) {
        // Update window info every second
        if self.last_update.elapsed() > Duration::from_secs(1) {
            self.window_info = WindowInfo::get_active_window();
            self.last_update = Instant::now();
        }

        // Process matrix events
        for event in self.matrix.get_events() {
            if let Some(window_info) = &self.window_info {
                // Disable remapping for specific applications
                let should_disable_remapping = window_info.class.contains("terminal")
                    || window_info.title.contains("vim")
                    || window_info.title.contains("code")
                    || window_info.title.contains("emacs");

                match event {
                    Event::Press(row, col) => {
                        if !should_disable_remapping && layout::is_remappable_key(row, col) {
                            // Apply our custom mapping
                            self.layout.event(Event::Press(row, col));
                            let remapped_action = layout::get_remapped_action(row, col);
                            self.layout.event(Event::Press(row, col));
                        } else {
                            // Process normally
                            self.layout.event(event);
                        }
                    }
                    Event::Release(row, col) => {
                        if !should_disable_remapping && layout::is_remappable_key(row, col) {
                            // Release both original and remapped key
                            self.layout.event(Event::Release(row, col));
                            let remapped_action = layout::get_remapped_action(row, col);
                            self.layout.event(Event::Release(row, col));
                        } else {
                            // Process normally
                            self.layout.event(event);
                        }
                    }
                }
            } else {
                // Fallback if window info isn't available
                self.layout.event(event);
            }
        }

        self.layout.tick();
    }

    fn get_report(&mut self) -> keyberon::key_code::KbHidReport {
        self.layout.keycodes().collect()
    }
}

fn main() {
    let mut keyboard = Keyboard::new();
    
    loop {
        keyboard.update();
        
        // Add appropriate delay to prevent high CPU usage
        std::thread::sleep(Duration::from_millis(1));
    }
}
