use crate::errors::*;
use enigo::*;
use std::collections::*;
use std::iter::*;

pub struct Robot {
    enigo: Enigo,
    mouse_speed: f32,
    wheel_speed: f32,
}

pub enum MouseButton {
    Left,
    Right,
}

pub enum WheelDirection {
    Up,
    Down,
}

impl Robot {
    pub fn new(mouse_speed: f32, wheel_speed: f32) -> Robot {
        let enigo = Enigo::new();
        Robot {
            enigo,
            mouse_speed,
            wheel_speed,
        }
    }

    pub async fn mouse_move(&mut self, x: i32, y: i32) -> Result<()> {
        let x = (x as f32 * self.mouse_speed).round() as i32;
        let y = (y as f32 * self.mouse_speed).round() as i32;
        self.enigo.mouse_move_relative(x, y);
        Ok(())
    }

    pub async fn mouse_press(&mut self, button: MouseButton) -> Result<()> {
        let eb = enigo::MouseButton::from(button);
        self.enigo.mouse_down(eb);
        Ok(())
    }

    pub async fn mouse_release(&mut self, button: MouseButton) -> Result<()> {
        let eb = enigo::MouseButton::from(button);
        self.enigo.mouse_up(eb);
        Ok(())
    }

    pub async fn mouse_wheel(&mut self, dir: WheelDirection) -> Result<()> {
        let d = match dir {
            WheelDirection::Up => -1,
            WheelDirection::Down => 1,
        };
        let d = (d as f32 * self.wheel_speed).round() as i32;
        self.enigo.mouse_scroll_y(d);
        Ok(())
    }

    fn to_keys(&self, letter: String) -> LinkedList<enigo::Key> {
        fn to_key(l: &str) -> Vec<enigo::Key> {
            match l {
                "backspace" => vec![enigo::Key::Backspace],
                "enter" => vec![enigo::Key::Return],
                "space" => vec![enigo::Key::Space],
                x => x.chars().map(enigo::Key::Layout).collect(),
            }
        }

        letter.split("--").map(to_key).flatten().collect()
    }

    pub async fn keyboard_type_str(&mut self, letter: String) -> Result<()> {
        self.to_keys(letter)
            .iter()
            .for_each(|k| self.enigo.key_click(*k));
        Ok(())
    }

    pub async fn keyboard_type_int(&mut self, key: u16) -> Result<()> {
        let key = enigo::Key::Raw(key);
        self.enigo.key_click(key);
        Ok(())
    }
}

impl From<MouseButton> for enigo::MouseButton {
    fn from(b: MouseButton) -> enigo::MouseButton {
        match b {
            MouseButton::Left => enigo::MouseButton::Left,
            MouseButton::Right => enigo::MouseButton::Right,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_to_keys(letter: &str, expected: &[enigo::Key]) {
        let r = Robot::new(1.0, 1.0);
        let actual = r.to_keys(String::from(letter));
        let expected = LinkedList::from_iter(expected.iter().map(|r| *r));
        assert_eq!(expected, actual);
    }

    #[test]
    fn ksb_single_char() {
        assert_to_keys("F", &[enigo::Key::Layout('F')]);
    }

    #[test]
    fn ksb_two_chars() {
        assert_to_keys("F--o", &[enigo::Key::Layout('F'), enigo::Key::Layout('o')]);
    }

    #[test]
    fn ksb_minus() {
        assert_to_keys("-", &[enigo::Key::Layout('-')]);
    }

    #[test]
    fn ksb_minus_multi() {
        "F-----o".split("--").for_each(|k| println!("{}", k));
        assert_to_keys(
            "F-----o",
            &[
                enigo::Key::Layout('F'),
                enigo::Key::Layout('-'),
                enigo::Key::Layout('o'),
            ],
        );
    }

    #[test]
    fn ksb_spec_space() {
        assert_to_keys(
            "C--e--m--space--C--a--t",
            &[
                enigo::Key::Layout('C'),
                enigo::Key::Layout('e'),
                enigo::Key::Layout('m'),
                enigo::Key::Space,
                enigo::Key::Layout('C'),
                enigo::Key::Layout('a'),
                enigo::Key::Layout('t'),
            ],
        );
    }

    #[test]
    fn ksb_spec_backspace() {
        assert_to_keys(
            "C--e--x--backspace--m",
            &[
                enigo::Key::Layout('C'),
                enigo::Key::Layout('e'),
                enigo::Key::Layout('x'),
                enigo::Key::Backspace,
                enigo::Key::Layout('m'),
            ],
        );
    }
}
