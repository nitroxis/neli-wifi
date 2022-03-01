use crate::attr::Nl80211Attr;
use crate::bss::Bss;
use crate::cmd::Nl80211Cmd;
use crate::interface::Interface;
use crate::station::Station;
use crate::{NL_80211_GENL_NAME, NL_80211_GENL_VERSION};

use neli::consts::genl::{CtrlAttr, CtrlCmd};
use neli::consts::{nl::GenlId, nl::NlmF, nl::NlmFFlags, nl::Nlmsg, socket::NlFamily};
use neli::err::NlError;
use neli::genl::{Genlmsghdr, Nlattr};
use neli::nl::{NlPayload, Nlmsghdr};
use neli::socket::NlSocketHandle;
use neli::types::GenlBuffer;

/// A generic netlink socket to send commands and receive messages
pub struct Socket {
    sock: NlSocketHandle,
    family_id: u16,
}

impl Socket {
    /// Create a new nl80211 socket with netlink
    pub fn connect() -> Result<Self, NlError<GenlId, Genlmsghdr<CtrlCmd, CtrlAttr>>> {
        let mut sock = NlSocketHandle::connect(NlFamily::Generic, None, &[])?;
        let family_id = sock.resolve_genl_family(NL_80211_GENL_NAME)?;
        Ok(Self { sock, family_id })
    }

    /// Get information for all your wifi interfaces
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use neli_wifi::Socket;
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>>{
    ///     let wifi_interfaces = Socket::connect()?.get_interfaces_info();
    ///     for wifi_interface in wifi_interfaces? {
    ///         println!("{:#?}", wifi_interface);
    ///     }
    /// #   Ok(())
    /// # }
    ///```
    pub fn get_interfaces_info(&mut self) -> Result<Vec<Interface>, NlError> {
        let msghdr = Genlmsghdr::<Nl80211Cmd, Nl80211Attr>::new(
            Nl80211Cmd::CmdGetInterface,
            NL_80211_GENL_VERSION,
            GenlBuffer::new(),
        );

        let nlhdr = {
            let len = None;
            let nl_type = self.family_id;
            let flags = NlmFFlags::new(&[NlmF::Request, NlmF::Dump]);
            let seq = None;
            let pid = None;
            let payload = NlPayload::Payload(msghdr);
            Nlmsghdr::new(len, nl_type, flags, seq, pid, payload)
        };

        self.sock.send(nlhdr)?;

        let iter = self
            .sock
            .iter::<Nlmsg, Genlmsghdr<Nl80211Cmd, Nl80211Attr>>(false);
        let mut interfaces = Vec::new();
        for response in iter {
            let response = response.unwrap();
            match response.nl_type {
                Nlmsg::Noop => (),
                Nlmsg::Error => panic!("Error"),
                Nlmsg::Done => break,
                _ => {
                    let handle = response.nl_payload.get_payload().unwrap().get_attr_handle();
                    interfaces.push(handle.try_into()?);
                }
            };
        }

        Ok(interfaces)
    }

    /// Get access point information for a specific interface
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use neli_wifi::Socket;
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>>{
    ///   // First of all we need to get wifi interface information to get more data
    ///   let wifi_interfaces = Socket::connect()?.get_interfaces_info();
    ///   for wifi_interface in wifi_interfaces? {
    ///     if let Some(netlink_index) = wifi_interface.index {
    ///
    ///       // Then for each wifi interface we can fetch station information
    ///       let station_info = Socket::connect()?.get_station_info(&netlink_index.clone())?;
    ///           println!("{:#?}", station_info);
    ///       }
    ///     }
    /// #   Ok(())
    /// # }
    ///```
    pub fn get_station_info(&mut self, interface_attr_if_index: &[u8]) -> Result<Station, NlError> {
        let msghdr = Genlmsghdr::<Nl80211Cmd, Nl80211Attr>::new(
            Nl80211Cmd::CmdGetStation,
            NL_80211_GENL_VERSION,
            {
                let mut attrs = GenlBuffer::new();
                attrs.push(
                    Nlattr::new(
                        false,
                        false,
                        Nl80211Attr::AttrIfindex,
                        NlPayload::<(), Vec<u8>>::Payload(interface_attr_if_index.to_owned()),
                    )
                    .unwrap(),
                );
                attrs
            },
        );

        let nlhdr = {
            let len = None;
            let nl_type = self.family_id;
            let flags = NlmFFlags::new(&[NlmF::Request, NlmF::Dump]);
            let seq = None;
            let pid = None;
            let payload = NlPayload::Payload(msghdr);
            Nlmsghdr::new(len, nl_type, flags, seq, pid, payload)
        };

        self.sock.send(nlhdr)?;

        let iter = self
            .sock
            .iter::<Nlmsg, Genlmsghdr<Nl80211Cmd, Nl80211Attr>>(false);
        for response in iter {
            let response = response.unwrap();
            match response.nl_type {
                Nlmsg::Noop => (),
                Nlmsg::Error => panic!("Error"),
                Nlmsg::Done => break,
                _ => {
                    let handle = response.nl_payload.get_payload().unwrap().get_attr_handle();
                    return Ok(handle.try_into()?);
                }
            };
        }

        Ok(Station::default())
    }

    pub fn get_bss_info(&mut self, interface_attr_if_index: &[u8]) -> Result<Bss, NlError> {
        let msghdr = Genlmsghdr::<Nl80211Cmd, Nl80211Attr>::new(
            Nl80211Cmd::CmdGetScan,
            NL_80211_GENL_VERSION,
            {
                let mut attrs = GenlBuffer::new();
                attrs.push(
                    Nlattr::new(
                        false,
                        false,
                        Nl80211Attr::AttrIfindex,
                        NlPayload::<(), Vec<u8>>::Payload(interface_attr_if_index.to_owned()),
                    )
                    .unwrap(),
                );
                attrs
            },
        );

        let nlhdr = {
            let len = None;
            let nl_type = self.family_id;
            let flags = NlmFFlags::new(&[NlmF::Request, NlmF::Dump]);
            let seq = None;
            let pid = None;
            let payload = NlPayload::Payload(msghdr);
            Nlmsghdr::new(len, nl_type, flags, seq, pid, payload)
        };

        self.sock.send(nlhdr)?;

        let iter = self
            .sock
            .iter::<Nlmsg, Genlmsghdr<Nl80211Cmd, Nl80211Attr>>(false);
        for response in iter {
            let response = response.unwrap();
            match response.nl_type {
                Nlmsg::Noop => (),
                Nlmsg::Error => panic!("Error"),
                Nlmsg::Done => break,
                _ => {
                    let handle = response.nl_payload.get_payload().unwrap().get_attr_handle();
                    return Ok(handle.try_into()?);
                }
            }
        }
        Ok(Bss::default())
    }
}

impl From<Socket> for NlSocketHandle {
    /// Returns the underlying generic netlink socket
    fn from(sock: Socket) -> Self {
        sock.sock
    }
}
