//! Part 10 (optional): a small state machine guarding transaction lifecycle
//! transitions.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxState {
    Created,
    Validated,
    Signed,
    Broadcast,
    Confirmed,
    Rejected,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidTransition {
    pub from: TxState,
    pub to: TxState,
}

impl fmt::Display for TxState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            TxState::Created => "created",
            TxState::Validated => "validated",
            TxState::Signed => "signed",
            TxState::Broadcast => "broadcast",
            TxState::Confirmed => "confirmed",
            TxState::Rejected => "rejected",
        };

        formatter.write_str(label)
    }
}

impl fmt::Display for InvalidTransition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "cannot move a transaction from {} to {}",
            self.from, self.to
        )
    }
}

impl std::error::Error for InvalidTransition {}

impl TxState {
    /// Returns the next state, or an error when the move is not part of the
    /// lifecycle. Confirmed and Rejected are terminal.
    pub fn transition(self, next: TxState) -> Result<TxState, InvalidTransition> {
        let allowed = matches!(
            (self, next),
            (TxState::Created, TxState::Validated)
                | (TxState::Created, TxState::Rejected)
                | (TxState::Validated, TxState::Signed)
                | (TxState::Validated, TxState::Rejected)
                | (TxState::Signed, TxState::Broadcast)
                | (TxState::Signed, TxState::Rejected)
                | (TxState::Broadcast, TxState::Confirmed)
                | (TxState::Broadcast, TxState::Rejected)
        );

        if allowed {
            Ok(next)
        } else {
            Err(InvalidTransition {
                from: self,
                to: next,
            })
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(self, TxState::Confirmed | TxState::Rejected)
    }
}
