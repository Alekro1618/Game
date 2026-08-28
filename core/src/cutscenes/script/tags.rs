use engine::render::color::Color;

#[derive(Clone, Copy)]
pub enum Tag {
    Bold,
    Color(Color),
    Loud,
    Quiet,
    Scared,
    Unknown
}

impl From<&[char]> for Tag {
    fn from(value: &[char]) -> Self {
        let length = value.len();

        if length == 0 {
            return Tag::Unknown;
        }

        match value[0] {
            'b' => Tag::Bold,
            'l' => Tag::Loud,
            'q' => Tag::Quiet,
            's' => Tag::Scared,
            'c' => Tag::Color(Color::from(&value[1..])),
            _ => Tag::Unknown
        }
    }
}