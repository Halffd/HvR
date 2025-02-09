use keyberon::action::{k, Action};
use keyberon::key_code::KeyCode;

// Define our layers
const DEFAULT: usize = 0;

// Create a complete keyboard layout
#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers = &[
    &[
        // Row 0
        &[k(KeyCode::Escape), k(KeyCode::Kb1), k(KeyCode::Kb2), k(KeyCode::Kb3), k(KeyCode::Kb4), k(KeyCode::Kb5), k(KeyCode::Kb6), k(KeyCode::Kb7), k(KeyCode::Kb8), k(KeyCode::Kb9), k(KeyCode::Kb0), k(KeyCode::BSpace)],
        
        // Row 1
        &[k(KeyCode::Tab), k(KeyCode::Q), k(KeyCode::W), k(KeyCode::E), k(KeyCode::R), k(KeyCode::T), k(KeyCode::Y), k(KeyCode::U), k(KeyCode::I), k(KeyCode::O), k(KeyCode::P), k(KeyCode::Enter)],
        
        // Row 2 (with arrow keys mapped to WASD)
        &[k(KeyCode::LCtrl), k(KeyCode::A), k(KeyCode::S), k(KeyCode::D), k(KeyCode::F), k(KeyCode::G), k(KeyCode::H), k(KeyCode::J), k(KeyCode::K), k(KeyCode::L), k(KeyCode::SColon), k(KeyCode::Quote)],
        
        // Row 3
        &[k(KeyCode::LShift), k(KeyCode::Z), k(KeyCode::X), k(KeyCode::C), k(KeyCode::V), k(KeyCode::B), k(KeyCode::N), k(KeyCode::M), k(KeyCode::Comma), k(KeyCode::Dot), k(KeyCode::Slash), k(KeyCode::RShift)],

        // Numpad section (if available)
        &[k(KeyCode::NumLock), k(KeyCode::Kp7), k(KeyCode::Kp8), k(KeyCode::Kp9), k(KeyCode::KpMinus)],
        &[k(KeyCode::KpSlash), k(KeyCode::Kp4), k(KeyCode::Kp5), k(KeyCode::Kp6), k(KeyCode::KpPlus)],
        &[k(KeyCode::KpAsterisk), k(KeyCode::Kp1), k(KeyCode::Kp2), k(KeyCode::Kp3), k(KeyCode::KpEnter)],
        &[k(KeyCode::Kp0), k(KeyCode::KpDot), k(KeyCode::No), k(KeyCode::No), k(KeyCode::No)],

        // Special function row for our custom mappings
        &[
            k(KeyCode::Up),     // W key becomes Up Arrow
            k(KeyCode::Left),   // A key becomes Left Arrow
            k(KeyCode::Down),   // S key becomes Down Arrow
            k(KeyCode::Right),  // D key becomes Right Arrow
            k(KeyCode::VolUp),    // Numpad Plus becomes Volume Up
            k(KeyCode::VolDown),  // Numpad Minus becomes Volume Down
            k(KeyCode::Mute),        // Numpad Divide becomes Mute
        ],
    ],
];

// Define key positions for special mappings
pub const WASD_UP: (u8, u8) = (1, 2);     // W key position
pub const WASD_LEFT: (u8, u8) = (2, 1);   // A key position
pub const WASD_DOWN: (u8, u8) = (2, 2);   // S key position
pub const WASD_RIGHT: (u8, u8) = (2, 3);  // D key position

pub const NUMPAD_PLUS: (u8, u8) = (5, 4);    // Numpad Plus position
pub const NUMPAD_MINUS: (u8, u8) = (4, 4);   // Numpad Minus position
pub const NUMPAD_DIVIDE: (u8, u8) = (5, 0);  // Numpad Divide position

// Helper function to check if a key position is a remappable key
pub fn is_remappable_key(row: u8, col: u8) -> bool {
    matches!(
        (row, col),
        WASD_UP | WASD_LEFT | WASD_DOWN | WASD_RIGHT |
        NUMPAD_PLUS | NUMPAD_MINUS | NUMPAD_DIVIDE
    )
}

// Helper function to get the remapped action for a key
pub fn get_remapped_action(row: u8, col: u8) -> Action {
    match (row, col) {
        // WASD to Arrow keys
        WASD_UP => k(KeyCode::Up),
        WASD_LEFT => k(KeyCode::Left),
        WASD_DOWN => k(KeyCode::Down),
        WASD_RIGHT => k(KeyCode::Right),
        
        // Numpad to Volume controls
        NUMPAD_PLUS => k(KeyCode::VolUp),
        NUMPAD_MINUS => k(KeyCode::VolDown),
        NUMPAD_DIVIDE => k(KeyCode::Mute),
        
        // Default case (shouldn't happen if used correctly)
        _ => k(KeyCode::No),
    }
} 