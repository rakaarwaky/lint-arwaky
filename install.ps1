# Auto-Linter Installer for Windows
# Cross-platform: Supports Windows (PowerShell)
# Usage: 
#   Local: powershell -ExecutionPolicy Bypass -File install.ps1
#   Remote: Invoke-WebRequest -Uri https://raw.githubusercontent.com/rakaarwaky/auto-linter/main/install.ps1 | Invoke-Expression

param(
    [string]$PythonVersion = "3.12"
)

$ErrorActionPreference = "Stop"

# Colors for output
function Write-Bold { param([string]$Message) Write-Host -ForegroundColor White $Message }
function Write-Green { param([string]$Message) Write-Host -ForegroundColor Green $Message }
function Write-Yellow { param([string]$Message) Write-Host -ForegroundColor Yellow $Message }
function Write-Red { param([string]$Message) Write-Host -ForegroundColor Red $Message }

# Banner
Write-Bold @"

  _         _        _     _                        
 /_\  _   _| |_ ___ | |   (_)_ __  _   ___  __     
//_\| | | | __/ _ \| |   | | '_ \| | | \ \/ /    
/  _ \ |_| | || (_) | |___| | | | | |_| |>  <     
\_/ \_/\__,_|\__\___/|_____|_|_| |_|\__,_/_/\_\   
                                                 
  Lint Architecture for Python & JavaScript/TypeScript
"@

Write-Green "  Detected: Windows"

# Check Python
Write-Bold "[1/5] Checking Python..."

$Python = $null

# Try to find Python
$pythonCommands = @("python$PythonVersion", "python3.13", "python3.12", "python3", "python")
foreach ($cmd in $pythonCommands) {
    try {
        $ver = & $cmd -c "import sys; print(f'{sys.version_info.major}.{sys.version_info.minor}')" 2>$null
        if ($ver) {
            $major = [int]($ver.Split('.')[0])
            $minor = [int]($ver.Split('.')[1])
            if ($major -ge 3 -and $minor -ge 12) {
                $Python = $cmd
                Write-Green "  Found: $cmd ($ver)"
                break
            }
        }
    } catch { continue }
}

if (-not $Python) {
    Write-Red "  Python >= 3.12 not found!"
    Write-Host "  Install Python 3.12+ first:"
    Write-Host "    Option 1: Download from https://www.python.org/downloads/"
    Write-Host "    Option 2: Using winget: winget install Python.Python.3.12"
    Write-Host "    Option 3: Using Chocolatey: choco install python312"
    exit 1
}

# Choose install method
Write-Bold ""
Write-Bold "[2/5] Install method:"

$InstallMethod = ""
if (Get-Command uv -ErrorAction SilentlyContinue) {
    $InstallMethod = "uv"
    Write-Green "  Using uv (recommended)"
} elseif (Get-Command pip -ErrorAction SilentlyContinue) {
    $InstallMethod = "pip"
    Write-Host "  Using pip"
} elseif (Get-Command pip3 -ErrorAction SilentlyContinue) {
    $InstallMethod = "pip"
    Write-Host "  Using pip3"
} else {
    Write-Red "  No pip or uv found!"
    exit 1
}

# Install
Write-Bold ""
Write-Bold "[3/5] Installing auto-linter..."

if ($InstallMethod -eq "uv") {
    uv tool install auto-linter 2>$null || uv pip install auto-linter
} else {
    & $Python -m pip install --user auto-linter
}

# Verify installation
$autoLintCmd = Get-Command auto-lint -ErrorAction SilentlyContinue
$autoLinterCmd = Get-Command auto-linter -ErrorAction SilentlyContinue

if ($autoLintCmd) {
    Write-Green "  Installed: $($autoLintCmd.Source)"
} elseif ($autoLinterCmd) {
    Write-Green "  Installed: $($autoLinterCmd.Source)"
} else {
    # Try to find LOCAL_BIN dynamically
    $userBase = & $Python -m site --user-base 2>$null
    if ($userBase) {
        $localBin = "$userBase\Scripts"
        if (Test-Path "$localBin\auto-lint.exe") {
            Write-Yellow "  auto-lint is at $localBin\auto-lint.exe"
            Write-Yellow "  Add to PATH: `$env:PATH += ';$localBin'"
            $env:PATH += ";$localBin"
        }
    }
}

# Init config
Write-Bold ""
Write-Bold "[4/5] Initializing configuration..."

$autoLintCmd = Get-Command auto-lint -ErrorAction SilentlyContinue
if ($autoLintCmd) {
    & auto-lint setup init
} else {
    Write-Yellow "  Could not find auto-lint command for init"
    Write-Host "  Run manually: auto-lint setup init"
}

# Done
Write-Bold ""
Write-Bold "[5/5] Done!"

Write-Green ""
Write-Green "Auto-Linter is ready."
Write-Host ""
Write-Host "Quick start:"
Write-Host "  auto-lint check .\src           # lint your code"
Write-Host "  auto-lint setup doctor          # diagnose issues"
Write-Host "  auto-lint setup mcp-config    # get MCP server config"
Write-Host ""
Write-Host "As MCP server:"
Write-Host "  auto-linter                 # start MCP server (stdio)"
Write-Host ""
Write-Host "For MCP clients (Claude, Hermes, VS Code):"
Write-Host "  auto-lint setup mcp-config --client claude"
Write-Host "  auto-lint setup mcp-config --client hermes"