// PURPOSE: response VO for the single-subsystem feature
//
// The single checker returns a verdict the agent passes straight through, so
// the response pairs a boolean outcome with the detail that produced it.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SingleResponse {
    pub passed: bool,
    pub detail: String,
}
