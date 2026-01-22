// Copyright 2022 Mandiant, Inc. All Rights Reserved
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed under the License
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and limitations under the License.

use std::rc::Rc;

use uuid::Uuid;

use crate::{
    RcString,
    chunks::firehose::firehose_log::FirehoseItemInfo,
    decoders::{
        DecoderError,
        bool::{lowercase_bool, uppercase_bool},
        darwin::{errno_codes, format_permission, permission},
        dns::{
            dns_acceptable, dns_addrmv, dns_counts, dns_getaddrinfo_opts, dns_idflags, dns_ip_addr,
            dns_protocol, dns_reason, dns_records, dns_yes_no, get_dns_mac_addr, get_domain_name,
            get_service_binding, parse_dns_header,
        },
        location::{
            client_authorization_status, client_manager_state_tracker_state, daemon_status_type,
            io_message, location_manager_state_tracker_state, sqlite_location,
            subharvester_identifier,
        },
        network::{ipv_four, ipv_six, sockaddr},
        opendirectory::{errors, member_details, member_id_type, sid_details},
        time::parse_time,
        uuid::parse_uuid,
    },
    rc_string,
    util::format_uuid,
};

pub enum Decoded {
    Other(RcString), // Default value for work in progress, to be removed when all other types are implemented
    Error(RcString),
    Masked(RcString),
    UpBool(bool),
    LoBool(bool),
    Uuid(Uuid),
    Errno(&'static str),
    Permission(u8, u8, u8),
}

impl Decoded {
    pub fn to_rc_string(&self) -> RcString {
        match self {
            Self::Other(value) | Self::Error(value) | Self::Masked(value) => value.clone(),
            Self::UpBool(value) => rc_string!(value.then(|| "NO").unwrap_or("YES")),
            Self::LoBool(value) => rc_string!(value.then(|| "false").unwrap_or("true")),
            Self::Uuid(value) => rc_string!(format_uuid(*value)),
            Self::Errno(value) => rc_string!(*value),
            Self::Permission(user, owner, group) => {
                rc_string!(format_permission(*user, *owner, *group))
            }
        }
    }
}

/// Check if we support one of Apple's custom logging objects
pub(crate) fn check_objects(
    format_string: &str,
    message_values: &[FirehoseItemInfo],
    item_type: u8,
    item_index: usize,
) -> Decoded {
    let mut index = item_index;
    const PRECISION_ITEM: u8 = 0x12;

    // Increment index get the actual firehose item data
    if item_type == PRECISION_ITEM {
        index += 1;
        if index > message_values.len() {
            return Decoded::Error(rc_string!(format!(
                "Index out of bounds for FirehoseItemInfo Vec. Got adjusted index {}, Vec size is {}. This should not have happened",
                index,
                message_values.len()
            )));
        }
    }

    const MASKED_HASH_TYPE: u8 = 0xf2;
    // Check if the log value is hashed or marked private
    if (format_string.contains("mask.hash") && message_values[index].item_type == MASKED_HASH_TYPE)
        || message_values[index].message_strings.as_str() == "<private>"
    {
        return Decoded::Masked(message_values[index].message_strings.clone());
    }

    let message_strings = message_values[index].message_strings.as_str();

    // Check if log value contains one the supported decoders
    let message_value = to_decoded_value(format_string, message_strings);

    match message_value {
        Ok(value) => value,
        Err(e) => {
            log::error!("[macos-unifiedlogs] Failed to decode log object. Error: {e:?}");
            Decoded::Error(rc_string!(format!("Decoder error: {e:?}")))
        }
    }
}

fn to_decoded_value<'a>(
    format_string: &'a str,
    message_strings: &'a str,
) -> Result<Decoded, DecoderError<'a>> {
    let decoded = if format_string.contains("BOOL") {
        Decoded::UpBool(message_strings == "0")
    } else if format_string.contains("bool") {
        Decoded::LoBool(message_strings == "0")
    } else if format_string.contains("uuid_t") {
        Decoded::Uuid(parse_uuid(&message_strings)?)
    } else if format_string.contains("darwin.errno") {
        Decoded::Errno(errno_codes(&message_strings))
    } else if format_string.contains("darwin.mode") {
        permission(&message_strings)
    } else {
        let ok = if format_string.contains("odtypes:ODError") {
            errors(&message_strings)
        } else if format_string.contains("odtypes:mbridtype") {
            member_id_type(&message_strings)
        } else if format_string.contains("odtypes:mbr_details") {
            member_details(&message_strings)?
        } else if format_string.contains("odtypes:nt_sid_t") {
            sid_details(&message_strings)?
        } else if format_string.contains("location:CLClientAuthorizationStatus") {
            client_authorization_status(&message_strings)?
        } else if format_string.contains("location:CLDaemonStatus_Type::Reachability") {
            daemon_status_type(&message_strings)?
        } else if format_string.contains("location:CLSubHarvesterIdentifier") {
            subharvester_identifier(&message_strings)?
        } else if format_string.contains("location:SqliteResult") {
            sqlite_location(&message_strings)?.to_string()
        } else if format_string.contains("location:_CLClientManagerStateTrackerState") {
            client_manager_state_tracker_state(&message_strings)?
        } else if format_string.contains("location:_CLLocationManagerStateTrackerState") {
            location_manager_state_tracker_state(&message_strings)?
        } else if format_string.contains("network:in6_addr") {
            ipv_six(&message_strings).map(|ip| ip.to_string())?
        } else if format_string.contains("network:in_addr") {
            ipv_four(&message_strings).map(|ip| ip.to_string())?
        } else if format_string.contains("network:sockaddr") {
            sockaddr(&message_strings)?
        } else if format_string.contains("time_t") {
            parse_time(&message_strings)?
        } else if format_string.contains("mdns:dnshdr") {
            parse_dns_header(&message_strings)?
        } else if format_string.contains("mdns:rd.svcb") {
            get_service_binding(&message_strings)?
        } else if format_string.contains("location:IOMessage") {
            io_message(&message_strings)?.to_string()
        } else if format_string.contains("mdnsresponder:domain_name") {
            get_domain_name(&message_strings)?
        } else if format_string.contains("mdnsresponder:mac_addr") {
            get_dns_mac_addr(&message_strings)?
        } else if format_string.contains("mdnsresponder:ip_addr") {
            dns_ip_addr(&message_strings)?
        } else if format_string.contains("mdns:addrmv") {
            dns_addrmv(&message_strings)
        } else if format_string.contains("mdns:rrtype") {
            dns_records(&message_strings)?.to_string()
        } else if format_string.contains("mdns:nreason") {
            dns_reason(&message_strings)?.to_string()
        } else if format_string.contains("mdns:protocol") {
            dns_protocol(&message_strings)?.to_string()
        } else if format_string.contains("mdns:dns.idflags") {
            dns_idflags(&message_strings)?
        } else if format_string.contains("mdns:dns.counts") {
            dns_counts(&message_strings)?.to_string()
        } else if format_string.contains("mdns:yesno") {
            dns_yes_no(&message_strings)
        } else if format_string.contains("mdns:acceptable") {
            dns_acceptable(&message_strings)
        } else if format_string.contains("mdns:gaiopts") {
            dns_getaddrinfo_opts(&message_strings)?.to_string()
        } else {
            String::new()
        };

        Decoded::Other(rc_string!(ok))
    };

    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use super::check_objects;
    use super::*;
    use crate::chunks::firehose::firehose_log::FirehoseItemInfo;

    #[test]
    fn test_check_objects_lowercase_bool() {
        let test_format = "%{bool}d";
        let test_item_info = FirehoseItemInfo {
            message_strings: rc_string!("1"),
            item_type: 0,
            item_size: 4,
        };
        let test_type = 0;
        let test_index = 0;

        let results = check_objects(test_format, &[test_item_info], test_type, test_index);
        assert_eq!(results.to_rc_string().as_str(), "true")
    }

    #[test]
    fn test_check_objects_uppercase_bool() {
        let test_format = "%{BOOL}d";
        let test_item_info = FirehoseItemInfo {
            message_strings: rc_string!("1"),
            item_type: 0,
            item_size: 4,
        };
        let test_type = 0;
        let test_index = 0;

        let results = check_objects(test_format, &[test_item_info], test_type, test_index);
        assert_eq!(results.to_rc_string().as_str(), "YES")
    }

    #[test]
    fn test_odtypes() {
        let test_format = "%{odtypes:mbr_details}d";
        let test_item_info = FirehoseItemInfo {
            message_strings: rc_string!("I/7///8vTG9jYWwvRGVmYXVsdAA="),
            item_type: 50,
            item_size: 0,
        };
        let test_type = 50; // 0x32
        let test_index = 0;

        let results = check_objects(test_format, &[test_item_info], test_type, test_index);
        assert_eq!(results.to_rc_string().as_str(), "user: -2@/Local/Default");
    }

    #[test]
    fn test_check_objects_uuid() {
        let test_format = "%{public,uuid_t}.16P";
        let test_item_info = FirehoseItemInfo {
            message_strings: rc_string!("hZV+HTbETtKGqAZXvN3ikw=="),
            item_type: 50,
            item_size: 16,
        };
        let test_type = 50; // 0x32
        let test_index = 0;

        let results = check_objects(test_format, &[test_item_info], test_type, test_index);
        assert_eq!(
            results.to_rc_string().as_str(),
            "85957E1D36C44ED286A80657BCDDE293"
        )
    }

    #[test]
    fn test_private() {
        let test_format = "%{public,uuid_t}.16P";
        let test_item_info = FirehoseItemInfo {
            message_strings: rc_string!("<private>"),
            item_type: 50,
            item_size: 16,
        };
        let test_type = 50; // 0x32
        let test_index = 0;

        let results = check_objects(test_format, &[test_item_info], test_type, test_index);
        assert_eq!(results.to_rc_string().as_str(), "<private>")
    }

    #[test]
    fn test_hash() {
        let test_format = "%{public,mask.hash}.16P";
        let test_item_info = FirehoseItemInfo {
            message_strings: rc_string!("hash"),
            item_type: 242,
            item_size: 16,
        };
        let test_type = 242; // 0x32
        let test_index = 0;

        let results = check_objects(test_format, &[test_item_info], test_type, test_index);
        assert_eq!(results.to_rc_string().as_str(), "hash")
    }
}
