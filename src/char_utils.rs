pub struct CharIter<'a> {
    iter: std::str::CharIndices<'a>,
    peek_pair: Option<(usize, char)>,
    total_bytes: usize,
}

impl<'a> CharIter<'a> {
    pub fn new(input: &'a str) -> Self {
        CharIter {
            iter: input.char_indices(),
            peek_pair: None,
            total_bytes: input.len(),
        }
    }

    fn fill_peek(&mut self) {
        if self.peek_pair.is_none() {
            if let Some(p) = self.iter.next() {
                self.peek_pair = Some(p);
            }
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        self.fill_peek();
        Some(self.peek_pair?.1)
    }

    pub fn peek_position(&mut self) -> usize {
        self.fill_peek();
        match self.peek_pair {
            Some((p, _)) => p,
            None => self.total_bytes,
        }
    }

    pub fn consume(&mut self) -> Option<char> {
        match self.peek_pair {
            Some((_, c)) => {
                self.peek_pair = None;
                Some(c)
            }
            None => self.iter.next().map(|(_, c)| c),
        }
    }
}
