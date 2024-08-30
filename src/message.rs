// Copyright 2022 Mandiant, Inc. All Rights Reserved
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed under the License
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and limitations under the License.

use std::mem::size_of;

use crate::chunks::firehose::firehose_log::FirehoseItemInfo;
use crate::decoders::decoder;
use log::{error, info, warn};
use nom::branch::alt;
use nom::bytes::complete::{is_a, is_not, take, take_until};
use nom::character::complete::digit0;

struct FormatAndMessage {
    formatter: String,
    message: String,
}

const FLOAT_TYPES: [&str; 6] = ["f", "F", "e", "E", "g", "G"];
const INT_TYPES: [&str; 4] = ["d", "D", "i", "u"];
const HEX_TYPES: [&str; 5] = ["x", "X", "a", "A", "p"];
const OCTAL_TYPES: [&str; 2] = ["o", "O"];
const ERROR_TYPES: [&str; 1] = ["m"];
const STRING_TYPES: [&str; 6] = ["c", "s", "@", "S", "C", "P"];

/// Format the Unified Log message entry based on the parsed log items. Formatting follows the C lang prinf formatting process
pub fn format_firehose_log_message(
    format_string: String,
    item_message: &Vec<FirehoseItemInfo>,
) -> String {
    let mut log_message = format_string;
    let mut format_and_message_vec: Vec<FormatAndMessage> = Vec::new();
    info!("Unified log base message: {:?}", log_message);
    info!("Unified log entry strings: {:?}", item_message);

    // Some log entries may be completely empty (no format string or message data)
    /*
        tp 1976 + 32:       log default (shared_cache, has_subsystem)
        thread:         0000000000000d8f
        time:           +56.919s
        walltime:       1642303846 - 2022-01-15 19:30:46 (Saturday)
        location:       pc:0x1bd34bda fmt:0x1bde2be0
        image uuid:     4CA1B500-20EF-3E30-B63B-3E3579524A7F
        image path:     /System/Library/PrivateFrameworks/TelephonyUtilities.framework/Versions/A/TelephonyUtilities
        format:
        subsystem:      156 com.apple.calls.telephonyutilities.Default
    */
    if log_message.is_empty() && item_message.is_empty() {
        return String::new();
    }
    if log_message.is_empty() {
        return item_message[0].message_strings.to_owned();
    }

    let mut item_index: usize;
    let mut item_at_index: &FirehoseItemInfo;

    // auto update index & dependent stuff at the same time
    // assert that the index is valid & return if not
    macro_rules! update_index_or {
        ( $i:expr, $or:block ) => {
            item_index = $i;
            if let Some(msg) = item_message.get(item_index) {
                #[allow(unused_assignments)]
                {
                    item_at_index = msg
                }
            } else {
                $or
            }
        };
    }

    let hope_this_dummy_value_wont_be_used = FirehoseItemInfo {
        item_type: 0,
        item_size: 0,
        message_strings: String::new(),
    };

    update_index_or!(0, {
        item_at_index = &hope_this_dummy_value_wont_be_used;
    });

    let mut log_message_vec: Vec<String> = Vec::new();
    for values in format_and_message_vec {
        // Split the values by printf formatter
        // We have to do this instead of using replace because our replacement string may also contain a printf formatter
        let message_results = log_message.split_once(&values.formatter);
        match message_results {
            Some((message_part, remaining_message)) => {
                log_message_vec.push(message_part.to_string());
                log_message_vec.push(values.message);
                log_message = remaining_message.to_string();
            }
            None => error!(
                "Failed to split log message ({}) by printf formatter: {}",
                log_message, &values.formatter
            ),
        }
    }
    log_message_vec.push(log_message);
    log_message_vec.join("")
}

// Format strings are based on C printf formats. Parse format specification
fn parse_formatter<'a>(
    formatter: &'a str,
    message_value: &'a [FirehoseItemInfo],
    item_type: &'a u8,
    item_index: usize,
) -> nom::IResult<&'a str, String> {
    let mut index: usize;
    let mut message_at_index: &FirehoseItemInfo;

    // auto update index & dependent stuff at the same time
    // assert that the index is valid & return if not
    macro_rules! update_index {
        ( $i:expr ) => {
            index = $i;
            if let Some(msg) = message_value.get(index) {
                message_at_index = msg
            } else {
                return Ok(("", String::from("UNABLE TO PARSE MESSAGE")));
            }
        };
    }
    update_index!(item_index);

    let precision_items = [0x10, 0x12];
    let mut precision_value = 0;
    if precision_items.contains(item_type) {
        precision_value = message_at_index.item_size as usize;
        update_index!(index + 1);
    }

    let mut message = message_at_index.message_strings.to_owned();

    let number_item_type: Vec<u8> = vec![0x0, 0x1, 0x2];

    // If the message formatter is expects a string/character and the message string is a number type
    // Try to convert to a character/string
    if formatter.to_lowercase().ends_with('c')
        && number_item_type.contains(&message_at_index.item_type)
    {
        let char_results = message_at_index.message_strings.parse::<u32>();
        match char_results {
            Ok(char_message) => message = (char_message as u8 as char).to_string(),
            Err(err) => {
                error!(
                    "[macos-unifiedlogs] Failed to parse number item to char string: {:?}",
                    err
                );
                return Ok((
                    "",
                    String::from("Failed to parse number item to char string"),
                ));
            }
        }
    }

    let mut left_justify = false;
    //let mut space_value = false;
    let mut hashtag = false;
    let mut pad_zero = false;
    let mut plus_minus = false;
    let mut width_index = 1;
    //let mut has_apostrophe = false;
    for (index, format_values) in formatter.chars().enumerate() {
        if index == 0 {
            continue;
        }

        match format_values {
            '-' => left_justify = true,
            '+' => plus_minus = true,
            //' ' => space_value = true,
            '#' => hashtag = true,
            '0' => pad_zero = true,
            // '\'' => has_apostrophe = true,
            _ => {
                width_index = index;
                break;
            }
        }
    }

    let mut formatter_message = &formatter[width_index..];
    let (input, mut width) = digit0(formatter_message)?;
    formatter_message = input;
    let width_value;

    if formatter_message.starts_with('*') {
        // Also seen number type value 0 used for dynamic width/precision value
        let dynamic_precision_value = 0x0;
        if item_type == &dynamic_precision_value && message_at_index.item_size == 0 {
            precision_value = message_at_index.item_size as usize;
            update_index!(index + 1);
            message = message_at_index.message_strings.to_owned();
        }

        width_value = format!("{}", precision_value);
        width = width_value.as_str();
        let (input, _) = take(size_of::<u8>())(formatter_message)?;
        formatter_message = input;
    }

    if formatter_message.starts_with('.') {
        let (input, _) = is_a(".")(formatter_message)?;
        let (input, precision_data) = is_not("hljzZtqLdDiuUoOcCxXfFeEgGaASspPn%@")(input)?;
        if precision_data != "*" {
            let precision_results = precision_data.parse::<usize>();
            match precision_results {
                Ok(value) => precision_value = value,
                Err(err) => error!(
                    "[macos-unifiedlogs] Failed to parse format precision value: {:?}",
                    err
                ),
            }
        } else if precision_value != 0 {
            // For dynamic length use the length of the message string
            precision_value = message_value.len();
        }
        formatter_message = input;
    }

    // Get Length data if it exists or get the type format
    let (input, length_data) =
        alt((is_a("hlwIztq"), is_a("cmCdiouxXeEfgGaAnpsSZP@")))(formatter_message)?;
    formatter_message = input;

    let mut type_data = length_data;
    let length_values = ["h", "hh", "l", "ll", "w", "I", "z", "t", "q"];
    if length_values.contains(&length_data) {
        let (_, type_format) = is_a("cmCdiouxXeEfgGaAnpsSZP@")(formatter_message)?;
        type_data = type_format;
    }

    // Error types map error code to error message string. Currently not mapping to error message string
    // Ex: open on %s: %m
    //    "open on /var/folders: No such file or directory"
    // "No such file or directory" is error code 2
    if ERROR_TYPES.contains(&type_data) {
        message = format!("Error code: {}", message);
        return Ok(("", message));
    }

    if !width.is_empty() {
        let mut width_value = 0;
        let width_results = width.parse::<usize>();
        match width_results {
            Ok(value) => width_value = value,
            Err(err) => error!(
                "[macos-unifiedlogs] Failed to parse format width value: {:?}",
                err
            ),
        }
        if pad_zero {
            // Pad using zeros instead of spaces
            if left_justify {
                message = format_alignment_left(
                    message,
                    width_value,
                    precision_value,
                    type_data,
                    plus_minus,
                    hashtag,
                )
            } else {
                message = format_alignment_right(
                    message,
                    width_value,
                    precision_value,
                    type_data,
                    plus_minus,
                    hashtag,
                )
            }
        } else {
            // Pad spaces instead of zeros
            if left_justify {
                message = format_alignment_left_space(
                    message,
                    width_value,
                    precision_value,
                    type_data,
                    plus_minus,
                    hashtag,
                )
            } else {
                message = format_alignment_right_space(
                    message,
                    width_value,
                    precision_value,
                    type_data,
                    plus_minus,
                    hashtag,
                )
            }
        }
    } else if left_justify {
        message = format_left(message, precision_value, type_data, plus_minus, hashtag)
    } else {
        message = format_right(message, precision_value, type_data, plus_minus, hashtag);
    }

    Ok(("", message))
}

// Function to parse formatters containing types. Ex: %{errno}d, %{public}s, %{private}s, %{sensitive}
fn parse_type_formatter<'a>(
    formatter: &'a str,
    message_value: &'a [FirehoseItemInfo],
    item_type: &'a u8,
    item_index: usize,
) -> nom::IResult<&'a str, String> {
    let (format, format_type) = take_until("}")(formatter)?;

    let apple_object = decoder::check_objects(format_type, message_value, item_type, item_index);

    // If we successfully decoded an apple object, then there is nothing to format.
    // Signpost entries have not been seen with custom objects
    if !apple_object.is_empty() {
        return Ok(("", apple_object));
    }

    let (_, mut message) = parse_formatter(format, message_value, item_type, item_index)?;
    if format_type.contains("signpost") {
        let (_, signpost_message) = parse_signpost_format(format_type)?;
        message = format!("{} ({})", message, signpost_message);
    }
    Ok(("", message))
}

// Try to parse additional signpost metadata.
// Ex: %{public,signpost.description:attribute}@
//     %{public,signpost.telemetry:number1,name=SOSSignpostNameSOSCCCopyApplicantPeerInfo}d
fn parse_signpost_format(signpost_format: &str) -> nom::IResult<&str, String> {
    let mut signpost_message;
    let (signpost_value, _) = is_a("%{")(signpost_format)?;

    if signpost_format.starts_with("%{sign") {
        let signpost_vec: Vec<&str> = signpost_value.split(',').collect();
        signpost_message = signpost_vec[0].to_string();
    } else {
        let signpost_vec: Vec<&str> = signpost_value.split(',').collect();
        signpost_message = signpost_vec[1].to_string();
        signpost_message = signpost_message.trim().to_string();
    }
    Ok(("", signpost_message))
}

// Align the message to the left and pad using zeros instead of spaces
fn format_alignment_left(
    format_message: String,
    format_width: usize,
    format_precision: usize,
    type_data: &str,
    plus_minus: bool,
    hashtag: bool,
) -> String {
    let mut message = format_message;
    let mut precision_value = format_precision;
    let mut plus_option = String::new();

    let mut adjust_width = 0;
    if plus_minus {
        plus_option = String::from("+");
        adjust_width = 1;
    }

    if FLOAT_TYPES.contains(&type_data) {
        let float_message = parse_float(message);
        if precision_value == 0 {
            let message_float = float_message.to_string();
            let float_precision: Vec<&str> = message_float.split('.').collect();
            if float_precision.len() == 2 {
                precision_value = float_precision[1].len();
            }
        }
        message = format!(
            "{plus_symbol}{:0<width$.precision$}",
            float_message,
            width = format_width - adjust_width,
            precision = precision_value,
            plus_symbol = plus_option
        );
    } else if INT_TYPES.contains(&type_data) {
        let int_message = parse_int(message);
        message = format!(
            "{plus_symbol}{:0<width$.precision$}",
            int_message,
            width = format_width - adjust_width,
            precision = precision_value,
            plus_symbol = plus_option
        );
    } else if STRING_TYPES.contains(&type_data) {
        if precision_value == 0 {
            precision_value = message.len()
        }
        message = format!(
            "{plus_symbol}{:0<width$.precision$}",
            message,
            width = format_width - adjust_width,
            precision = precision_value,
            plus_symbol = plus_option
        );
    } else if HEX_TYPES.contains(&type_data) {
        let hex_message = parse_int(message);
        if hashtag {
            message = format!(
                "{plus_symbol}{:0<#width$.precision$X}",
                hex_message,
                width = format_width - adjust_width,
                precision = precision_value,
                plus_symbol = plus_option
            );
        } else {
            message = format!(
                "{plus_symbol}{:0<width$.precision$X}",
                hex_message,
                width = format_width - adjust_width,
                precision = precision_value,
                plus_symbol = plus_option
            );
        }
    } else if OCTAL_TYPES.contains(&type_data) {
        let octal_message = parse_int(message);
        if hashtag {
            message = format!(
                "{plus_symbol}{:0<#width$.precision$o}",
                octal_message,
                width = format_width - adjust_width,
                precision = precision_value,
                plus_symbol = plus_option
            );
        } else {
            message = format!(
                "{plus_symbol}{:0<width$.precision$o}",
                octal_message,
                width = format_width - adjust_width,
                precision = precision_value,
                plus_symbol = plus_option
            );
        }
    }
    message
}

// Align the message to the right and pad using zeros instead of spaces
fn format_alignment_right(
    format_message: String,
    format_width: usize,
    format_precision: usize,
    type_data: &str,
    plus_minus: bool,
    hashtag: bool,
) -> String {
    let mut message = format_message;
    let mut precision_value = format_precision;
    let mut plus_option = String::new();

    let mut adjust_width = 0;
    if plus_minus {
        plus_option = String::from("+");
        adjust_width = 1;
    }

    if FLOAT_TYPES.contains(&type_data) {
        let float_message = parse_float(message);
        if precision_value == 0 {
            let message_float = float_message.to_string();
            let float_precision: Vec<&str> = message_float.split('.').collect();
            if float_precision.len() == 2 {
                precision_value = float_precision[1].len();
            }
        }
        message = format!(
            "{plus_symbol}{:0>width$.precision$}",
            float_message,
            width = format_width - adjust_width,
            precision = precision_value,
            plus_symbol = plus_option
        );
    } else if INT_TYPES.contains(&type_data) {
        let int_message = parse_int(message);
        message = format!(
            "{plus_symbol}{:0>width$.precision$}",
            int_message,
            width = format_width - adjust_width,
            precision = precision_value,
            plus_symbol = plus_option
        );
    } else if STRING_TYPES.contains(&type_data) {
        if precision_value == 0 {
            precision_value = message.len()
        }
        message = format!(
            "{plus_symbol}{:0>width$.precision$}",
            message,
            width = format_width - adjust_width,
            precision = precision_value,
            plus_symbol = plus_option
        );
    } else if HEX_TYPES.contains(&type_data) {
        let hex_message = parse_int(message);
        if hashtag {
            message = format!(
                "{plus_symbol}{:0>#width$.precision$X}",
                hex_message,
                width = format_width - adjust_width,
                precision = precision_value,
                plus_symbol = plus_option
            );
        } else {
            message = format!(
                "{plus_symbol}{:0>width$.precision$X}",
                hex_message,
                width = format_width - adjust_width,
                precision = precision_value,
                plus_symbol = plus_option
            );
        }
    } else if OCTAL_TYPES.contains(&type_data) {
        let octal_message = parse_int(message);
        if hashtag {
            message = format!(
                "{plus_symbol}{:0>#width$.precision$o}",
                octal_message,
                width = format_width - adjust_width,
                precision = precision_value,
                plus_symbol = plus_option
            );
        } else {
            message = format!(
                "{plus_symbol}{:0>width$.precision$o}",
                octal_message,
                width = format_width - adjust_width,
                precision = precision_value,
                plus_symbol = plus_option
            );
        }
    }
    message
}

// Align the message to the left and pad using spaces
fn format_alignment_left_space(
    format_message: String,
    format_width: usize,
    format_precision: usize,
    type_data: &str,
    plus_minus: bool,
    hashtag: bool,
) -> String {
    let mut message = format_message;
    let mut precision_value = format_precision;
    let mut plus_option = String::new();

    let mut adjust_width = 0;
    if plus_minus {
        plus_option = String::from("+");
        adjust_width = 1;
    }

    if FLOAT_TYPES.contains(&type_data) {
        let float_message = parse_float(message);
        if precision_value == 0 {
            let message_float = float_message.to_string();
            let float_precision: Vec<&str> = message_float.split('.').collect();
            if float_precision.len() == 2 {
                precision_value = float_precision[1].len();
            }
        }
        message = format!(
            "{plus_symbol}{:<width$.precision$}",
            float_message,
            width = format_width - adjust_width,
            precision = precision_value,
            plus_symbol = plus_option
        );
    } else if INT_TYPES.contains(&type_data) {
        let int_message = parse_int(message);
        message = format!(
            "{plus_symbol}{:<width$.precision$}",
            int_message,
            width = format_width - adjust_width,
            precision = precision_value,
            plus_symbol = plus_option
        );
    } else if STRING_TYPES.contains(&type_data) {
        if precision_value == 0 {
            precision_value = message.len()
        }
        message = format!(
            "{plus_symbol}{:<width$.precision$}",
            message,
            width = format_width - adjust_width,
            precision = precision_value,
            plus_symbol = plus_option
        );
    } else if HEX_TYPES.contains(&type_data) {
        let hex_message = parse_int(message);
        if hashtag {
            message = format!(
                "{plus_symbol}{:<#width$.precision$X}",
                hex_message,
                width = format_width - adjust_width,
                precision = precision_value,
                plus_symbol = plus_option
            );
        } else {
            message = format!(
                "{plus_symbol}{:<width$.precision$X}",
                hex_message,
                width = format_width - adjust_width,
                precision = precision_value,
                plus_symbol = plus_option
            );
        }
    } else if OCTAL_TYPES.contains(&type_data) {
        let octal_message = parse_int(message);
        if hashtag {
            message = format!(
                "{plus_symbol}{:<#width$.precision$o}",
                octal_message,
                width = format_width - adjust_width,
                precision = precision_value,
                plus_symbol = plus_option
            );
        } else {
            message = format!(
                "{plus_symbol}{:<width$.precision$o}",
                octal_message,
                width = format_width - adjust_width,
                precision = precision_value,
                plus_symbol = plus_option
            );
        }
    }
    message
}

// Align the message to the right and pad using spaces
fn format_alignment_right_space(
    format_message: String,
    format_width: usize,
    format_precision: usize,
    type_data: &str,
    plus_minus: bool,
    hashtag: bool,
) -> String {
    let mut message = format_message;
    let mut precision_value = format_precision;
    let mut plus_option = String::new();

    let mut adjust_width = 0;
    if plus_minus {
        plus_option = String::from("+");
        adjust_width = 1;
    }

    if FLOAT_TYPES.contains(&type_data) {
        let float_message = parse_float(message);
        if precision_value == 0 {
            let message_float = float_message.to_string();
            let float_precision: Vec<&str> = message_float.split('.').collect();
            if float_precision.len() == 2 {
                precision_value = float_precision[1].len();
            }
        }
        message = format!(
            "{plus_symbol}{:>width$.precision$}",
            float_message,
            width = format_width - adjust_width,
            precision = precision_value,
            plus_symbol = plus_option
        );
    } else if INT_TYPES.contains(&type_data) {
        let int_message = parse_int(message);
        message = format!(
            "{plus_symbol}{:>width$.precision$}",
            int_message,
            width = format_width - adjust_width,
            precision = precision_value,
            plus_symbol = plus_option
        );
    } else if STRING_TYPES.contains(&type_data) {
        if precision_value == 0 {
            precision_value = message.len()
        }
        message = format!(
            "{plus_symbol}{:>width$.precision$}",
            message,
            width = format_width - adjust_width,
            precision = precision_value,
            plus_symbol = plus_option
        );
    } else if HEX_TYPES.contains(&type_data) {
        let hex_message = parse_int(message);
        if hashtag {
            message = format!(
                "{plus_symbol}{:>#width$.precision$X}",
                hex_message,
                width = format_width - adjust_width,
                precision = precision_value,
                plus_symbol = plus_option
            );
        } else {
            message = format!(
                "{plus_symbol}{:>width$.precision$X}",
                hex_message,
                width = format_width - adjust_width,
                precision = precision_value,
                plus_symbol = plus_option
            );
        }
    } else if OCTAL_TYPES.contains(&type_data) {
        let octal_message = parse_int(message);
        if hashtag {
            message = format!(
                "{plus_symbol}{:>#width$.precision$o}",
                octal_message,
                width = format_width - adjust_width,
                precision = precision_value,
                plus_symbol = plus_option
            );
        } else {
            message = format!(
                "{plus_symbol}{:>width$.precision$o}",
                octal_message,
                width = format_width - adjust_width,
                precision = precision_value,
                plus_symbol = plus_option
            );
        }
    }
    message
}

// Align the message to the left
fn format_left(
    format_message: String,
    format_precision: usize,
    type_data: &str,
    plus_minus: bool,
    hashtag: bool,
) -> String {
    let mut message = format_message;
    let mut precision_value = format_precision;
    let mut plus_option = String::new();

    if plus_minus {
        plus_option = String::from("+");
    }

    if FLOAT_TYPES.contains(&type_data) {
        let float_message = parse_float(message);
        if precision_value == 0 {
            let message_float = float_message.to_string();
            let float_precision: Vec<&str> = message_float.split('.').collect();
            if float_precision.len() == 2 {
                precision_value = float_precision[1].len();
            }
        }

        message = format!(
            "{plus_symbol}{:<.precision$}",
            float_message,
            precision = precision_value,
            plus_symbol = plus_option
        );
    } else if INT_TYPES.contains(&type_data) {
        let int_message = parse_int(message);
        message = format!(
            "{plus_symbol}{:<.precision$}",
            int_message,
            precision = precision_value,
            plus_symbol = plus_option
        );
    } else if STRING_TYPES.contains(&type_data) {
        if precision_value == 0 {
            precision_value = message.len()
        }
        message = format!(
            "{plus_symbol}{:<.precision$}",
            message,
            precision = precision_value,
            plus_symbol = plus_option
        );
    } else if HEX_TYPES.contains(&type_data) {
        let hex_message = parse_int(message);
        if hashtag {
            message = format!(
                "{plus_symbol}{:<#.precision$X}",
                hex_message,
                precision = precision_value,
                plus_symbol = plus_option
            );
        } else {
            message = format!(
                "{plus_symbol}{:<.precision$X}",
                hex_message,
                precision = precision_value,
                plus_symbol = plus_option
            );
        }
    } else if OCTAL_TYPES.contains(&type_data) {
        let octal_message = parse_int(message);
        if hashtag {
            message = format!(
                "{plus_symbol}{:<#.precision$o}",
                octal_message,
                precision = precision_value,
                plus_symbol = plus_option
            );
        } else {
            message = format!(
                "{plus_symbol}{:<.precision$o}",
                octal_message,
                precision = precision_value,
                plus_symbol = plus_option
            );
        }
    }
    message
}

// Align the message to the right (default)
fn format_right(
    format_message: String,
    format_precision: usize,
    type_data: &str,
    plus_minus: bool,
    hashtag: bool,
) -> String {
    let mut message = format_message;
    let mut precision_value = format_precision;
    let mut plus_option = String::new();

    if plus_minus {
        plus_option = String::from("+");
    }

    if FLOAT_TYPES.contains(&type_data) {
        let float_message = parse_float(message);
        if precision_value == 0 {
            let message_float = float_message.to_string();
            let float_precision: Vec<&str> = message_float.split('.').collect();
            if float_precision.len() == 2 {
                precision_value = float_precision[1].len();
            }
        }

        message = format!(
            "{plus_symbol}{:>.precision$}",
            float_message,
            precision = precision_value,
            plus_symbol = plus_option
        );
    } else if INT_TYPES.contains(&type_data) {
        let int_message = parse_int(message);
        message = format!(
            "{plus_symbol}{:>.precision$}",
            int_message,
            precision = precision_value,
            plus_symbol = plus_option
        );
    } else if STRING_TYPES.contains(&type_data) {
        if precision_value == 0 {
            precision_value = message.len()
        }
        message = format!(
            "{plus_symbol}{:>.precision$}",
            message,
            precision = precision_value,
            plus_symbol = plus_option
        );
    } else if HEX_TYPES.contains(&type_data) {
        let hex_message = parse_int(message);
        if hashtag {
            message = format!(
                "{plus_symbol}{:>#.precision$X}",
                hex_message,
                precision = precision_value,
                plus_symbol = plus_option
            );
        } else {
            message = format!(
                "{plus_symbol}{:>.precision$X}",
                hex_message,
                precision = precision_value,
                plus_symbol = plus_option
            );
        }
    } else if OCTAL_TYPES.contains(&type_data) {
        let octal_message = parse_int(message);
        message = format!(
            "{plus_symbol}{:>#.precision$o}",
            octal_message,
            precision = precision_value,
            plus_symbol = plus_option
        );
    }
    message
}

// Parse the float string log message to float value
fn parse_float(message: String) -> f64 {
    let byte_results = message.parse::<i64>();
    match byte_results {
        Ok(bytes) => return f64::from_bits(bytes as u64),
        Err(err) => error!(
            "[macos-unifiedlogs] Failed to parse float log message value: {}, err: {:?}",
            message, err
        ),
    }
    f64::from_bits(0)
}

// Parse the int string log message to int value
fn parse_int(message: String) -> i64 {
    let int_results = message.parse::<i64>();
    match int_results {
        Ok(message) => return message,
        Err(err) => error!(
            "[macos-unifiedlogs] Failed to parse int log message value: {}, err: {:?}",
            message, err
        ),
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::chunks::firehose::firehose_log::FirehoseItemInfo;
    use crate::message::{
        format_alignment_left, format_alignment_left_space, format_alignment_right,
        format_alignment_right_space, format_firehose_log_message, format_left, format_right,
        parse_float, parse_formatter, parse_int, parse_signpost_format, parse_type_formatter,
    };

    #[test]
    fn test_format_firehose_log_message() {
        let test_data = String::from("opendirectoryd (build %{public}s) launched...");
        let mut item_message: Vec<FirehoseItemInfo> = Vec::new();
        item_message.push(FirehoseItemInfo {
            message_strings: String::from("796.100"),
            item_type: 34,
            item_size: 0,
        });

        let log_string = format_firehose_log_message(test_data, &item_message);
        assert_eq!(log_string, "opendirectoryd (build 796.100) launched...")
    }

    #[test]
    fn test_parse_formatter() {
        let test_format = "%+04d";
        let mut test_message = Vec::new();

        let test_data = FirehoseItemInfo {
            message_strings: String::from("2"),
            item_type: 2,
            item_size: 2,
        };
        test_message.push(test_data);

        let item_index = 0;
        let (_, formatted_results) = parse_formatter(
            test_format,
            &test_message,
            &test_message[0].item_type,
            item_index,
        )
        .unwrap();
        assert_eq!(formatted_results, "+002");

        let test_format = "%04d";
        let (_, formatted_results) = parse_formatter(
            test_format,
            &test_message,
            &test_message[0].item_type,
            item_index,
        )
        .unwrap();
        assert_eq!(formatted_results, "0002");

        let test_format = "%#4x";
        let (_, formatted_results) = parse_formatter(
            test_format,
            &test_message,
            &test_message[0].item_type,
            item_index,
        )
        .unwrap();
        assert_eq!(formatted_results, " 0x2");

        let test_format = "%#04o";
        test_message[0].message_strings = String::from("100");
        let (_, formatted_results) = parse_formatter(
            test_format,
            &test_message,
            &test_message[0].item_type,
            item_index,
        )
        .unwrap();
        assert_eq!(formatted_results, "0o144");

        let test_format = "%07o";
        let (_, formatted_results) = parse_formatter(
            test_format,
            &test_message,
            &test_message[0].item_type,
            item_index,
        )
        .unwrap();
        assert_eq!(formatted_results, "0000144");

        let test_format = "%x";
        test_message[0].message_strings = String::from("10");
        let (_, formatted_results) = parse_formatter(
            test_format,
            &test_message,
            &test_message[0].item_type,
            item_index,
        )
        .unwrap();
        assert_eq!(formatted_results, "A");

        let test_float = "%+09.4f";
        test_message[0].message_strings = String::from("4570111009880014848");
        let (_, formatted_results) = parse_formatter(
            test_float,
            &test_message,
            &test_message[0].item_type,
            item_index,
        )
        .unwrap();
        assert_eq!(formatted_results, "+000.0035");

        let test_float = "%9.4f";
        let (_, formatted_results) = parse_formatter(
            test_float,
            &test_message,
            &test_message[0].item_type,
            item_index,
        )
        .unwrap();
        assert_eq!(formatted_results, "   0.0035");

        let test_float = "%-8.4f";
        let (_, formatted_results) = parse_formatter(
            test_float,
            &test_message,
            &test_message[0].item_type,
            item_index,
        )
        .unwrap();
        assert_eq!(formatted_results, "0.0035  ");

        let test_float = "%f";
        test_message[0].message_strings = String::from("4614286721111404799");
        let (_, formatted_results) = parse_formatter(
            test_float,
            &test_message,
            &test_message[0].item_type,
            item_index,
        )
        .unwrap();
        assert_eq!(formatted_results, "3.154944");

        let test_int = "%d";
        test_message[0].message_strings = String::from("-248");
        let (_, formatted_results) = parse_formatter(
            test_int,
            &test_message,
            &test_message[0].item_type,
            item_index,
        )
        .unwrap();
        assert_eq!(formatted_results, "-248");

        let test_float = "%f";
        test_message[0].message_strings = String::from("-4611686018427387904");
        let (_, formatted_results) = parse_formatter(
            test_float,
            &test_message,
            &test_message[0].item_type,
            item_index,
        )
        .unwrap();
        assert_eq!(formatted_results, "-2");

        let test_float = "%f";
        test_message[0].message_strings = String::from("-4484628366119329180");
        let (_, formatted_results) = parse_formatter(
            test_float,
            &test_message,
            &test_message[0].item_type,
            item_index,
        )
        .unwrap();
        assert_eq!(formatted_results, "-650937839.633862");

        let test_string = "%s";
        test_message[0].message_strings = String::from("The big red dog jumped over the crab");
        let (_, formatted_results) = parse_formatter(
            test_string,
            &test_message,
            &test_message[0].item_type,
            item_index,
        )
        .unwrap();
        assert_eq!(formatted_results, "The big red dog jumped over the crab");

        let test_string = "%.2@";
        test_message[0].message_strings = String::from("aaabbbb");
        let (_, formatted_results) = parse_formatter(
            test_string,
            &test_message,
            &test_message[0].item_type,
            item_index,
        )
        .unwrap();
        assert_eq!(formatted_results, "aa");

        let test_string = "%*s";
        test_message[0].item_size = 10;
        test_message[0].item_type = 0x12;
        let test_data2 = FirehoseItemInfo {
            message_strings: String::from("hi"),
            item_type: 2,
            item_size: 2,
        };
        test_message.push(test_data2);
        let (_, formatted_results) = parse_formatter(
            test_string,
            &test_message,
            &test_message[0].item_type,
            item_index,
        )
        .unwrap();
        assert_eq!(formatted_results, "        hi");
    }

    #[test]
    fn test_parse_type_formatter() {
        let mut test_format = "%{public}s";
        let mut test_message = Vec::new();

        let mut test_data = FirehoseItemInfo {
            message_strings: String::from("test"),
            item_type: 2,
            item_size: 4,
        };
        test_message.push(test_data);

        let item_index = 0;
        let (_, formatted_results) = parse_type_formatter(
            test_format,
            &test_message,
            &test_message[0].item_type,
            item_index,
        )
        .unwrap();
        assert_eq!(formatted_results, "test");

        test_format = "%{public, signpost.description:begin_time}llu";
        let mut test_message = Vec::new();

        test_data = FirehoseItemInfo {
            message_strings: String::from("1"),
            item_type: 2,
            item_size: 4,
        };
        test_message.push(test_data);

        let item_index = 0;
        let (_, formatted_results) = parse_type_formatter(
            test_format,
            &test_message,
            &test_message[0].item_type,
            item_index,
        )
        .unwrap();
        assert_eq!(formatted_results, "1 (signpost.description:begin_time)");
    }

    #[test]
    fn test_parse_signpost_format() {
        let test_format = "%{public, signpost.description:begin_time";
        let (_, results) = parse_signpost_format(test_format).unwrap();
        assert_eq!(results, "signpost.description:begin_time");
    }

    #[test]
    fn test_format_alignment_left() {
        let test_type = "d";
        let test_width = 4;
        let test_precision = 0;
        let test_format = String::from("2");
        let plus_minus = false;
        let hashtag = false;
        let formatted_results = format_alignment_left(
            test_format,
            test_width,
            test_precision,
            &test_type,
            plus_minus,
            hashtag,
        );
        assert_eq!(formatted_results, "2000");
    }

    #[test]
    fn test_format_alignment_right() {
        let test_type = "d";
        let test_width = 4;
        let test_precision = 0;
        let test_format = String::from("2");
        let plus_minus = false;
        let hashtag = false;
        let formatted_results = format_alignment_right(
            test_format,
            test_width,
            test_precision,
            &test_type,
            plus_minus,
            hashtag,
        );
        assert_eq!(formatted_results, "0002");
    }

    #[test]
    fn test_format_alignment_left_space() {
        let test_type = "d";
        let test_width = 4;
        let test_precision = 0;
        let test_format = String::from("2");
        let plus_minus = false;
        let hashtag = false;
        let formatted_results = format_alignment_left_space(
            test_format,
            test_width,
            test_precision,
            &test_type,
            plus_minus,
            hashtag,
        );
        assert_eq!(formatted_results, "2   ");
    }

    #[test]
    fn test_format_alignment_right_space() {
        let test_type = "d";
        let test_width = 4;
        let test_precision = 0;
        let test_format = String::from("2");
        let plus_minus = false;
        let hashtag = false;
        let formatted_results = format_alignment_right_space(
            test_format,
            test_width,
            test_precision,
            &test_type,
            plus_minus,
            hashtag,
        );
        assert_eq!(formatted_results, "   2");
    }

    #[test]
    fn test_format_left() {
        let test_type = "d";
        let test_precision = 0;
        let test_format = String::from("2");
        let plus_minus = false;
        let hashtag = false;
        let formatted_results =
            format_left(test_format, test_precision, &test_type, plus_minus, hashtag);
        assert_eq!(formatted_results, "2");
    }

    #[test]
    fn test_format_right() {
        let test_type = "d";
        let test_precision = 0;
        let test_format = String::from("2");
        let plus_minus = false;
        let hashtag = false;
        let formatted_results =
            format_right(test_format, test_precision, &test_type, plus_minus, hashtag);
        assert_eq!(formatted_results, "2");
    }

    #[test]
    fn test_parse_float() {
        let value = String::from("4611911198408756429");
        let results = parse_float(value);
        assert_eq!(results, 2.1);
    }

    #[test]
    fn test_parse_int() {
        let value = String::from("2");
        let results = parse_int(value);
        assert_eq!(results, 2);
    }
}
