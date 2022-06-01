// Copyright 2016 - 2018 Ulrik Sverdrup "bluss"
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//! architechture specific parameters

/// Columns in C, B that we handle at a time. (5th loop)
/// 
/// Cuts B into B0, B1, .. Bj, .. B_NC
pub const S_NC: usize = 1024;

/// Rows of Bj at a time (4th loop)
///
/// Columns of A at a time.
///
/// Cuts A into Ap
///
/// Cuts Bj into Bp, which is packed into B~.
///
/// Size of B~ is NC x KC
pub const S_KC: usize = 256;

/// Rows of Ap at a time. (3rd loop)
///
/// Cuts Ap into A0, A1, .., Ai, .. A_MC
///
/// Ai is packed into A~.
///
/// Size of A~ is KC x MC
pub const S_MC: usize = 64;

/// Columns in C, B that we handle at a time. (5th loop)
/// 
/// Cuts B into B0, B1, .. Bj, .. B_NC
pub const D_NC: usize = 1024;

/// Rows of Bj at a time (4th loop)
///
/// Columns of A at a time.
///
/// Cuts A into Ap
///
/// Cuts Bj into Bp, which is packed into B~.
///
/// Size of B~ is NC x KC
pub const D_KC: usize = 256;

/// Rows of Ap at a time. (3rd loop)
///
/// Cuts Ap into A0, A1, .., Ai, .. A_MC
///
/// Ai is packed into A~.
///
/// Size of A~ is KC x MC
pub const D_MC: usize = 64;
