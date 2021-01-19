use super::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Occurs(pub [u64; MAX]);

impl Default for Occurs {
    fn default() -> Self {
        Self([0; MAX])
    }
}

impl Add<char> for Occurs {
    type Output = Self;

    fn add(mut self, c: char) -> Self {
        self += c;
        self
    }
}

impl AddAssign<char> for Occurs {
    fn add_assign(&mut self, c: char) {
        if c.is_ascii() {
            self[c] += 1;
        }
    }
}

impl Add<&str> for Occurs {
    type Output = Self;

    fn add(mut self, s: &str) -> Self {
        self += s;
        self
    }
}

impl AddAssign<&str> for Occurs {
    fn add_assign(&mut self, s: &str) {
        for c in s.chars() {
            *self += c;
        }
    }
}

impl Index<char> for Occurs {
    type Output = u64;

    fn index(&self, c: char) -> &Self::Output {
        &self.0[c as usize]
    }
}

impl IndexMut<char> for Occurs {
    fn index_mut(&mut self, c: char) -> &mut Self::Output {
        &mut self.0[c as usize]
    }
}
