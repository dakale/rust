// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// compile-pass
// pretty-expanded FIXME #23616

pub trait Promisable: Send + Sync {}
impl<T: Send + Sync> Promisable for T {}

pub fn propagate<'a, T, E, F, G>(mut action: F)
    -> Box<FnMut(Result<T, E>) -> Result<T, E> + 'a>
    where
        T: Promisable + Clone + 'a,
        E: Promisable + Clone + 'a,
        F: FnMut(&T) -> Result<T, E> + Send + 'a,
        G: FnMut(Result<T, E>) -> Result<T, E> + 'a {
    Box::new(move |result: Result<T, E>| {
        match result {
            Ok(ref t) => action(t),
            Err(ref e) => Err(e.clone()),
        }
    })
}

fn main() {}
