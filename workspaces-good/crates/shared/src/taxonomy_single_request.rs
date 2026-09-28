// PURPOSE: request VO for the single-subsystem feature
//
// The feature keeps one request type, so it carries the value the agent
// forwards to its checker rather than a verb that selects between subsystems.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SingleRequest {
    pub target: String,
}
