//! Error manipulation utilities.

use crate::ra_macro_util::WithError;
use syn::Error;

/// Collect errors.
pub fn collect_errs<const N: usize>(errors: [&dyn WithError; N]) -> Vec<&Error> {
    errors.iter().filter_map(|x| x.err()).collect::<Vec<_>>()
}

/// Combines multiple errors into one.
pub fn combine_errs<'a, I>(errors: I) -> Option<Error>
where
    I: IntoIterator<Item = &'a Error>,
{
    let mut ret = None;
    for err in errors.into_iter() {
        if ret.is_none() {
            ret = Some(err.clone());
            break;
        }

        ret.as_mut().unwrap().combine(err.clone());
    }

    ret
}
