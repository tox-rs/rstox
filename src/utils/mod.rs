use std::iter::Iterator;
use core::{MAX_MESSAGE_LENGTH};

pub struct MessageSplit<'a> {
    msg: &'a str,
    pos: usize,
    max_len: usize,
}

impl<'a> MessageSplit<'a> {
    pub fn friend_split(msg: &str) -> MessageSplit {
        MessageSplit {
            msg: msg,
            pos: 0,
            max_len: MAX_MESSAGE_LENGTH,
        }
    }

    pub fn group_split(msg: &str)-> MessageSplit {
        MessageSplit {
            msg: msg,
            pos: 0,
            max_len: 1300,
        }
    }
}

impl<'a> Iterator for MessageSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        if self.pos >= self.msg.len() {
            return None
        } else if self.msg.len() <= self.pos + self.max_len {
            unsafe {
                let res = self.msg.slice_unchecked(self.pos, self.msg.len());
                self.pos += self.max_len;
                return Some(res)
            }
        } 

        let bytes = self.msg.as_bytes();
        let mut pos = self.pos + self.max_len;
        while bytes[pos] & 0xC0 == 0x80 {
            pos -= 1;
        }

        let mut white_pos = pos;
        for _ in 0..self.max_len/2 {
            match bytes[white_pos] {
                b'\t' | b'\n' | b' ' => {
                    let res = unsafe { self.msg.slice_unchecked(self.pos, white_pos) };
                    self.pos = white_pos + 1;
                    return Some(res)
                },
                _ => { white_pos -= 1; }
            }
        }

        let res = unsafe { self.msg.slice_unchecked(self.pos, pos) };
        self.pos = pos;
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::repeat;

    #[test] fn split_spaceless() {
        let msg: String = repeat('a').take(1350).collect();
        let mut split = MessageSplit::group_split(&msg);
        assert_eq!(1300, split.next().unwrap().len());
        assert_eq!(50, split.next().unwrap().len());
        assert_eq!(None, split.next());
    }
    #[test] fn tail_space() {
        let msg = repeat('a').take(1299).collect::<String>() + " bbb";
        let mut split = MessageSplit::group_split(&msg);
        assert_eq!(1299, split.next().unwrap().len());
        assert_eq!(Some("bbb"), split.next());
        assert_eq!(None, split.next());
    }
    #[test] fn exact_size() {
        let msg = repeat('a').take(1300).collect::<String>();
        let mut split = MessageSplit::group_split(&msg);
        assert_eq!(1300, split.next().unwrap().len());
        assert_eq!(None, split.next());
    }
    #[test] fn test_unicode() {
        let msg = repeat('௵').take(434).collect::<String>();
        let mut split = MessageSplit::group_split(&msg);
        assert_eq!(1299, split.next().unwrap().len());
        assert_eq!(3, split.next().unwrap().len());
        assert_eq!(None, split.next());
    }
}
