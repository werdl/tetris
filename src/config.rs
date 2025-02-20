use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal;
use serde::de::{self, Deserializer, Visitor};
use serde::{Deserialize, Serialize, Serializer};
use std::io::Write;
use std::{fmt, io};

#[derive(Clone)]
pub struct KeyCodeWrapper {
    pub code: KeyCode,
}

impl Serialize for KeyCodeWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.code {
            KeyCode::Backspace => serializer.serialize_str("Backspace"),
            KeyCode::Enter => serializer.serialize_str("Enter"),
            KeyCode::Left => serializer.serialize_str("Left"),
            KeyCode::Right => serializer.serialize_str("Right"),
            KeyCode::Up => serializer.serialize_str("Up"),
            KeyCode::Down => serializer.serialize_str("Down"),
            KeyCode::Home => serializer.serialize_str("Home"),
            KeyCode::End => serializer.serialize_str("End"),
            KeyCode::PageUp => serializer.serialize_str("PageUp"),
            KeyCode::PageDown => serializer.serialize_str("PageDown"),
            KeyCode::Tab => serializer.serialize_str("Tab"),
            KeyCode::BackTab => serializer.serialize_str("BackTab"),
            KeyCode::Delete => serializer.serialize_str("Delete"),
            KeyCode::Insert => serializer.serialize_str("Insert"),
            KeyCode::F(n) => serializer.serialize_str(&format!("F{}", n)),
            KeyCode::Char(c) => serializer.serialize_str(&format!("Char({})", c)),
            KeyCode::Null => serializer.serialize_str("Null"),
            KeyCode::Esc => serializer.serialize_str("Esc"),
            KeyCode::CapsLock => serializer.serialize_str("CapsLock"),
            KeyCode::ScrollLock => serializer.serialize_str("ScrollLock"),
            KeyCode::NumLock => serializer.serialize_str("NumLock"),
            KeyCode::PrintScreen => serializer.serialize_str("PrintScreen"),
            KeyCode::Pause => serializer.serialize_str("Pause"),
            KeyCode::Menu => serializer.serialize_str("Menu"),
            KeyCode::KeypadBegin => serializer.serialize_str("KeypadBegin"),
            KeyCode::Media(media) => serializer.serialize_str(&format!("Media({:?})", media)),
            KeyCode::Modifier(modifier) => {
                serializer.serialize_str(&format!("Modifier({:?})", modifier))
            }
        }
    }
}

impl<'de> Deserialize<'de> for KeyCodeWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Ok(Self {
            code: match value.as_str() {
                "Backspace" => KeyCode::Backspace,
                "Enter" => KeyCode::Enter,
                "Left" => KeyCode::Left,
                "Right" => KeyCode::Right,
                "Up" => KeyCode::Up,
                "Down" => KeyCode::Down,
                "Home" => KeyCode::Home,
                "End" => KeyCode::End,
                "PageUp" => KeyCode::PageUp,
                "PageDown" => KeyCode::PageDown,
                "Tab" => KeyCode::Tab,
                "BackTab" => KeyCode::BackTab,
                "Delete" => KeyCode::Delete,
                "Insert" => KeyCode::Insert,
                "Null" => KeyCode::Null,
                "Esc" => KeyCode::Esc,
                "CapsLock" => KeyCode::CapsLock,
                "ScrollLock" => KeyCode::ScrollLock,
                "NumLock" => KeyCode::NumLock,
                "PrintScreen" => KeyCode::PrintScreen,
                "Pause" => KeyCode::Pause,
                "Menu" => KeyCode::Menu,
                "KeypadBegin" => KeyCode::KeypadBegin,
                _ if value.starts_with("F") => {
                    let num = value[1..].parse::<u8>().map_err(de::Error::custom)?;
                    KeyCode::F(num)
                }
                _ if value.starts_with("Char(") && value.ends_with(")") => {
                    let c = value[5..value.len() - 1]
                        .chars()
                        .next()
                        .ok_or_else(|| de::Error::custom("invalid char"))?;
                    KeyCode::Char(c)
                }
                _ => {
                    return Err(de::Error::unknown_variant(
                        value.as_str(),
                        &[
                            "Backspace",
                            "Enter",
                            "Left",
                            "Right",
                            "Up",
                            "Down",
                            "Home",
                            "End",
                            "PageUp",
                            "PageDown",
                            "Tab",
                            "BackTab",
                            "Delete",
                            "Insert",
                            "Null",
                            "Esc",
                            "CapsLock",
                            "ScrollLock",
                            "NumLock",
                            "PrintScreen",
                            "Pause",
                            "Menu",
                            "KeypadBegin",
                            "F",
                            "Char",
                        ],
                    ))
                }
            },
        })
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub move_left: KeyCodeWrapper,
    pub move_right: KeyCodeWrapper,
    pub hard_drop: KeyCodeWrapper,
    pub soft_drop: KeyCodeWrapper,
    pub rotate_cw: KeyCodeWrapper,
    pub rotate_ccw: KeyCodeWrapper,
    pub hold: KeyCodeWrapper,
    pub pause: KeyCodeWrapper,
    pub quit: KeyCodeWrapper,

    pub soft_drop_ms_per_cell: u8,
}

fn input(prompt: String) -> KeyCodeWrapper {
    // disable raw mode for print
    terminal::disable_raw_mode().unwrap();
    print!("{}", prompt);
    io::Stdout::flush(&mut io::stdout()).unwrap();
    terminal::enable_raw_mode().unwrap();
    loop {
        match event::read() {
            Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => {
                // print the key pressed
                terminal::disable_raw_mode().unwrap();
                println!(
                    "{}",
                    serde_json::to_string(&KeyCodeWrapper { code: key.code }).unwrap()
                );
                terminal::enable_raw_mode().unwrap();
                return KeyCodeWrapper { code: key.code };
            }
            _ => {}
        }
    }
}

pub fn interactive_config() -> Config {
    let move_left = input("Press the key you want to use for moving left".to_string());
    let move_right = input("Press the key you want to use for moving right".to_string());
    let hard_drop = input("Press the key you want to use for hard dropping".to_string());
    let soft_drop = input("Press the key you want to use for soft dropping".to_string());
    let rotate_cw = input("Press the key you want to use for rotating clockwise".to_string());
    let rotate_ccw =
        input("Press the key you want to use for rotating counter-clockwise".to_string());
    let hold = input("Press the key you want to use for holding".to_string());
    let pause = input("Press the key you want to use for pausing".to_string());
    let quit = input("Press the key you want to use for quitting".to_string());

    terminal::disable_raw_mode().unwrap();
    let soft_drop_ms_per_cell = loop {
        println!("Enter the number of milliseconds you want to wait between each cell when soft dropping");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<u8>() {
            Ok(ms) => break ms,
            Err(_) => {
                println!("Invalid input, please enter a number");
            }
        }
    };

    terminal::enable_raw_mode().unwrap();

    Config {
        move_left,
        move_right,
        hard_drop,
        soft_drop,
        rotate_cw,
        rotate_ccw,
        hold,
        pause,
        quit,
        soft_drop_ms_per_cell,
    }
}
