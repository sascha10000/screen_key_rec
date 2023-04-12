use device_query::{DeviceEvents, DeviceState, Keycode};
use screenshots::Screen;
use std::{
    fs, thread,
    time::{Duration, Instant},
};

pub enum MouseEventType {
    MouseUp,
    MouseDown
}

pub enum KeyEventType {
    KeyUp,
    KeyDown
}

pub fn capture(file_prefix: &str) {
    let start = Instant::now();
    let screens = Screen::all().unwrap();

    for screen in screens {
        println!("capturer {screen:?}");
        let image = screen.capture().unwrap();
        let buffer = image.buffer();
        let duration = start
            .elapsed()
            .to_owned()
            .as_millis()
            .to_owned()
            .to_string()
            .to_owned();
        fs::write(
            format!(
                "target/{}{}_{}.png",
                file_prefix, screen.display_info.id, duration
            ),
            buffer,
        )
        .unwrap();
    }
}

pub fn rec_on_key<K, M>(file_prefix: &'static str, run_permantently: bool, on_key: K, on_mouse: M)
where
    K: Fn(&Keycode, KeyEventType) -> (),
    M: Fn(&usize, MouseEventType) -> ()
{
    let device_state = DeviceState::new();
    
    let _guard = device_state.on_key_down(|key| {
        capture(file_prefix);
    });

    let _guard = device_state.on_key_up(|key| {
        capture(file_prefix);
    });

    let _guard = device_state.on_mouse_down(|key| {
        capture(file_prefix);
    });

    let _guard = device_state.on_mouse_up(|key| {
        capture(file_prefix);
    });

    if (run_permantently) {
        loop {
            thread::sleep(Duration::from_secs(1000));
        }
    }
}

#[test]
fn test_capture() {
    rec_on_key("yalla", true, |k, e| (), |m, e| ());
}
