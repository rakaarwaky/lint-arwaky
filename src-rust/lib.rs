#[cfg(any(feature = "check_taxonomy", feature = "check_surfaces"))]
pub mod taxonomy;

#[cfg(any(feature = "check_contract", feature = "check_surfaces"))]
pub mod contract;

#[cfg(any(feature = "check_infrastructure", feature = "check_surfaces"))]
pub mod infrastructure;

#[cfg(any(feature = "check_capabilities", feature = "check_surfaces"))]
pub mod capabilities;

#[cfg(any(feature = "check_agent", feature = "check_surfaces"))]
pub mod agent;

#[cfg(feature = "check_surfaces")]
pub mod surfaces;
