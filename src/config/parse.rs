use crate::{backend::*, constants::*, globals::*};
use hyper::Uri;
use std::{collections::HashMap, sync::Mutex};

// #[cfg(feature = "tls")]
use std::path::PathBuf;

pub fn parse_opts(globals: &mut Globals, backends: &mut HashMap<String, Backend>) {
  // TODO:
  globals.listen_sockets = LISTEN_ADDRESSES
    .to_vec()
    .iter()
    .flat_map(|x| {
      vec![
        format!("{}:{}", x, HTTP_LISTEN_PORT).parse().unwrap(),
        format!("{}:{}", x, HTTPS_LISTEN_PORT).parse().unwrap(),
      ]
    })
    .collect();
  globals.http_port = Some(HTTP_LISTEN_PORT);
  globals.https_port = Some(HTTPS_LISTEN_PORT);

  // TODO:
  let mut map_example: HashMap<String, Upstream> = HashMap::new();
  map_example.insert(
    "/maps".to_string(),
    Upstream {
      uri: vec![
        "https://www.bing.com".parse::<Uri>().unwrap(),
        "https://www.bing.co.jp".parse::<Uri>().unwrap(),
      ],
      cnt: Default::default(),
      lb: Default::default(),
    },
  );
  backends.insert(
    "localhost".to_string(),
    Backend {
      app_name: "Localhost to Google except for maps".to_string(),
      hostname: "localhost".to_string(),
      reverse_proxy: ReverseProxy {
        default_upstream: Upstream {
          uri: vec![
            "https://www.google.com".parse::<Uri>().unwrap(),
            "https://www.google.co.jp".parse::<Uri>().unwrap(),
          ],
          cnt: Default::default(),
          lb: Default::default(),
        },
        // default_upstream_uri: vec!["http://abehiroshi.la.coocan.jp/".parse::<Uri>().unwrap()], // httpのみの場合の好例
        upstream: map_example,
      },
      https_redirection: Some(false), // TODO: ここはtlsが存在する時はSomeにすべき。Noneはtlsがないときのみのはず

      tls_cert_path: Some(PathBuf::from(r"localhost1.pem")),
      tls_cert_key_path: Some(PathBuf::from(r"localhost1.pem")),
      server_config: Mutex::new(None),
    },
  );
}
