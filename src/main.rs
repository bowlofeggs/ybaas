/* Copyright © 2019 Igor Gnatenko and Randy Barlow

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, version 3 of the License.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.*/
//! # ybaas
//!
//! ```ybaas``` provides a web service that returns fake HOTP tokens so you can paste them into IRC.

use warp::{path, Filter};
use yubibomb::hotp;

/// Returns an HOTP token over GET requests.
///
/// # Examples
///
/// ```
/// $ curl http://localhost:3030/hotp
/// 720502
/// ```
#[tokio::main]
async fn main() {
    let token = path!("hotp").map(|| format!("{}", hotp()));

    warp::serve(token).run(([0, 0, 0, 0], 3030)).await;
}
