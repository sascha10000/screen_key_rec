use device_query::{DeviceEvents, DeviceState, Keycode};
use screenshots::Screen;
use std::{
    fs, thread,
    time::{Duration, Instant}, sync::Arc,
};

pub enum MouseEventType {
    MouseUp,
    MouseDown
}

pub enum KeyEventType {
    KeyUp,
    KeyDown
}

impl From<KeyEventType> for String {
    fn from(value: KeyEventType) -> String {
        String::from("KeyDown")
    }
}

pub fn capture_and_write(file_prefix: &str) {
    let screens = Screen::all().unwrap();
    let file_prefix = file_prefix.to_string();
    thread::spawn(move || {
        for (i, screen) in screens.iter().enumerate() {
            let buffer = capture(screen);
            let ts = chrono::offset::Utc::now().format("%Y-%m-%d %H_%M_%S%.f");
            let file_name = format!("target/{}_{}_screen_{}.png", file_prefix, ts, i);
            println!("{}", file_name);
            fs::write(
                file_name,
                &buffer,
            )
            .unwrap();
        }
    });
}

pub fn capture(screen: &Screen) -> Vec<u8> {
    let image = screen.capture().unwrap();
    image.buffer().to_owned()
}

pub fn rec_on_key<K1, M1, K2, M2>(file_prefix: &'static str, run_permantently: bool, on_key_down: K1, on_mouse_down: M1, on_key_up: K2, on_mouse_up: M2)
where
    K1: Fn(&Keycode, KeyEventType) -> () + std::marker::Send + std::marker::Sync + 'static,
    K2: Fn(&Keycode, KeyEventType) -> () + std::marker::Send + std::marker::Sync + 'static,
    M1: Fn(&usize, MouseEventType) -> () + std::marker::Send + std::marker::Sync + 'static,
    M2: Fn(&usize, MouseEventType) -> () + std::marker::Send + std::marker::Sync + 'static
{
    let device_state = DeviceState::new();    
    let _guard = device_state.on_key_down(move |key| {
        capture_and_write(file_prefix);
        on_key_down(key, KeyEventType::KeyDown);
    });

    let _guard = device_state.on_key_up(move |key| {
        capture_and_write(file_prefix);
        on_key_up(key, KeyEventType::KeyUp);
    }); 
    
    let _guard = device_state.on_mouse_down(move |key| {
        capture_and_write(file_prefix);
        on_mouse_down(key, MouseEventType::MouseDown); 
    });

    let _guard = device_state.on_mouse_up(move |key| {
        capture_and_write(file_prefix);
        on_mouse_up(key, MouseEventType::MouseUp);
    }); 

    if run_permantently {
        loop {
            thread::sleep(Duration::from_secs(1000));
        }
    }
}

#[test]
fn test_capture() {
    rec_on_key("yalla", true, |k, e| (), |m, e| (), |k,e| (), |k,e| ());
}
