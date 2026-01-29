use serde::{Deserialize, Serialize};

/// Represents a DCC (Digital Command Control) address for model trains.
/// Valid DCC addresses range from 1 to 10239.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type, specta::Type,
)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct DccAddress(u16);

impl DccAddress {
    pub fn new(address: u16) -> anyhow::Result<Self> {
        if address == 0 || address > 10239 {
            return Err(anyhow::anyhow!("DCC address must be between 1 and 10239"));
        }
        Ok(DccAddress(address))
    }

    pub fn value(&self) -> u16 {
        self.0
    }
}

impl std::fmt::Display for DccAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Deref for DccAddress {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_create_valid_dcc_address() {
        let addr = DccAddress::new(5000).expect("expected valid DCC address");
        assert_eq!(addr.value(), 5000);
    }

    #[test]
    fn it_should_create_invalid_dcc_address_zero() {
        let err = DccAddress::new(0).expect_err("DCC address of 0 should fail");
        let msg = format!("{}", err);
        assert!(msg.contains("must be between 1 and 10239"));
    }

    #[test]
    fn it_should_create_invalid_dcc_address_too_high() {
        let err = DccAddress::new(20000).expect_err("DCC address above 10239 should fail");
        let msg = format!("{}", err);
        assert!(msg.contains("must be between 1 and 10239"));
    }
}
