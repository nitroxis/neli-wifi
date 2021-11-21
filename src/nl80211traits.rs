use crate::attr::{Attrs, Nl80211Attr};
use neli::err::DeError;

/// Parse netlink messages attributes returned by a nl80211 command
pub trait ParseNlAttr: Sized {
    fn parse(attrs: Attrs<'_, Nl80211Attr>) -> Result<Self, DeError>;
}
