# Copyright 2017 The Rust Project Developers. See the COPYRIGHT
# file at the top-level directory of this distribution and at
# http://rust-lang.org/COPYRIGHT.
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

# ignore-tidy-linelength

set -ex

echo Use sccache built with Azure Storage support
curl -fo /usr/local/bin/sccache \
  https://rustlangtools.blob.core.windows.net/public/stable-x86_64-unknown-linux-musl.sccache

chmod +x /usr/local/bin/sccache
