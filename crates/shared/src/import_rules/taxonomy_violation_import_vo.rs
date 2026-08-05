// PURPOSE: AesImportViolation — data container for import rule violations (AES201-205)
// Messages are written in capabilities layer, not here.
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_name_vo::SymbolName;

#[derive(Debug, Clone)]
pub enum AesImportViolation {
    ForbiddenImport {
        source_layer: LayerNameVO,
        forbidden_layer: LayerNameVO,
        allowed: Vec<LayerNameVO>,
        reason: Option<LintMessage>,
        fix: LintMessage,
    },
    MissingImport {
        source_layer: LayerNameVO,
        required: SymbolName,
        reason: Option<LintMessage>,
    },
    FixUnusedImport {
        reason: Option<LintMessage>,
    },
    ImportIntentViolation {
        source_layer: LayerNameVO,
        import_type: SymbolName,
        intent: SymbolName,
        reason: Option<LintMessage>,
    },
    CircularImport {
        reason: Option<LintMessage>,
    },
}
