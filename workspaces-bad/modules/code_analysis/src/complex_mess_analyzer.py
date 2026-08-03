"""
complex_mess_analyzer.py — DELIBERATELY COMPLEX TEST FILE

This file is INTENTIONALLY designed to violate Radon rules:
  - Cyclomatic complexity threshold: 10 (each function targets 25+)
  - Maintainability index threshold: Very Low

DO NOT REFACTOR. This is a test fixture for the auto_linter project.
"""

import sys
import json
import math
from typing import Any, Dict, List, Optional, Tuple, Union


def analyze_data_complexity(
    data_source: str,
    config: Dict[str, Any],
    options: Optional[Dict[str, Any]] = None,
    flags: Optional[List[str]] = None,
) -> Dict[str, Any]:
    """
    Super complex data analysis function with massive cyclomatic complexity (30+).
    Every branch is a deliberate mess of nested conditions.
    """
    result = {"status": "pending", "data": {}, "errors": [], "warnings": []}

    if options is None:
        options = {}
    if flags is None:
        flags = []

    raw_data = None
    if data_source == "file":
        try:
            with open(config.get("path", "/dev/null"), "r") as f:
                raw_data = f.read()
        except FileNotFoundError:
            result["errors"].append("File not found")
        except PermissionError:
            result["errors"].append("Permission denied")
        except IsADirectoryError:
            result["errors"].append("Is a directory")
        except OSError:
            result["errors"].append("OS error occurred")
        else:
            if raw_data is not None:
                if len(raw_data) > 0:
                    if config.get("parse_json", False):
                        try:
                            raw_data = json.loads(raw_data)
                        except json.JSONDecodeError:
                            result["errors"].append("Invalid JSON")
                        except ValueError:
                            result["errors"].append("Value error in JSON")
                        except TypeError:
                            result["errors"].append("Type error in JSON")
                        else:
                            if isinstance(raw_data, list):
                                result["data"]["type"] = "array"
                            elif isinstance(raw_data, dict):
                                result["data"]["type"] = "object"
                            else:
                                result["data"]["type"] = "scalar"
                    else:
                        result["data"]["type"] = "text"
                        result["data"]["size"] = len(raw_data)
                else:
                    result["warnings"].append("Empty file")
                    if config.get("fail_on_empty", False):
                        result["errors"].append("Empty file not allowed")
    elif data_source == "api":
        endpoint = config.get("endpoint", "")
        if endpoint.startswith("http"):
            if "secure" in endpoint or "https" in endpoint:
                result["data"]["protocol"] = "https"
            else:
                result["data"]["protocol"] = "http"
            if "timeout" in config:
                timeout = config["timeout"]
                if timeout > 60:
                    result["warnings"].append("Timeout too long")
                elif timeout < 1:
                    result["warnings"].append("Timeout too short")
                elif timeout == 30:
                    result["data"]["timeout"] = "default"
                else:
                    result["data"]["timeout"] = str(timeout)
            else:
                result["warnings"].append("No timeout configured")
        else:
            result["errors"].append("Invalid endpoint")
    elif data_source == "database":
        db_type = config.get("type", "sqlite")
        if db_type == "postgres":
            if "host" in config:
                if "db_port" in config:
                    if config["db_port"] == 5432:
                        result["data"]["db"] = "postgres_default"
                    else:
                        result["data"]["db"] = "postgres_custom"
                else:
                    result["warnings"].append("No db_port for postgres")
            else:
                result["errors"].append("No host for postgres")
        elif db_type == "mysql":
            if config.get("host") and config.get("user"):
                result["data"]["db"] = "mysql_configured"
            else:
                result["errors"].append("MySQL incomplete config")
        elif db_type == "sqlite":
            if config.get("path"):
                result["data"]["db"] = "sqlite_file"
            else:
                result["errors"].append("SQLite needs path")
        else:
            result["errors"].append(f"Unknown db type: {db_type}")
    elif data_source == "stdin":
        raw_data = sys.stdin.read() if not sys.stdin.isatty() else None
        if raw_data:
            result["data"]["source"] = "stdin"
            result["data"]["size"] = len(raw_data)
        else:
            result["warnings"].append("No stdin data")
            if config.get("strict_stdin", False):
                result["errors"].append("Stdin required but empty")
    elif data_source == "memory":
        if "buffer" in config:
            buf = config["buffer"]
            if isinstance(buf, bytes):
                result["data"]["encoding"] = "bytes"
            elif isinstance(buf, str):
                result["data"]["encoding"] = "string"
            elif isinstance(buf, (list, dict)):
                result["data"]["encoding"] = "structured"
            else:
                result["data"]["encoding"] = "unknown"
        else:
            result["errors"].append("No buffer in memory source")
    else:
        result["errors"].append(f"Unknown data source: {data_source}")
        if config.get("fallback_default", False):
            result["data"]["source"] = "fallback"
            result["data"]["type"] = "unknown"

    # Process flags with nested complexity
    if flags:
        for flag in flags:
            if flag == "verbose":
                result["verbose"] = True
            elif flag == "debug":
                result["debug"] = True
                if "errors" in result and result["errors"]:
                    result["debug_info"] = result["errors"]
            elif flag == "strict":
                result["strict"] = True
                if "warnings" in result and result["warnings"]:
                    result["errors"].extend(result["warnings"])
                    result["warnings"] = []
            elif flag == "quiet":
                result["quiet"] = True
            elif flag == "no_cache":
                result["cache"] = False
            elif flag == "force":
                result["forced"] = True
                if "errors" in result:
                    result["errors"] = []
            elif flag == "dry_run":
                result["dry_run"] = True
            else:
                result["warnings"].append(f"Unknown flag: {flag}")

    # Determine final status
    if result["errors"]:
        if all("warning" in e.lower() or "minor" in e.lower() for e in result["errors"]):
            result["status"] = "warning"
        else:
            result["status"] = "error"
    elif result["warnings"]:
        result["status"] = "warning"
    else:
        if result["data"]:
            result["status"] = "success"
        else:
            result["status"] = "empty"

    return result


def validate_and_transform_mess(
    input_data: Any,
    rules: List[Dict[str, Any]],
    transformers: Optional[List[str]] = None,
    mode: str = "strict",
) -> Tuple[bool, Any, List[str]]:
    """
    Extremely complex validation and transformation function.
    Cyclomatic complexity: 30+ with deeply nested conditions.
    """
    errors = []
    warnings = []
    transformed = None
    valid = False

    if transformers is None:
        transformers = ["default"]

    if input_data is None:
        errors.append("Input is None")
        return False, None, errors

    if isinstance(input_data, str):
        if mode == "strict":
            if len(input_data) == 0:
                errors.append("Empty string")
            elif len(input_data) > 10000:
                errors.append("String too long")
            elif not input_data.strip():
                errors.append("Whitespace only")
            elif any(c in input_data for c in ["<", ">", "&"]):
                if mode == "strict":
                    errors.append("HTML chars in strict mode")
                elif mode == "permissive":
                    warnings.append("HTML chars detected")
                    transformed = input_data.replace("<", "&lt;").replace(">", "&gt;")
                else:
                    transformed = input_data
            else:
                valid = True
                transformed = input_data
        elif mode == "permissive":
            if len(input_data) > 100000:
                errors.append("String too long even for permissive")
            else:
                valid = True
                transformed = input_data.strip()
                if not transformed:
                    warnings.append("Empty after strip")
                    if rules and any(r.get("strict_empty", False) for r in rules):
                        errors.append("Strict empty rule violated")
        elif mode == "silent":
            transformed = input_data[:50000] if input_data else ""
            valid = True
        else:
            errors.append(f"Unknown mode: {mode}")

    elif isinstance(input_data, (int, float)):
        if mode == "strict":
            if math.isnan(input_data):
                errors.append("NaN value")
            elif math.isinf(input_data):
                errors.append("Infinite value")
            elif input_data < 0:
                errors.append("Negative value not allowed")
            elif input_data > 1_000_000:
                warnings.append("Very large number")
            else:
                valid = True
                transformed = input_data
        elif mode == "permissive":
            if math.isnan(input_data) or math.isinf(input_data):
                warnings.append("Special float value")
                transformed = 0.0
            else:
                transformed = float(input_data)
            valid = True
        else:
            transformed = float(input_data) if not math.isnan(input_data) else 0.0
            valid = True

    elif isinstance(input_data, (list, tuple)):
        if mode == "strict":
            if len(input_data) == 0:
                errors.append("Empty collection")
            elif len(input_data) > 1000:
                errors.append("Collection too large")
            else:
                all_valid = True
                for i, item in enumerate(input_data):
                    if item is None:
                        if rules and any(r.get("allow_none", False) for r in rules):
                            continue
                        else:
                            warnings.append(f"Item {i} is None")
                            all_valid = False
                    elif isinstance(item, dict):
                        if "id" not in item:
                            warnings.append(f"Item {i} missing id")
                if all_valid:
                    valid = True
                    transformed = input_data
                else:
                    errors.append("Some items invalid")
        elif mode == "permissive":
            filtered = [x for x in input_data if x is not None]
            if filtered:
                valid = True
                transformed = filtered
            else:
                warnings.append("All items were None")
                transformed = []
                valid = True
        else:
            transformed = [x for x in input_data if x is not None][:100]
            valid = True

    elif isinstance(input_data, dict):
        if mode == "strict":
            required_keys = ["id", "name", "type"] if not rules else []
            if rules:
                for rule in rules:
                    if "required_keys" in rule:
                        required_keys.extend(rule["required_keys"])
            missing = [k for k in required_keys if k not in input_data]
            if missing:
                errors.append(f"Missing keys: {missing}")
            else:
                valid = True
                transformed = input_data
        elif mode == "permissive":
            valid = True
            transformed = {k: v for k, v in input_data.items() if v is not None}
            if not transformed:
                warnings.append("All values were None")
        else:
            valid = True
            transformed = input_data

    elif isinstance(input_data, bytes):
        try:
            decoded = input_data.decode("utf-8")
            if mode == "strict":
                if len(decoded) > 50000:
                    errors.append("Decoded bytes too long")
                else:
                    valid = True
                    transformed = decoded
            else:
                valid = True
                transformed = decoded[:50000]
        except UnicodeDecodeError:
            if mode == "strict":
                errors.append("Bytes not UTF-8 decodable")
            else:
                warnings.append("Fallback to latin-1")
                transformed = input_data.decode("latin-1", errors="replace")
                valid = True
        except LookupError:
            errors.append("Unknown encoding")
    else:
        errors.append(f"Unsupported type: {type(input_data).__name__}")

    # Apply transformers
    if valid and transformed is not None:
        for t in transformers:
            if t == "default":
                pass
            elif t == "lower":
                if isinstance(transformed, str):
                    transformed = transformed.lower()
            elif t == "upper":
                if isinstance(transformed, str):
                    transformed = transformed.upper()
            elif t == "strip":
                if isinstance(transformed, str):
                    transformed = transformed.strip()
            elif t == "deduplicate":
                if isinstance(transformed, list):
                    seen = set()
                    deduped = []
                    for x in transformed:
                        h = hash(str(x))
                        if h not in seen:
                            seen.add(h)
                            deduped.append(x)
                    transformed = deduped
            elif t == "sort":
                if isinstance(transformed, list):
                    try:
                        transformed = sorted(transformed)
                    except TypeError:
                        warnings.append("Cannot sort mixed types")
            elif t == "compact":
                if isinstance(transformed, dict):
                    transformed = {k: v for k, v in transformed.items() if v is not None}
            elif t == "flatten":
                if isinstance(transformed, list):
                    flat = []
                    for item in transformed:
                        if isinstance(item, list):
                            flat.extend(item)
                        else:
                            flat.append(item)
                    transformed = flat
            elif t == "truncate":
                if isinstance(transformed, str) and len(transformed) > 1000:
                    transformed = transformed[:1000]
            elif t == "round_floats":
                if isinstance(transformed, (int, float)):
                    transformed = round(float(transformed), 2)
            else:
                warnings.append(f"Unknown transformer: {t}")

    return valid, transformed, errors + warnings


class ComplexityValidator:
    """
    A class with intentionally over-complex methods to trigger Radon violations.
    """

    def __init__(
        self,
        base_config: Optional[Dict[str, Any]] = None,
        strict_mode: bool = True,
        max_depth: int = 10,
    ):
        self.config = base_config or {}
        self.strict_mode = strict_mode
        self.max_depth = max_depth
        self._internal_state = {}
        self._cache = {}
        self._history = []

    def deep_validate_with_routing(
        self,
        payload: Any,
        context: Optional[Dict[str, Any]] = None,
        routing_key: Optional[str] = None,
    ) -> Dict[str, Any]:
        """
        Cyclomatic complexity: 35+ — a routing nightmare of nested conditions.
        """
        ctx = context or {}
        result = {"pass": False, "checks": [], "errors": [], "routing": routing_key}

        if routing_key is None:
            routing_key = ctx.get("default_route", "standard")
            result["routing"] = routing_key

        if routing_key == "standard":
            if isinstance(payload, dict):
                if "type" in payload:
                    ptype = payload["type"]
                    if ptype == "user":
                        if "email" in payload:
                            if "@" in payload["email"]:
                                if "." in payload["email"].split("@")[-1]:
                                    result["checks"].append("email_valid")
                                    if "name" in payload:
                                        if len(payload["name"]) > 0:
                                            if len(payload["name"]) < 200:
                                                result["checks"].append("name_valid")
                                            else:
                                                result["errors"].append("name too long")
                                        else:
                                            result["errors"].append("name empty")
                                    else:
                                        if self.strict_mode:
                                            result["errors"].append("name required")
                                        else:
                                            result["checks"].append("name_missing")
                                else:
                                    result["errors"].append("email no tld")
                            else:
                                result["errors"].append("email no at")
                        else:
                            result["errors"].append("email required")
                    elif ptype == "admin":
                        if "role" in payload:
                            role = payload["role"]
                            if role == "super":
                                if "token" in payload:
                                    if len(payload["token"]) > 32:
                                        result["checks"].append("super_admin")
                                    else:
                                        result["errors"].append("token too short")
                                else:
                                    result["errors"].append("token required")
                            elif role == "moderator":
                                if "permissions" in payload:
                                    perms = payload["permissions"]
                                    if isinstance(perms, list):
                                        if len(perms) > 0:
                                            result["checks"].append("moderator")
                                        else:
                                            result["errors"].append("no permissions")
                                    else:
                                        result["errors"].append("permissions not list")
                                else:
                                    result["warnings"].append("no permissions defined")
                            elif role == "viewer":
                                result["checks"].append("viewer")
                                if "restrictions" in payload:
                                    result["checks"].append("restricted_viewer")
                            else:
                                result["errors"].append(f"unknown role: {role}")
                        else:
                            result["errors"].append("role required")
                    elif ptype == "system":
                        if "command" in payload:
                            cmd = payload["command"]
                            if cmd == "ping":
                                result["checks"].append("ping")
                            elif cmd == "status":
                                if "service" in payload:
                                    result["checks"].append(f"status_{payload['service']}")
                                else:
                                    result["errors"].append("service required for status")
                            elif cmd == "reload":
                                if "target" in payload:
                                    result["checks"].append("reload")
                                else:
                                    result["errors"].append("target required")
                            else:
                                result["errors"].append(f"unknown command: {cmd}")
                        else:
                            result["errors"].append("command required")
                    elif ptype == "data":
                        if "format" in payload:
                            fmt = payload["format"]
                            if fmt == "csv":
                                result["checks"].append("csv")
                            elif fmt == "json":
                                result["checks"].append("json")
                            elif fmt == "xml":
                                result["checks"].append("xml")
                            elif fmt == "yaml":
                                result["checks"].append("yaml")
                            elif fmt == "binary":
                                result["checks"].append("binary")
                            else:
                                result["errors"].append(f"unknown format: {fmt}")
                        else:
                            result["errors"].append("format required")
                    elif ptype == "event":
                        if "action" in payload:
                            action = payload["action"]
                            if action == "create":
                                result["checks"].append("event_create")
                            elif action == "update":
                                result["checks"].append("event_update")
                            elif action == "delete":
                                result["checks"].append("event_delete")
                            elif action == "archive":
                                result["checks"].append("event_archive")
                            else:
                                result["errors"].append(f"unknown action: {action}")
                        else:
                            result["errors"].append("action required")
                    elif ptype == "config":
                        if "key" in payload and "value" in payload:
                            result["checks"].append("config_pair")
                            if isinstance(payload["value"], dict):
                                result["checks"].append("nested_config")
                        else:
                            result["errors"].append("config needs key and value")
                    elif ptype == "batch":
                        if "items" in payload:
                            items = payload["items"]
                            if isinstance(items, list):
                                if len(items) > 0:
                                    result["checks"].append(f"batch_{len(items)}")
                                else:
                                    result["errors"].append("empty batch")
                            else:
                                result["errors"].append("batch items not list")
                        else:
                            result["errors"].append("batch missing items")
                    else:
                        result["errors"].append(f"unknown type: {ptype}")
                else:
                    result["errors"].append("payload missing type")
            elif isinstance(payload, (list, tuple)):
                if len(payload) == 0:
                    result["errors"].append("empty list payload")
                else:
                    for i, item in enumerate(payload):
                        if i >= self.max_depth:
                            break
                        if isinstance(item, dict):
                            if "type" in item:
                                result["checks"].append(f"item_{i}_typed")
                            else:
                                result["warnings"].append(f"item_{i}_untyped")
                        else:
                            result["checks"].append(f"item_{i}_scalar")
                    result["pass"] = True
            elif isinstance(payload, str):
                if len(payload) > 0:
                    result["checks"].append("string_payload")
                    if len(payload) > 1000:
                        result["warnings"].append("large string")
                    else:
                        result["pass"] = True
                else:
                    result["errors"].append("empty string payload")
            elif isinstance(payload, (int, float)):
                result["checks"].append("numeric_payload")
                if payload > 0:
                    result["pass"] = True
                else:
                    result["warnings"].append("non-positive number")
            elif payload is None:
                result["errors"].append("null payload")
            else:
                result["errors"].append(f"unexpected payload type: {type(payload).__name__}")

        elif routing_key == "fast_track":
            if payload is not None:
                result["pass"] = True
                result["checks"].append("fast_track_pass")
            else:
                result["errors"].append("fast_track requires non-null")

        elif routing_key == "audit":
            if isinstance(payload, dict):
                keys = list(payload.keys())
                if len(keys) > 0:
                    for k in keys:
                        if k.startswith("_"):
                            result["warnings"].append(f"private key: {k}")
                        elif k.startswith("audit_"):
                            result["checks"].append(f"audit_{k}")
                        else:
                            result["checks"].append(f"field_{k}")
                    result["pass"] = True
                else:
                    result["errors"].append("empty dict for audit")
            else:
                result["errors"].append("audit requires dict payload")

        elif routing_key == "dry_run":
            result["pass"] = True
            result["checks"].append("dry_run_noop")

        elif routing_key == "recovery":
            if isinstance(payload, dict) and "error" in payload:
                err = payload["error"]
                if isinstance(err, str):
                    if "timeout" in err.lower():
                        result["checks"].append("retry")
                    elif "rate" in err.lower():
                        result["checks"].append("backoff")
                    elif "auth" in err.lower():
                        result["checks"].append("reauth")
                    elif "not found" in err.lower():
                        result["checks"].append("recreate")
                    else:
                        result["checks"].append("generic_recovery")
                    result["pass"] = True
                elif isinstance(err, dict):
                    if "code" in err:
                        code = err["code"]
                        if code >= 500:
                            result["checks"].append("server_error_recovery")
                        elif code >= 400:
                            result["checks"].append("client_error_recovery")
                        else:
                            result["checks"].append("unknown_error")
                        result["pass"] = True
                    else:
                        result["errors"].append("error dict has no code")
                else:
                    result["errors"].append("unexpected error type")
            else:
                result["errors"].append("recovery requires error in payload")
        else:
            result["errors"].append(f"unknown routing_key: {routing_key}")

        # Final pass determination
        if not result["errors"]:
            if result["checks"]:
                result["pass"] = True
            else:
                result["warnings"].append("no checks performed")
                result["pass"] = self.strict_mode is False

        return result


def calculate_nested_metric_blob(
    values: List[Union[int, float, None, str]],
    weights: Optional[Dict[str, float]] = None,
    method: str = "weighted",
) -> Dict[str, Any]:
    """
    Cyclomatic complexity: 25+ — nested metric calculation nightmare.
    """
    if weights is None:
        weights = {"default": 1.0}

    result = {
        "count": 0, "sum": 0.0, "mean": 0.0, "median": 0.0,
        "std": 0.0, "min": None, "max": None, "valid": False
    }

    numeric = []
    for v in values:
        if v is None:
            continue
        if isinstance(v, str):
            try:
                numeric.append(float(v))
            except (ValueError, TypeError):
                continue
        elif isinstance(v, (int, float)):
            if not math.isnan(v) and not math.isinf(v):
                numeric.append(float(v))
            else:
                continue
        else:
            continue

    if len(numeric) == 0:
        return result

    result["count"] = len(numeric)
    result["sum"] = sum(numeric)
    result["min"] = min(numeric)
    result["max"] = max(numeric)
    result["mean"] = result["sum"] / result["count"]

    sorted_vals = sorted(numeric)
    n = len(sorted_vals)
    if n % 2 == 0:
        result["median"] = (sorted_vals[n // 2 - 1] + sorted_vals[n // 2]) / 2.0
    else:
        result["median"] = float(sorted_vals[n // 2])

    if n > 1:
        variance = sum((x - result["mean"]) ** 2 for x in numeric) / (n - 1)
        result["std"] = math.sqrt(variance)
    else:
        result["std"] = 0.0

    result["valid"] = True

    # Apply weighting method with even more nested garbage
    if method == "weighted":
        w = weights.get("primary", weights.get("default", 1.0))
        w2 = weights.get("secondary", 0.5)
        w3 = weights.get("tertiary", 0.25)
        result["weighted_sum"] = result["sum"] * w
        result["weighted_mean"] = result["mean"] * w
        if result["std"] > 0:
            cv = result["std"] / result["mean"]
            if cv < 0.1:
                result["dispersion"] = "very_low"
            elif cv < 0.3:
                result["dispersion"] = "low"
            elif cv < 0.5:
                result["dispersion"] = "moderate"
            elif cv < 0.8:
                result["dispersion"] = "high"
            else:
                result["dispersion"] = "very_high"
        else:
            result["dispersion"] = "none"
        if result["min"] is not None and result["max"] is not None:
            rng = result["max"] - result["min"]
            if rng > 0:
                result["normalized_range"] = rng / result["max"]
            else:
                result["normalized_range"] = 0.0
    elif method == "flat":
        result["flat_sum"] = result["sum"]
    elif method == "boosted":
        boost = weights.get("boost", 2.0)
        result["boosted_sum"] = result["sum"] * boost
    elif method == "clamped":
        clamp_min = weights.get("clamp_min", 0.0)
        clamp_max = weights.get("clamp_max", 100.0)
        result["clamped_mean"] = max(clamp_min, min(clamp_max, result["mean"]))
    elif method == "log":
        if result["sum"] > 0:
            result["log_sum"] = math.log(result["sum"])
            result["log_mean"] = math.log(result["mean"])
        else:
            result["log_sum"] = 0.0
            result["log_mean"] = 0.0
    elif method == "exp":
        result["exp_sum"] = math.exp(result["sum"] / result["count"])
    elif method == "percentile":
        if n >= 3:
            idx_p25 = max(0, min(n - 1, int(n * 0.25)))
            idx_p50 = max(0, min(n - 1, int(n * 0.50)))
            idx_p75 = max(0, min(n - 1, int(n * 0.75)))
            result["p25"] = sorted_vals[idx_p25]
            result["p50"] = sorted_vals[idx_p50]
            result["p75"] = sorted_vals[idx_p75]
            iqr = result["p75"] - result["p25"]
            result["iqr"] = iqr
            if iqr > 0:
                lower = result["p25"] - 1.5 * iqr
                upper = result["p75"] + 1.5 * iqr
                outliers = [x for x in numeric if x < lower or x > upper]
                result["outlier_count"] = len(outliers)
                result["outlier_ratio"] = len(outliers) / n
            else:
                result["outlier_count"] = 0
                result["outlier_ratio"] = 0.0
        else:
            result["p25"] = sorted_vals[0]
            result["p50"] = sorted_vals[-1]
            result["p75"] = sorted_vals[-1]
            result["iqr"] = 0.0
            result["outlier_count"] = 0
            result["outlier_ratio"] = 0.0
    elif method == "zscore":
        if result["std"] > 0:
            zscores = [(x - result["mean"]) / result["std"] for x in numeric]
            result["zscore_min"] = min(zscores)
            result["zscore_max"] = max(zscores)
            result["zscore_mean"] = sum(zscores) / len(zscores)
            beyond_2sigma = sum(1 for z in zscores if abs(z) > 2)
            result["beyond_2sigma"] = beyond_2sigma
            beyond_3sigma = sum(1 for z in zscores if abs(z) > 3)
            result["beyond_3sigma"] = beyond_3sigma
        else:
            result["zscore_min"] = 0.0
            result["zscore_max"] = 0.0
            result["zscore_mean"] = 0.0
            result["beyond_2sigma"] = 0
            result["beyond_3sigma"] = 0
    elif method == "custom":
        if "multiplier" in weights:
            m = weights["multiplier"]
            result["custom_sum"] = result["sum"] * m
            result["custom_mean"] = result["mean"] * m
        if "offset" in weights:
            o = weights["offset"]
            result["offset_sum"] = result["sum"] + o
        if "power" in weights:
            p = weights["power"]
            result["powered_mean"] = result["mean"] ** p
    else:
        result["method_error"] = f"Unknown method: {method}"

    return result
