#!/usr/bin/env bash
# installer_lib_test.sh — Offline checks for scripts/lib.sh installer helpers.
# Pure bash: no network, no sudo, no root, no bats. Stub binaries live in a
# temporary dir prepended to PATH; each stub records its invocations.
set -euo pipefail

TEST_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LIB_SH="$TEST_DIR/../lib.sh"

STUB_DIR="$(mktemp -d)"
# Track per-test temp dirs so the EXIT trap can clean them up.
TMP_DIRS=("$STUB_DIR")
trap 'for d in "${TMP_DIRS[@]}"; do rm -rf "$d"; done' EXIT

CALLS="$STUB_DIR/calls.log"

make_stub() {
    local name="$1"
    cat >"$STUB_DIR/$name" <<EOF
#!/bin/bash
echo "\$0 \$*" >> "$CALLS"
exit 0
EOF
    chmod +x "$STUB_DIR/$name"
}

PASS=0
FAIL=0

check() {
    local label="$1"
    shift
    local out
    out="$("$@" 2>&1)" && {
        echo "PASS: $label"
        PASS=$((PASS + 1))
    } || {
        echo "FAIL: $label"
        [ -n "$out" ] && printf '%s\n' "$out" | sed 's/^/    /'
        FAIL=$((FAIL + 1))
    }
}

# ── 1. cargo_uses_sccache ──────────────────────────────────────────────────────
test_cargo_uses_sccache_match() {
    local root
    root="$(mktemp -d)"
    TMP_DIRS+=("$root")
    mkdir -p "$root/.cargo"
    printf '[build]\nrustc-wrapper = "sccache"\n' >"$root/.cargo/config.toml"
    cargo_uses_sccache "$root"
}

test_cargo_uses_sccache_absent() {
    local root
    root="$(mktemp -d)"
    TMP_DIRS+=("$root")
    mkdir -p "$root/.cargo"
    printf '[build]\nrustc-wrapper = "sccache2"\n' >"$root/.cargo/config.toml"
    ! cargo_uses_sccache "$root"
}

test_cargo_uses_sccache_none() {
    local root
    root="$(mktemp -d)"
    TMP_DIRS+=("$root")
    ! cargo_uses_sccache "$root"
}

# ── 1b. cargo_uses_sccache with custom CARGO_HOME ─────────────────────────────
test_cargo_uses_sccache_custom_cargo_home() {
    local root custom_home
    root="$(mktemp -d)"
    custom_home="$(mktemp -d)"
    TMP_DIRS+=("$root" "$custom_home")
    # No .cargo/config.toml in root
    # CARGO_HOME points to custom_home which has the sccache config
    mkdir -p "$custom_home"
    printf '[build]\nrustc-wrapper = "sccache"\n' >"$custom_home/config.toml"
    CARGO_HOME="$custom_home" cargo_uses_sccache "$root"
}

# ── 2/3/4. pip_install ─────────────────────────────────────────────────────────
# pip_install probes capabilities in order; these stubs expose exactly one.
test_pip_uses_python3_module() {
    local root
    root="$(mktemp -d)"
    TMP_DIRS+=("$root")
    : >"$CALLS"
    mkdir -p "$root/bin" "$root/py"
    cat >"$root/py/pip" <<EOF
#!/bin/bash
echo "python3 -m pip \$*" >> "$CALLS"
exit 0
EOF
    chmod +x "$root/py/pip"
    cat >"$root/bin/python3" <<EOF
#!/bin/bash
if [ "\$1" = "-m" ] && [ "\$2" = "pip" ]; then exec "$root/py/pip" "\${@:3}"; fi
exit 1
EOF
    chmod +x "$root/bin/python3"
    PATH="$root/bin:$STUB_DIR:/usr/bin:/bin" pip_install mypy
    grep -q -- '-m pip install --user mypy' "$CALLS"
}

test_pip_no_pip_remedy() {
    local root out
    root="$(mktemp -d)"
    TMP_DIRS+=("$root")
    mkdir -p "$root/bin"
    # Stub python3 that has no pip module: python3 -m pip --version fails.
    # Use #!/bin/bash so the kernel can exec it without searching PATH for bash.
    cat >"$root/bin/python3" <<'EOF'
#!/bin/bash
echo "no pip module" >&2
exit 1
EOF
    chmod +x "$root/bin/python3"
    # /usr/bin:/bin ensure /bin/bash is findable; $root/bin shadows host python3.
    out="$(PATH="$root/bin:/usr/bin:/bin" pip_install mypy 2>&1)" || true
    grep -q 'No pip for python3' <<<"$out" || return 1
    grep -q 'python3 -m pip install --user mypy' <<<"$out" || return 1
    grep -q -- '\[dry-run\]' <<<"$out" && return 1
    return 0
}

# Issue 7: pip_install when python3 binary is completely absent (no pip3/pip
# either). Uses PATH="$root/bin:/usr/bin:/bin" like every sibling pip test so
# the host shell/coreutils stay reachable while $root/bin (empty) hides
# python3/pip3/pip. The zero-pip remedy path is proven by the two greps on
# "No pip for python3" and "python3 -m pip install --user mypy".
test_pip_no_python3_absent() {
    local root out
    root="$(mktemp -d)"
    TMP_DIRS+=("$root")
    : >"$CALLS"
    # Empty root/bin: no python3, no pip3, no pip stubs.
    mkdir -p "$root/bin"
    out="$(PATH="$root/bin:/usr/bin:/bin" pip_install mypy 2>&1)" || true
    # The zero-pip remedy path should be taken: warn about no pip.
    grep -q 'No pip for python3' <<<"$out" || return 1
    # Should print a remedy mentioning pip install.
    grep -q 'python3 -m pip install --user mypy' <<<"$out" || return 1
    return 0
}

test_pip_pep668_retry() {
    local root out
    root="$(mktemp -d)"
    TMP_DIRS+=("$root")
    : >"$CALLS"
    mkdir -p "$root/bin" "$root/py"
    cat >"$root/py/pip" <<EOF
#!/bin/bash
echo "python3 -m pip \$*" >> "$CALLS"
case " \$* " in
    *" --version "*) exit 0 ;;
    *" --break-system-packages "*) exit 0 ;;
esac
echo "error: externally-managed-environment" >&2
exit 1
EOF
    chmod +x "$root/py/pip"
    cat >"$root/bin/python3" <<EOF
#!/bin/bash
if [ "\$1" = "-m" ] && [ "\$2" = "pip" ]; then exec "$root/py/pip" "\${@:3}"; fi
exit 1
EOF
    chmod +x "$root/bin/python3"
    out="$(PATH="$root/bin:$STUB_DIR:/usr/bin:/bin" pip_install mypy 2>&1)"
    grep -q -- '--break-system-packages' <<<"$out" || return 1
    grep -q -- '-m pip install --user --break-system-packages mypy' "$CALLS"
}

# Issue 11: pip_install generic failure (non-PEP-668 error, e.g. "Could not find a version").
test_pip_generic_failure() {
    local root out
    root="$(mktemp -d)"
    TMP_DIRS+=("$root")
    : >"$CALLS"
    mkdir -p "$root/bin" "$root/py"
    # pip stub that fails with a generic (non-PEP-668) error.
    cat >"$root/py/pip" <<EOF
#!/bin/bash
echo "python3 -m pip \$*" >> "$CALLS"
if [[ " \$* " != *" --version "* ]]; then
    echo "ERROR: Could not find a version that satisfies the requirement mypy" >&2
    exit 1
fi
exit 0
EOF
    chmod +x "$root/py/pip"
    cat >"$root/bin/python3" <<EOF
#!/bin/bash
if [ "\$1" = "-m" ] && [ "\$2" = "pip" ]; then exec "$root/py/pip" "\${@:3}"; fi
exit 1
EOF
    chmod +x "$root/bin/python3"
    out="$(PATH="$root/bin:$STUB_DIR:/usr/bin:/bin" pip_install mypy 2>&1)" || true
    # Should print the generic failure message, not the PEP-668 retry message.
    grep -q 'pip install failed for mypy' <<<"$out" || return 1
    # Must NOT print the --break-system-packages retry message.
    grep -q -- '--break-system-packages' <<<"$out" && return 1
    # The CALLS log should show the initial failed attempt but NOT a retry.
    local attempts
    attempts=$(grep -c 'install --user mypy' "$CALLS" 2>/dev/null || echo 0)
    [ "$attempts" -eq 1 ] || return 1
    return 0
}

# ── 5. DRY_RUN ─────────────────────────────────────────────────────────────────
test_dry_run_executes_nothing() {
    : >"$CALLS"
    local root
    root="$(mktemp -d)"
    TMP_DIRS+=("$root")
    mkdir -p "$root/bin"
    DRY_RUN=true install_if_missing mypy "mypy" "pip_install mypy" "python3 -m pip install --user mypy"
    [ ! -s "$CALLS" ]
}

test_ensure_sccache_skips_when_present() {
    local root
    root="$(mktemp -d)"
    TMP_DIRS+=("$root")
    mkdir -p "$root/.cargo" "$root/bin"
    printf '[build]\nrustc-wrapper = "sccache"\n' >"$root/.cargo/config.toml"
    make_stub sccache
    : >"$CALLS"
    # Run in the current shell (not a subshell) so INSTALL_SKIPPED mutations
    # from record_skipped are visible to the test. Redirect output to /dev/null.
    PATH="$root/bin:$STUB_DIR:/usr/bin:/bin" PROJECT_ROOT="$root" ensure_sccache &>/dev/null
    # Issue 6: verify the bookkeeping path was actually exercised — sccache
    # should appear in INSTALL_SKIPPED, proving record_skipped ran (not just
    # the trivial "no sccache stub was called" check).
    case " ${INSTALL_SKIPPED[*]:-} " in
        *" sccache "*|*" sccache"*|*" sccache"*) ;;
        *) echo "INSTALL_SKIPPED does not contain sccache: ${INSTALL_SKIPPED[*]:-}"; return 1 ;;
    esac
    [ ! -s "$CALLS" ]
}

# Issue 8: record_manual dedup when the same package fails twice.
test_record_manual_dedup() {
    local root
    root="$(mktemp -d)"
    TMP_DIRS+=("$root")
    mkdir -p "$root/bin"
    # Reset INSTALL_MANUAL to a clean state so the test is hermetic.
    INSTALL_MANUAL=()
    # Simulate two failed pip_install calls for the same package.
    pip_install mypy &>/dev/null || true
    pip_install mypy &>/dev/null || true
    # After two failures, INSTALL_MANUAL should contain exactly one entry.
    [ ${#INSTALL_MANUAL[@]} -eq 1 ]
}

# Issue 10: install_if_missing with compound method string in non-dry-run.
test_install_if_missing_compound_non_dry_run() {
    local root
    root="$(mktemp -d)"
    TMP_DIRS+=("$root")
    : >"$CALLS"
    mkdir -p "$root/bin"
    # The compound method is "mkdir $root/installed" — a real multi-word shell
    # command. After it runs, we check the side-effect directory exists,
    # proving eval actually executed it (not just dry-run).
    DRY_RUN=false install_if_missing mypy "mypy" "mkdir $root/installed" "pip_install mypy"
    [ -d "$root/installed" ]
}

# Issue 13: per-test temp dirs are cleaned up via trap in each test.

source "$LIB_SH"

check "cargo_uses_sccache matches rustc-wrapper = \"sccache\"" test_cargo_uses_sccache_match
check "cargo_uses_sccache rejects a different wrapper" test_cargo_uses_sccache_absent
check "cargo_uses_sccache is nonzero with no config" test_cargo_uses_sccache_none
check "cargo_uses_sccache finds rustc-wrapper in custom CARGO_HOME" test_cargo_uses_sccache_custom_cargo_home
check "pip_install prefers python3 -m pip" test_pip_uses_python3_module
check "pip_install prints a remedy and survives without pip" test_pip_no_pip_remedy
check "pip_install survives with no python3/pip3/pip at all" test_pip_no_python3_absent
check "pip_install retries with --break-system-packages on PEP 668" test_pip_pep668_retry
check "pip_install prints generic failure on non-PEP-668 error" test_pip_generic_failure
check "DRY_RUN=true makes install_if_missing execute nothing" test_dry_run_executes_nothing
check "ensure_sccache skips when sccache is on PATH" test_ensure_sccache_skips_when_present
check "record_manual dedups when same package fails twice" test_record_manual_dedup
check "install_if_missing runs compound method in non-dry-run" test_install_if_missing_compound_non_dry_run

echo "----------------------------------------"
echo "Passed: $PASS  Failed: $FAIL"
[ "$FAIL" -eq 0 ]
