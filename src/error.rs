use std::num::TryFromIntError;

use thiserror::Error;

use crate::parser::Relative;

pub type Result<T> = std::result::Result<T, OrgModeDateTimeError>;

#[derive(Error, Debug)]
pub enum OrgModeDateTimeError {
    #[error("Int conversion failed")]
    TryFromIntError(#[from] TryFromIntError),
    #[error("Unrepresentable past date")]
    UnrepresentablePastRelativeDate(Relative),
    #[error("Unrepresentable future date")]
    UnrepresentableFutureRelativeDate(Relative),
}
