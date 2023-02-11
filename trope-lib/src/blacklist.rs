use crate::Namespace;

pub const PAGE_BLACKLIST: [(Namespace, &str); 6] = [
  (Namespace::Main, "CircularLoop"),
  (Namespace::Main, "CircularRedirect"),
  (Namespace::Main, "InfiniteLoopRedirect"),
  (Namespace::Main, "ThisPageRedirectsToItself"),
  (Namespace::Main, "RedirectLoop"),
  (Namespace::Main, "RedirectBack"),
];
