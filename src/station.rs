use crate::attr::{Attrs, Nl80211Attr, Nl80211RateInfo, Nl80211StaInfo};

use neli::attr::Attribute;
use neli::err::DeError;

/// A struct representing a remote station (Access Point)
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Station {
    /// Station bssid (u8)
    pub bssid: Option<Vec<u8>>,
    pub inactive_time: Option<u32>,
    pub rx_bytes: Option<u64>,
    /// Total received packets (MSDUs and MMPDUs) from this station
    pub rx_packets: Option<u32>,
    pub tx_bytes: Option<u64>,
    /// Total transmitted packets (MSDUs and MMPDUs) to this station
    pub tx_packets: Option<u32>,
    /// Total retries (MPDUs) to this station
    pub tx_retries: Option<u32>,
    /// Total failed packets (MPDUs) to this station
    pub tx_failed: Option<u32>,
    pub beacon_rx: Option<u64>,
    /// Count of times beacon loss was detected
    pub beacon_loss: Option<u32>,
    pub rx_drop_misc: Option<u64>,
    /// Signal strength of last received PPDU (dBm)
    pub signal: Option<i8>,
    /// Signal strength average (dBm)
    pub average_signal: Option<i8>,
    pub beacon_signal_avg: Option<i8>,
    pub t_offset: Option<u64>,
    /// Transmission bitrate
    pub tx_bitrate: Option<u32>,
    /// Reception bitrate
    pub rx_bitrate: Option<u32>,
    pub rx_duration: Option<u64>,
    pub tx_duration: Option<u64>,
    pub ack_signal: Option<i8>,
    pub ack_signal_avg: Option<i8>,
    /// Time since the station is last connected in seconds
    pub connected_time: Option<u32>,
}

impl TryFrom<Attrs<'_, Nl80211Attr>> for Station {
    type Error = DeError;

    fn try_from(attrs: Attrs<'_, Nl80211Attr>) -> Result<Self, Self::Error> {
        let mut res = Self::default();
        if let Some(bssid) = attrs.get_attribute(Nl80211Attr::AttrMac) {
            res.bssid = Some(Vec::from(bssid.nla_payload.as_ref()));
        }

        if let Some(info) = attrs.get_attribute(Nl80211Attr::AttrStaInfo) {
            let attrs = info.get_attr_handle::<Nl80211StaInfo>().unwrap();
            for attr in attrs.iter() {
                match attr.nla_type.nla_type {
                    Nl80211StaInfo::StaInfoInactiveTime => {
                        res.inactive_time = Some(attr.get_payload_as()?)
                    }
                    Nl80211StaInfo::StaInfoRxBytes64 => res.rx_bytes = Some(attr.get_payload_as()?),
                    Nl80211StaInfo::StaInfoRxBytes => {
                        if res.rx_bytes.is_none() {
                            res.rx_bytes = Some(attr.get_payload_as::<u32>()? as u64)
                        }
                    }
                    Nl80211StaInfo::StaInfoRxPackets => {
                        res.rx_packets = Some(attr.get_payload_as()?)
                    }
                    Nl80211StaInfo::StaInfoTxBytes64 => res.tx_bytes = Some(attr.get_payload_as()?),
                    Nl80211StaInfo::StaInfoTxBytes => {
                        if res.tx_bytes.is_none() {
                            res.tx_bytes = Some(attr.get_payload_as::<u32>()? as u64)
                        }
                    }
                    Nl80211StaInfo::StaInfoTxPackets => {
                        res.tx_packets = Some(attr.get_payload_as()?)
                    }
                    Nl80211StaInfo::StaInfoTxRetries => {
                        res.tx_retries = Some(attr.get_payload_as()?)
                    }
                    Nl80211StaInfo::StaInfoTxFailed => res.tx_failed = Some(attr.get_payload_as()?),
                    Nl80211StaInfo::StaInfoBeaconLoss => {
                        res.beacon_loss = Some(attr.get_payload_as()?)
                    }
                    Nl80211StaInfo::StaInfoBeaconRx => res.beacon_rx = Some(attr.get_payload_as()?),
                    Nl80211StaInfo::StaInfoRxDropMisc => {
                        res.rx_drop_misc = Some(attr.get_payload_as()?)
                    }
                    Nl80211StaInfo::StaInfoSignal => res.signal = Some(attr.get_payload_as()?),
                    Nl80211StaInfo::StaInfoSignalAvg => {
                        res.average_signal = Some(attr.get_payload_as()?)
                    }
                    Nl80211StaInfo::StaInfoBeaconSignalAvg => {
                        res.beacon_signal_avg = Some(attr.get_payload_as()?)
                    }
                    Nl80211StaInfo::StaInfoTOffset => res.t_offset = Some(attr.get_payload_as()?),
                    Nl80211StaInfo::StaInfoTxBitrate => {
                        if let Some(rate) = attr
                            .get_attr_handle::<Nl80211RateInfo>()?
                            .get_attribute(Nl80211RateInfo::RateInfoBitrate32)
                        {
                            res.tx_bitrate = Some(rate.get_payload_as()?);
                        }
                    }
                    Nl80211StaInfo::StaInfoTxDuration => {
                        res.tx_duration = Some(attr.get_payload_as()?)
                    }
                    Nl80211StaInfo::StaInfoRxBitrate => {
                        if let Some(rate) = attr
                            .get_attr_handle::<Nl80211RateInfo>()?
                            .get_attribute(Nl80211RateInfo::RateInfoBitrate32)
                        {
                            res.rx_bitrate = Some(rate.get_payload_as()?);
                        }
                    }
                    Nl80211StaInfo::StaInfoRxDuration => {
                        res.rx_duration = Some(attr.get_payload_as()?)
                    }
                    Nl80211StaInfo::StaInfoAckSignal => {
                        res.ack_signal = Some(attr.get_payload_as()?)
                    }
                    Nl80211StaInfo::StaInfoAckSignalAvg => {
                        res.ack_signal_avg = Some(attr.get_payload_as()?)
                    }
                    Nl80211StaInfo::StaInfoConnectedTime => {
                        res.connected_time = Some(attr.get_payload_as()?)
                    }
                    _ => (),
                }
            }
        }
        Ok(res)
    }
}

#[cfg(test)]
mod tests_station {
    use super::*;
    use crate::attr::Nl80211Attr::AttrMac;
    use crate::attr::Nl80211Attr::AttrStaInfo;
    use neli::attr::AttrHandle;
    use neli::genl::{AttrType, Nlattr};
    use neli::types::Buffer;

    fn new_attr(t: Nl80211Attr, d: Vec<u8>) -> Nlattr<Nl80211Attr, Buffer> {
        Nlattr {
            nla_len: (4 + d.len()) as _,
            nla_type: AttrType {
                nla_nested: false,
                nla_network_order: true,
                nla_type: t,
            },
            nla_payload: d.into(),
        }
    }

    #[test]
    fn test_parser() {
        let handler = vec![
            new_attr(AttrMac, vec![46, 46, 46, 46, 46, 46]),
            new_attr(
                AttrStaInfo,
                vec![
                    8, 0, 16, 0, 17, 27, 0, 0, 8, 0, 1, 0, 248, 2, 0, 0, 8, 0, 2, 0, 43, 98, 156,
                    29, 8, 0, 3, 0, 99, 123, 109, 1, 12, 0, 23, 0, 43, 98, 156, 29, 0, 0, 0, 0, 12,
                    0, 24, 0, 99, 123, 109, 1, 0, 0, 0, 0, 5, 0, 7, 0, 218, 0, 0, 0, 5, 0, 13, 0,
                    215, 0, 0, 0, 20, 0, 25, 0, 5, 0, 0, 0, 216, 0, 0, 0, 5, 0, 1, 0, 213, 0, 0, 0,
                    20, 0, 26, 0, 5, 0, 0, 0, 212, 0, 0, 0, 5, 0, 1, 0, 211, 0, 0, 0, 28, 0, 8, 0,
                    8, 0, 5, 0, 16, 4, 0, 0, 6, 0, 1, 0, 16, 4, 0, 0, 5, 0, 2, 0, 13, 0, 0, 0, 28,
                    0, 14, 0, 8, 0, 5, 0, 134, 1, 0, 0, 6, 0, 1, 0, 134, 1, 0, 0, 5, 0, 2, 0, 4, 0,
                    0, 0, 8, 0, 9, 0, 226, 128, 7, 0, 8, 0, 10, 0, 9, 170, 2, 0, 8, 0, 11, 0, 27,
                    130, 0, 0, 8, 0, 12, 0, 47, 0, 0, 0, 8, 0, 27, 0, 196, 160, 0, 0, 8, 0, 18, 0,
                    0, 0, 0, 0, 28, 0, 15, 0, 4, 0, 2, 0, 4, 0, 3, 0, 5, 0, 4, 0, 1, 0, 0, 0, 6, 0,
                    5, 0, 100, 0, 0, 0, 12, 0, 17, 0, 254, 0, 0, 0, 170, 0, 0, 0, 12, 0, 28, 0,
                    183, 3, 0, 0, 0, 0, 0, 0, 12, 0, 29, 0, 225, 254, 0, 0, 0, 0, 0, 0, 5, 0, 30,
                    0, 216, 0, 0, 0, 5, 0, 34, 0, 46, 0, 0, 0, 56, 8, 31, 0, 128, 0, 1, 0, 12, 0,
                    1, 0, 168, 103, 5, 0, 0, 0, 0, 0, 12, 0, 2, 0, 71, 169, 2, 0, 0, 0, 0, 0, 12,
                    0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 76, 0, 6,
                    0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 61, 39, 1, 0, 8,
                    0, 4, 0, 23, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0, 0, 8, 0, 8,
                    0, 0, 0, 0, 0, 8, 0, 9, 0, 38, 56, 109, 1, 8, 0, 10, 0, 71, 169, 2, 0, 128, 0,
                    2, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 76,
                    0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 0, 0, 0,
                    0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0, 0, 8,
                    0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 0, 0, 0, 0, 8, 0, 10, 0, 0, 0, 0, 0, 128, 0,
                    3, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 76,
                    0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 0, 0, 0,
                    0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0, 0, 8,
                    0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 0, 0, 0, 0, 8, 0, 10, 0, 0, 0, 0, 0, 128, 0,
                    4, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 76,
                    0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 0, 0, 0,
                    0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0, 0, 8,
                    0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 0, 0, 0, 0, 8, 0, 10, 0, 0, 0, 0, 0, 128, 0,
                    5, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 76,
                    0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 0, 0, 0,
                    0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0, 0, 8,
                    0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 0, 0, 0, 0, 8, 0, 10, 0, 0, 0, 0, 0, 128, 0,
                    6, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 76,
                    0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 0, 0, 0,
                    0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0, 0, 8,
                    0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 0, 0, 0, 0, 8, 0, 10, 0, 0, 0, 0, 0, 128, 0,
                    7, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 180, 0, 0, 0, 0, 0, 0,
                    0, 12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    76, 0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 180,
                    0, 0, 0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0,
                    0, 8, 0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 115, 64, 0, 0, 8, 0, 10, 0, 180, 0, 0,
                    0, 128, 0, 8, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 2, 0, 0, 0,
                    0, 0, 0, 0, 12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 76, 0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0,
                    2, 0, 0, 0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0,
                    0, 0, 8, 0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 32, 1, 0, 0, 8, 0, 10, 0, 2, 0, 0, 0,
                    128, 0, 9, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 76, 0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 0,
                    0, 0, 0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0,
                    0, 8, 0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 0, 0, 0, 0, 8, 0, 10, 0, 0, 0, 0, 0,
                    128, 0, 10, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 76, 0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 0,
                    0, 0, 0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0,
                    0, 8, 0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 0, 0, 0, 0, 8, 0, 10, 0, 0, 0, 0, 0,
                    128, 0, 11, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 76, 0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 0,
                    0, 0, 0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0,
                    0, 8, 0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 0, 0, 0, 0, 8, 0, 10, 0, 0, 0, 0, 0,
                    128, 0, 12, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 76, 0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 0,
                    0, 0, 0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0,
                    0, 8, 0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 0, 0, 0, 0, 8, 0, 10, 0, 0, 0, 0, 0,
                    128, 0, 13, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 76, 0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 0,
                    0, 0, 0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0,
                    0, 8, 0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 0, 0, 0, 0, 8, 0, 10, 0, 0, 0, 0, 0,
                    128, 0, 14, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 76, 0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 0,
                    0, 0, 0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0,
                    0, 8, 0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 0, 0, 0, 0, 8, 0, 10, 0, 0, 0, 0, 0,
                    128, 0, 15, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 76, 0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 0,
                    0, 0, 0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0,
                    0, 8, 0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 0, 0, 0, 0, 8, 0, 10, 0, 0, 0, 0, 0,
                    128, 0, 16, 0, 12, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 76, 0, 6, 0, 8, 0, 1, 0, 0, 0, 0, 0, 8, 0, 2, 0, 0, 0, 0, 0, 8, 0, 3, 0, 0,
                    0, 0, 0, 8, 0, 4, 0, 0, 0, 0, 0, 8, 0, 5, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0, 0,
                    0, 8, 0, 8, 0, 0, 0, 0, 0, 8, 0, 9, 0, 0, 0, 0, 0, 8, 0, 10, 0, 0, 0, 0, 0, 52,
                    0, 17, 0, 12, 0, 1, 0, 109, 25, 0, 0, 0, 0, 0, 0, 12, 0, 2, 0, 4, 0, 0, 0, 0,
                    0, 0, 0, 12, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0,
                    0,
                ],
            ),
        ];

        let station: Station = AttrHandle::new(handler.into_iter().collect())
            .try_into()
            .unwrap();
        let expected_station = Station {
            average_signal: Some(i8::from_le_bytes([215])),
            beacon_loss: Some(u32::from_le_bytes([0, 0, 0, 0])),
            bssid: Some(vec![46, 46, 46, 46, 46, 46]),
            connected_time: Some(u32::from_le_bytes([17, 27, 0, 0])),
            rx_bitrate: Some(u32::from_le_bytes([134, 1, 0, 0])),
            rx_packets: Some(u32::from_le_bytes([226, 128, 7, 0])),
            signal: Some(i8::from_le_bytes([218])),
            tx_bitrate: Some(u32::from_le_bytes([16, 4, 0, 0])),
            tx_failed: Some(u32::from_le_bytes([47, 0, 0, 0])),
            tx_packets: Some(u32::from_le_bytes([9, 170, 2, 0])),
            tx_retries: Some(u32::from_le_bytes([27, 130, 0, 0])),
            ..Default::default()
        };

        assert_eq!(station, expected_station)
    }
}
