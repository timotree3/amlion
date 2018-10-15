use std::num::NonZeroU8;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Letters {
    inner: HashMap<char, NonZeroU8>,
}

impl Letters {
    pub fn from_string(string: &str) -> Result<Letters, ()> {
        let mut result = Letters::empty();
        for (i, letter) in string.chars().enumerate() {
            if i >= ::std::usize::MAX {
                Err(())?;
            }
            result.add(letter);
        }
        eprintln!("starts with {:?}", result.inner);
        Ok(result)
    }

    pub fn has(&self, word: &str) -> bool {
        // eprintln!("TRACE: entering fn has");
        let mut allowed: Letters = self.clone();
        for letter in word.chars() {
            // eprintln!("DEBUG: checking letter {}", letter);
            allowed = match allowed.subtract(letter) {
                Ok(a) => a,
                Err(()) => return false,
            };
        }
        // eprintln!("TRACE: exiting fn has");
        true
    }

    fn add(&mut self, new: char) {
        self
            .inner
            .entry(new)
            .and_modify(|count| *count = NonZeroU8::new(count.get() + 1).unwrap())
            .or_insert(NonZeroU8::new(1).unwrap());
    }

    fn subtract(mut self, key: char) -> Result<Self,()> {
        // eprintln!("TRACE: entering fn subtract");
        // eprintln!("DEBUG: might error because of key not found");
        let old_count = self
            .inner
            .remove(&key)
            .ok_or(())?;
        // eprintln!("DEBUG: didn't error because of key not found");
        if let Some(updated_count) = NonZeroU8::new(old_count.get() - 1) {
            self
                .inner
                .insert(key, updated_count);
        }
        // eprintln!("TRACE: exiting fn subtract");
        Ok(self)
    }

    fn empty() -> Letters {
        Letters {
            inner: HashMap::new(),
        }
    }
}
