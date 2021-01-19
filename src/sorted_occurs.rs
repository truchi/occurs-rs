use super::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct SortedOccurs(pub [(char, u64); MAX]);

impl From<Occurs> for SortedOccurs {
    fn from(occurs: Occurs) -> Self {
        let mut sorted = Self::default();

        for (i, count) in occurs.0.iter().enumerate() {
            sorted.0[i] = (i as u8 as char, *count);
        }

        sorted.0.sort_by(|(_, a), (_, b)| b.cmp(a));

        sorted
    }
}

impl Default for SortedOccurs {
    fn default() -> Self {
        Self([(char::default(), 0); MAX])
    }
}

impl Display for SortedOccurs {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut total = None;

        for (c, count) in &self.0 {
            if *count != 0 && !c.is_ascii_alphanumeric() {
                if *c as u8 >= 33 {
                    total = total.or(Some(*count as f32));

                    writeln!(
                        f,
                        "  {}    {:3.0}%",
                        c,
                        *count as f32 / total.unwrap() * 100.
                    )?;
                }
            }
        }

        for (c, count) in &self.0 {
            if *count != 0 && !c.is_ascii_alphanumeric() {
                if *c as u8 >= 33 {
                    write!(f, "{} ", c)?;
                }
            }
        }

        Ok(())
    }
}
