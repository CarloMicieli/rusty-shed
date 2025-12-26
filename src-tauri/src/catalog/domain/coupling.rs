use crate::catalog::domain::{CouplingSocket, FeatureFlag};
use serde::{Deserialize, Serialize};

/// It represents the coupling configuration for a rolling stock.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Coupling {
    /// the rolling stock coupling socket
    pub socket: Option<CouplingSocket>,
    /// the rolling stock has a close coupling mechanism
    pub close_couplers: Option<FeatureFlag>,
    /// the rolling stock has a digital shunting couplers mechanism
    pub digital_shunting: Option<FeatureFlag>,
}

impl Coupling {
    /// Creates a new rolling stock coupling configuration
    pub fn new(
        socket: CouplingSocket,
        close_couplers: FeatureFlag,
        digital_shunting: FeatureFlag,
    ) -> Self {
        Coupling {
            socket: Some(socket),
            close_couplers: Some(close_couplers),
            digital_shunting: Some(digital_shunting),
        }
    }

    /// Creates a new close coupling configuration with the `socket` socket
    pub fn with_close_couplers(socket: CouplingSocket) -> Self {
        Coupling {
            socket: Some(socket),
            close_couplers: Some(FeatureFlag::Yes),
            digital_shunting: Some(FeatureFlag::No),
        }
    }

    /// Creates a new digital shunting coupling configuration
    pub fn with_digital_shunting_couplers() -> Self {
        Coupling {
            socket: Some(CouplingSocket::None),
            close_couplers: Some(FeatureFlag::No),
            digital_shunting: Some(FeatureFlag::Yes),
        }
    }

    /// the coupling socket if present
    pub fn socket(&self) -> Option<CouplingSocket> {
        self.socket
    }

    /// true if the coupling configuration include a mechanism to reduce the gaps between two
    /// rolling stocks; false otherwise
    pub fn close_couplers(&self) -> Option<FeatureFlag> {
        self.close_couplers
    }

    /// true if the coupling configuration implements digital control functionalities,
    /// false otherwise  
    pub fn digital_shunting(&self) -> Option<FeatureFlag> {
        self.digital_shunting
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_create_new_couplings() {
        let coupling = Coupling::new(
            CouplingSocket::Nem362,
            FeatureFlag::Yes,
            FeatureFlag::NotApplicable,
        );

        assert_eq!(coupling.socket(), Some(CouplingSocket::Nem362));
        assert_eq!(
            coupling.digital_shunting(),
            Some(FeatureFlag::NotApplicable)
        );
        assert_eq!(coupling.close_couplers(), Some(FeatureFlag::Yes));
    }

    #[test]
    fn it_should_create_close_couplers() {
        let coupling = Coupling::with_close_couplers(CouplingSocket::Nem362);

        assert_eq!(coupling.socket, Some(CouplingSocket::Nem362));
        assert_eq!(coupling.digital_shunting, Some(FeatureFlag::No));
        assert_eq!(coupling.close_couplers, Some(FeatureFlag::Yes));
    }

    #[test]
    fn it_should_create_digital_shunting_couplers() {
        let coupling = Coupling::with_digital_shunting_couplers();

        assert_eq!(coupling.socket, Some(CouplingSocket::None));
        assert_eq!(coupling.digital_shunting, Some(FeatureFlag::Yes));
        assert_eq!(coupling.close_couplers, Some(FeatureFlag::No));
    }
}
