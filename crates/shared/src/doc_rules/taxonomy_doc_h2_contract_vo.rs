// PURPOSE: DocH2Contract — H2 heading contract tuple VO for AES605 document structure

/// An (doc-name, required-H2s, allowed-H2s) triple governing AES605 heading checks.
pub type DocH2Contract = (
    &'static str,
    &'static [&'static str],
    &'static [&'static str],
);
