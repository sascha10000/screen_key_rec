use device_query::{DeviceEvents, DeviceState, Keycode, CallbackGuard, MouseButton};
use screenshots::Screen;
use std::{
    fs,
    sync::Arc,
    thread,
    time::Duration,
};
use std::marker::{ Sync, Send };

pub enum MouseEventType {
    MouseUp,
    MouseDown,
}

pub enum KeyEventType {
    KeyUp,
    KeyDown,
}

#[derive(Debug)]
pub struct RegisterEvents {
    key_up: bool,
    key_down: bool,
    mouse_up: bool,
    mouse_down: bool,
}

impl Default for RegisterEvents {
    fn default() -> Self {
        Self {
            key_up: true,
            key_down: true,
            mouse_up: true,
            mouse_down: true,
        }
    }
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
            println!("{}", file_name);
            fs::write(file_name, &buffer).unwrap();
        }
    });
}

pub fn capture_and_buffer() -> Vec<Vec<u8>> {
    let screens = Screen::all().unwrap();
    let mut buffers: Vec<Vec<u8>> = Vec::new();
    for screen in screens {
        buffers.push(capture(&screen));
    }

    buffers
}

pub fn capture(screen: &Screen) -> Vec<u8> {
    let image = screen.capture().unwrap();
    let buffer = image.buffer().to_owned();
    buffer
}

pub fn rec_to_file<K1, M1, K2, M2>(
    file_prefix: &'static str,
    events: RegisterEvents,
    run_permantently: bool,
    on_key_down: K1,
    on_mouse_down: M1,
    on_key_up: K2,
    on_mouse_up: M2,
) where
    K1: Fn(&Keycode, KeyEventType) -> () + Send + Sync + 'static,
    K2: Fn(&Keycode, KeyEventType) -> () + Send + Sync + 'static,
    M1: Fn(&usize, MouseEventType) -> () + Send + Sync + 'static,
    M2: Fn(&usize, MouseEventType) -> () + Send + Sync + 'static,
{
    let device_state = DeviceState::new();
    let _guard = if events.key_down {
        Some(device_state.on_key_down(move |key| {
            println!("CB exec");
            capture_and_write(file_prefix);
            on_key_down(key, KeyEventType::KeyDown);
        }))
    }
    else { None };

    let _guard = if events.key_up {
        Some(device_state.on_key_up(move |key| {
            capture_and_write(file_prefix);
            on_key_up(key, KeyEventType::KeyUp);
        }))
    }
    else { None };

    let _guard = if events.mouse_down {
        Some(device_state.on_mouse_down(move |key| {
            capture_and_write(file_prefix);
            on_mouse_down(key, MouseEventType::MouseDown);
        }))
    }
    else { None };

    let _guard = if events.mouse_up {
        Some(device_state.on_mouse_up(move |key| {
            capture_and_write(file_prefix);
            on_mouse_up(key, MouseEventType::MouseUp);
        }))
    }
    else { None };

    if run_permantently
        && (events.key_down || events.key_up || events.mouse_down || events.mouse_up)
    {
        loop {
            thread::sleep(Duration::from_secs(1000));
        }
    }
}

pub fn rec_buffer<OnKey, OnMouse>(
    events: RegisterEvents,
    run_permantently: bool,
    on_key: Arc<OnKey>,
    on_mouse: Arc<OnMouse>,
) where
    for<'lt> OnKey: Fn(Keycode, KeyEventType, Vec<Vec<u8>>) + Sync + Send + 'lt,
    for<'lt> OnMouse: Fn(usize, MouseEventType, Vec<Vec<u8>>) + Sync + Send + 'lt,
{
    let device_state = DeviceState::new();
    
    // TODO: Add guards (otherwise ref is dropped an therefore the event is gone)
    let _guard = if events.key_down {
        let kd = Arc::clone(&on_key);
        Some(device_state.on_key_down(move |key| {
            let buffers = capture_and_buffer();
            kd(key.clone(), KeyEventType::KeyDown, buffers);
        }))
    }
    else { None };

    let _guard = if events.key_up {
        let kd = Arc::clone(&on_key);
        Some(device_state.on_key_up(move |key| {
            let buffers = capture_and_buffer();
            kd(key.clone(), KeyEventType::KeyUp, buffers);
        }))
    }
    else { None };

    let _guard = if events.mouse_down {
        let md = Arc::clone(&on_mouse);
        Some(device_state.on_mouse_down(move |key| {
            let buffers = capture_and_buffer();
            md(key.clone(), MouseEventType::MouseDown, buffers);
        }))
    }
    else { None };

    let _guard = if events.mouse_up {
        let md = Arc::clone(&on_mouse);
        Some(device_state.on_mouse_up(move |key| {
            let buffers = capture_and_buffer();
            md(key.clone(), MouseEventType::MouseUp, buffers);
        }))
    }
    else { None };

    if run_permantently
        && (events.key_down || events.key_up || events.mouse_down || events.mouse_up)
    {
        loop {
            thread::sleep(Duration::from_secs(1000));
        }
    }
}

#[test]
fn test_capture() {
    rec_to_file(
        "yalla",
        RegisterEvents::default(),
        true,
        |k, e| (),
        |m, e| (),
        |k, e| (),
        |k, e| (),
    );
}
