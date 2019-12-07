//! Holds the previous position we sliced the reference at
//! Helps to *slice* the reference

pub struct Seeker {
    chromosome: String,
    previous_position: u64,
}

impl Seeker {
    pub fn new(chromosome: String, previous_position: u64) -> Self {
        Seeker {
            chromosome,
            previous_position,
        }
    }

    pub fn chromosome(&self) -> &str {
        &self.chromosome[..]
    }

    pub fn position(&self) -> u64 {
        self.previous_position
    }
}

pub struct Buf<T> {
    value: Option<T>,
}

impl<T> Buf<T> {
    // Yields a new empty buffer
    pub fn new() -> Self {
        Buf { value: None }
    }

    pub fn write(&mut self, new_value: T) {
        self.value = Some(new_value);
    }

    // reading from the buffer empties/truncates the buffer
    pub fn read(&mut self) -> Option<T> {
        self.value.take()
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_none()
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Buf
    #[test]
    fn test_buf_can_read_and_write() {
        let mut my_buffer: Buf<i32> = Buf::new();
        let val = 23;
        my_buffer.write(val);
        let read_val = my_buffer.read().unwrap();
        assert_eq!(val, read_val);
    }

    // Seeker
    //#[test]
    //#[ignore]
    fn test_seeker_holds_prev_position() {
        unimplemented!("Seeker tests")
    }
}
