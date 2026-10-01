// PURPOSE: MaintenanceResponse — response payload for the maintenance aggregate

use crate::taxonomy_maintenance_vo::MaintenanceStatsVO;
use crate::taxonomy_maintenance_vo::{
    DependencyReport, DoctorResultVO, HealthCheckResult, SecurityScanReport, SelfUpdateResultVO,
    ToolchainDiagnostics,
};

pub enum MaintenanceResponse {
    Stats {
        stats: MaintenanceStatsVO,
    },
    Clean,
    Update,
    SelfUpdate {
        result: SelfUpdateResultVO,
    },
    Doctor {
        result: DoctorResultVO,
    },
    Cancelled,
    Toolchain {
        diagnostics: ToolchainDiagnostics,
    },
    Health {
        health: HealthCheckResult,
    },
    Security {
        report: SecurityScanReport,
    },
    Dependencies {
        report: Result<DependencyReport, String>,
    },
}

impl MaintenanceResponse {
    pub fn into_stats(self) -> MaintenanceStatsVO {
        match self {
            Self::Stats { stats } => stats,
            _ => MaintenanceStatsVO::default(),
        }
    }

    /// Report whether a cleanup or update ran. Returns false for other verbs.
    pub fn into_ran(self) -> bool {
        matches!(self, Self::Clean | Self::Update)
    }

    /// Extract the self-update result. Returns an error result for other verbs.
    pub fn into_self_update(self) -> SelfUpdateResultVO {
        match self {
            Self::SelfUpdate { result } => result,
            _ => SelfUpdateResultVO::error("", "no self-update result available"),
        }
    }

    pub fn into_doctor(self) -> DoctorResultVO {
        match self {
            Self::Doctor { result } => result,
            _ => DoctorResultVO::default(),
        }
    }

    /// Report whether a job was cancelled. Returns false for other verbs.
    pub fn into_cancelled(self) -> bool {
        matches!(self, Self::Cancelled)
    }

    pub fn into_toolchain(self) -> ToolchainDiagnostics {
        match self {
            Self::Toolchain { diagnostics } => diagnostics,
            _ => ToolchainDiagnostics::default(),
        }
    }

    pub fn into_health(self) -> HealthCheckResult {
        match self {
            Self::Health { health } => health,
            _ => HealthCheckResult::default(),
        }
    }

    pub fn into_security_report(self) -> SecurityScanReport {
        match self {
            Self::Security { report } => report,
            _ => SecurityScanReport::default(),
        }
    }

    pub fn into_dependency_report(self) -> Result<DependencyReport, String> {
        match self {
            Self::Dependencies { report } => report,
            _ => Err(String::from("no dependency report available")),
        }
    }
}
