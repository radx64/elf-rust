use std::sync::atomic::AtomicBool;

static COLORS_ENABLED: AtomicBool = AtomicBool::new(true);

fn try_colorize(input: & 'static str) -> & 'static str
{
    if COLORS_ENABLED.load(std::sync::atomic::Ordering::Relaxed) == true{
        input
    } else {
        ""
    }
}

pub fn disable_colors(){
    COLORS_ENABLED.store(false, std::sync::atomic::Ordering::Relaxed);
}

pub fn enable_colors(){
    COLORS_ENABLED.store(true, std::sync::atomic::Ordering::Relaxed);
}

pub fn red() -> & 'static str {
    try_colorize("\x1b[91m")
} 
pub fn green() -> & 'static str {
    try_colorize("\x1b[92m")
} 
pub fn yellow() -> & 'static str {
    try_colorize("\x1b[93m")
} 
pub fn blue() -> & 'static str {
    try_colorize("\x1b[94m")
} 
pub fn purple() -> & 'static str {
    try_colorize("\x1b[95m")
} 
pub fn cyan() -> & 'static str {
    try_colorize("\x1b[96m")
} 
pub fn white() -> & 'static str {
    try_colorize("\x1b[97m")
} 
pub fn gray() -> & 'static str {
    try_colorize("\x1b[90m")
} 
pub fn default() -> & 'static str {
    try_colorize("\x1b[0m")
} 
