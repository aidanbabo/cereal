use std::marker::PhantomData;

#[derive(Clone, Copy, Debug)]
pub struct Span<'src> {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    _phantom: PhantomData<&'src ()>,
}

impl<'src> Span<'src> {
    pub fn new(_s: &'src str, start: usize, end: usize, line: usize) -> Self {
        Span {
            start,
            end,
            line,
            _phantom: PhantomData,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Spanned<'src, T> {
    t: T,
    pub span: Span<'src>,
}

impl<'src, T> Spanned<'src, T> {
    pub fn new(t: T, span: Span<'src>) -> Self {
        Spanned { t, span }
    }
}

use std::ops;
impl<'src, T> ops::Deref for Spanned<'src, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.t
    }
}

impl<'src, T> ops::DerefMut for Spanned<'src, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.t
    }
}

pub type S<'src, T> = Spanned<'src, T>;

pub trait Spannable<'src> {
    fn spanned(self, span: Span<'src>) -> Spanned<'src, Self>
    where
        Self: Sized;
}

impl<'src, T> Spannable<'src> for T {
    fn spanned(self, span: Span<'src>) -> Spanned<'src, T> {
        Spanned::new(self, span)
    }
}
