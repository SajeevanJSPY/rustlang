use std::str::Chars;

pub const EOF: char = '\0';

struct Cursor<'a> {
    len_remaining: u32,
    chars: Chars<'a>,
    #[cfg(debug_assertion)]
    prev: char
}

impl<'a> Cursor<'a> {
    pub fn new(chars: Chars<'a>) -> Self {
        Self {
            len_remaining: chars.as_str().len() as u32,
            chars,
            #[cfg(debug_assertion)]
            prev: EOF
        }
    }
}

