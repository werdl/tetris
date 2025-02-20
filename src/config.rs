use crossterm::event::KeyCode;
use serde::de::{self, Deserializer, Visitor};
use serde::{Deserialize, Serialize, Serializer};
use std::fmt;

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

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub MoveLeft: KeyCodeWrapper,
    pub MoveRight: KeyCodeWrapper,
    pub HardDrop: KeyCodeWrapper,
    pub SoftDrop: KeyCodeWrapper,
    pub RotateCW: KeyCodeWrapper,
    pub RotateCCW: KeyCodeWrapper,
    pub Hold: KeyCodeWrapper,
    pub Pause: KeyCodeWrapper,
    pub Quit: KeyCodeWrapper,

    pub SoftDropMsPerCell: u8,
}
