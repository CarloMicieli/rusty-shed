//! NMRA / NEM DCC connector types for digital decoder interfaces.
//!
//! Overview
//! --------
//! These enums represent the common mechanical/electrical decoder connector
//! standards (NMRA / NEM) used to attach multifunction DCC decoders to a
//! locomotive's wiring. Each variant maps to a canonical external string
//! used for serialization (see `serde(rename = "...")`) and for textual
//! parsing via `strum`'s `EnumString` derive.
//!
//! Practical notes
//! - Some locomotives include a blanking plug that must be removed before a
//!   decoder can be installed. Others are not DCC-ready and require a
//!   hardwired decoder or a model-specific control board replacement.
//! - The enum values serialize to strings such as `"NEM_651"` or `"PLUX_8"`
//!   (these values are defined by the `serde(rename = "...")` attributes).
//! - Parsing from strings uses `str::FromStr` (provided by `strum_macros::EnumString`).
//!   In the current setup parsing is case-sensitive; to support case-insensitive
//!   parsing you can add the appropriate `strum` attributes.

use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

/// The NMRA and NEM Connectors for digital control (DCC)
///
/// # Description
/// The NMRA and NEM adopted standard mechanical and electrical interfaces to connect Multifunction
/// Decoders to a locomotive's electrical system. These plugs and sockets make it simpler to install
/// a decoder into a suitably equipped locomotive.
///
/// In many cases a blanking plug must be removed before installing the decoder. If a locomotive
/// is not DCC-Ready it will lack an interface and must use a Hardwired Decoder or a drop-in
/// replacement DCC control board (if available) for that specific model.
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, Serialize, Deserialize)]
pub enum DccInterface {
    /// 6 Pin standard mechanical and electrical interfaces (NMRA Small)
    #[serde(rename = "NEM_651")]
    #[strum(serialize = "NEM_651")]
    Nem651,

    /// 8 Pin standard mechanical and electrical interfaces (NMRA Medium)
    #[serde(rename = "NEM_652")]
    #[strum(serialize = "NEM_652")]
    Nem652,

    /// 4 Pin standard mechanical and electrical interfaces (NMRA Large)
    #[serde(rename = "NEM_654")]
    #[strum(serialize = "NEM_654")]
    Nem654,

    /// The PluX8 connector consists of two rows of 4 pins.
    #[serde(rename = "PLUX_8")]
    #[strum(serialize = "PLUX_8")]
    Plux8,

    #[serde(rename = "PLUX_12")]
    #[strum(serialize = "PLUX_12")]
    Plux12,

    /// The PluX16 connector consists of two rows of 8 pins.
    #[serde(rename = "PLUX_16")]
    #[strum(serialize = "PLUX_16")]
    Plux16,

    /// The PluX22 connector consists of two rows of 11 pins.
    #[serde(rename = "PLUX_22")]
    #[strum(serialize = "PLUX_22")]
    Plux22,

    /// standard connector for extremely tight applications, such as TT and N scale locomotives (NEM 662)
    #[serde(rename = "NEXT_18")]
    #[strum(serialize = "NEXT_18")]
    Next18,

    #[serde(rename = "NEXT_18_S")]
    #[strum(serialize = "NEXT_18_S")]
    Next18S,

    /// 21MTC Connector interface is a standard adopted by both the NMRA and NEM (NEM 660).
    /// Its name comes from 21 pin Marklin/Trix Connector, developed by Marklin and ESU.
    #[serde(rename = "MTC_21")]
    #[strum(serialize = "MTC_21")]
    Mtc21,
}
