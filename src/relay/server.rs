// The MIT License (MIT)

// Copyright (c) 2014 Y. T. CHUNG <zonyitoo@gmail.com>

// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! Server side

use std::rc::Rc;
use std::io;

use tokio_core::reactor::Core;

use futures::Future;

use relay::udprelay::server::run as run_udp;
use relay::tcprelay::server::run as run_tcp;
use relay::dns_resolver::DnsResolver;
use config::Config;

/// Relay server running on server side.
///
/// ```no_run
/// use shadowsocks::relay::server::run;
/// use shadowsocks::config::{Config, ServerConfig};
/// use shadowsocks::crypto::CipherType;
///
/// let mut config = Config::new();
/// config.server = vec![
///     ServerConfig::basic("127.0.0.1:8388".parse().unwrap(),
///                         "server-password".to_string(),
///                         CipherType::Aes256Cfb)];
/// run(config).unwrap();
/// ```
///
pub fn run(config: Config) -> io::Result<()> {
    let mut lp = try!(Core::new());

    let handle = lp.handle();
    let config = Rc::new(config);

    let dns_resolver = DnsResolver::new(config.dns_cache_capacity);

    let tcp_fut = run_tcp(config.clone(), handle.clone(), dns_resolver.clone());

    if config.enable_udp {
        lp.run(tcp_fut.join(run_udp(config, handle, dns_resolver)).map(|_| ()))
    } else {
        lp.run(tcp_fut)
    }
}
