// PURPOSE: FileContentVO, Identity, LayerNameVO, LineContentVO — VOs for layer identity and file content
//
// These value objects are used throughout the AES layer-identity system:
// - FileContentVO wraps the raw text of a source file.
// - Identity identifies a single AES architectural layer.
// - LayerNameVO is a human-readable label for a layer.
// - LineContentVO wraps a single line of source text.
use crate::string_value_object;

string_value_object!(FileContentVO);
string_value_object!(Identity);
string_value_object!(LayerNameVO);
string_value_object!(LineContentVO);
