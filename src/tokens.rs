//! Since [std::iter::Peekable] in Rust advances the iterator,
//! I can't use it for peeking tokens.
//! I haven't found a better way than implementing a wrapper
//! that allows me to peek...

use std::vec::IntoIter;

use crate::{
    error::{Error, ErrorKind, Result},
    lexer::{Token, TokenKind},
    parser::{Ident, ParserCtx},
};

#[derive(Debug)]
pub struct Tokens {
    pub peeked: Option<Token>,
    inner: IntoIter<Token>,
}

impl Tokens {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            peeked: None,
            inner: tokens.into_iter(),
        }
    }

    /// Peeks into the next token without advancing the iterator.
    pub fn peek(&mut self) -> Option<Token> {
        // something in the peeked
        if let Some(token) = &self.peeked {
            Some(token.clone())
        } else {
            // otherwise get from iterator and store in peeked
            let token = self.inner.next();
            self.peeked = token.clone();
            token
        }
    }

    /// Like next() except that it also stores the last seen token in the given context
    /// (useful for debugging)
    pub fn bump(&mut self, ctx: &mut ParserCtx) -> Option<Token> {
        if let Some(token) = self.peeked.take() {
            ctx.last_token = Some(token.clone());
            Some(token)
        } else {
            let token = self.inner.next();
            if token.is_some() {
                ctx.last_token = token.clone();
            }
            token
        }
    }
    /// Like [Self::bump] but errors with `err` pointing to the latest token
    pub fn bump_err(&mut self, ctx: &mut ParserCtx, err: ErrorKind) -> Result<Token> {
        self.bump(ctx).ok_or(Error {
            kind: err,
            span: ctx.last_span(),
        })
    }

    /// Like [Self::bump] but errors if the token is not `typ`
    pub fn bump_expected(&mut self, ctx: &mut ParserCtx, typ: TokenKind) -> Result<Token> {
        let token = self.bump_err(ctx, ErrorKind::MissingToken)?;
        if token.kind == typ {
            Ok(token)
        } else {
            Err(Error {
                kind: ErrorKind::ExpectedToken(typ),
                span: ctx.last_span(),
            })
        }
    }

    pub fn bump_ident(&mut self, ctx: &mut ParserCtx, kind: ErrorKind) -> Result<Ident> {
        let token = self.bump_err(ctx, kind.clone())?;
        let span = token.span;
        match &token.kind {
            TokenKind::Identifier(value) => Ok(Ident {
                value: value.to_string(),
                span,
            }),
            _ => Err(Error { kind, span }),
        }
    }
}
