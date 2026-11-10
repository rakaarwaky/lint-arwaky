// PURPOSE: NamingAuditStats — how many files each naming rule read.
//
// A clean run and a run that checked nothing used to look the same: both
// produced an empty list of findings, so a reader could not tell "your names
// are clean" from "the rule never ran". The counts answer that question —
// they say how many files each rule read, so an empty result is never a
// silent pass.
//
// The counts are the *inspected* files, not the files that produced a
// finding: a rule that read 300 files and found nothing has done its job, and
// the report has to say so.

/// How many files each naming rule read during one audit.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NamingAuditStats {
    /// The files AES101 read.
    pub aes101_inspected: usize,
    /// The files AES102 read.
    pub aes102_inspected: usize,
    /// The files AES103 read.
    pub aes103_inspected: usize,
}

impl NamingAuditStats {
    /// The files every rule read together.
    pub fn total_inspected(&self) -> usize {
        self.aes101_inspected + self.aes102_inspected + self.aes103_inspected
    }
}
