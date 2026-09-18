/// I5: pending destructive action awaiting user confirmation.
///
/// `pending` holds the TuiEvent to execute once the user confirms (Enter/y).
/// Stored on `AppState.pending_confirm`; `None` means no confirmation is pending.
#[derive(Debug, Clone)]
pub struct ConfirmState {
    /// TuiEvent to execute on confirmation.
    pub pending: super::TuiEvent,
    /// Human-readable action label shown in the confirm prompt.
    pub label: String,
}
