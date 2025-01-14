// Copyright (c) 2001-2016, Alliance for Open Media. All rights reserved
// Copyright (c) 2017-2018, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use std::error::Error;

#[derive(Debug, Error)]
pub enum CliError {
  #[error(display = "{}: {}", msg, io)]
  Io { msg: String, io: std::io::Error },
  #[error(display = "{}: {:?}", msg, status)]
  Enc { msg: String, status: rav1e::EncoderStatus },
  #[error(display = "{}: {}", msg, status)]
  Config { msg: String, status: rav1e::InvalidConfig },
  #[error(display = "Cannot parse option `{}`: {}", opt, err)]
  ParseInt { opt: String, err: std::num::ParseIntError },
}

pub trait ToError {
  fn context(self, msg: &str) -> CliError;
}

impl ToError for std::io::Error {
  fn context(self, msg: &str) -> CliError {
    CliError::Io { msg: msg.to_owned(), io: self }
  }
}

impl ToError for rav1e::EncoderStatus {
  fn context(self, msg: &str) -> CliError {
    CliError::Enc { msg: msg.to_owned(), status: self }
  }
}

impl ToError for rav1e::InvalidConfig {
  fn context(self, msg: &str) -> CliError {
    CliError::Config { msg: msg.to_owned(), status: self }
  }
}

impl ToError for std::num::ParseIntError {
  fn context(self, opt: &str) -> CliError {
    CliError::ParseInt { opt: opt.to_lowercase(), err: self }
  }
}

pub fn print_error(e: &dyn Error) {
  error!("{}", e);
  let mut cause = e.source();
  while let Some(e) = cause {
    error!("Caused by: {}", e);
    cause = e.source();
  }
}
