// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use std::io::{Error, ErrorKind};

#[derive(Debug, Error)]
pub enum BindingSignatureError {
    #[error("{}", _0)]
    AnyhowError(#[from] anyhow::Error),

    #[error("{}", _0)]
    CommitmentError(#[from] crate::CommitmentError),

    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("{}", _0)]
    Message(String),

    #[error("Affine point is not in the correct subgroup on curve {:?}", _0)]
    NotInCorrectSubgroupOnCurve(Vec<u8>),

    #[error("The value balance is out of bounds: {}", _0)]
    OutOfBounds(i64),
}

impl From<Error> for BindingSignatureError {
    fn from(error: Error) -> Self {
        BindingSignatureError::Crate("std::io", format!("{:?}", error))
    }
}

impl From<BindingSignatureError> for Error {
    fn from(error: BindingSignatureError) -> Error {
        Error::new(ErrorKind::Other, error.to_string())
    }
}
