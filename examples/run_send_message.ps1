# examples/run_send_message.ps1
# Runs the Rust CLI to execute the `send_message` transition via WSL.
#
# Required env vars:
#   LEO_DIR      = "/home/<user>/simple_messaging"  # WSL path to Leo project
#   SENDER       = "aleo1..."
#   RECIPIENT    = "aleo1..."
#   PRIVATE_KEY  = "APrivateKey1..."
#
# Optional:
#   NETWORK_ID    = "testnet"
#   ENDPOINT_URL  = "https://api.explorer.provable.com/v1"   # base endpoint
#   PRIORITY_FEE  = "1000"
#   MSG_ID        = "1field"
#   DATA0         = "10field"
#   DATA1         = "20field"
#   DATA2         = "30field"
#   BROADCAST     = "1"  (set to 0 for local execute only)

if (-not $env:LEO_DIR)     { throw "Missing env var: LEO_DIR (WSL path, e.g. /home/<user>/simple_messaging)" }
if (-not $env:SENDER)      { throw "Missing env var: SENDER" }
if (-not $env:RECIPIENT)   { throw "Missing env var: RECIPIENT" }
if (-not $env:PRIVATE_KEY) { throw "Missing env var: PRIVATE_KEY" }

if (-not $env:NETWORK_ID)   { $env:NETWORK_ID = "testnet" }
if (-not $env:ENDPOINT_URL) { $env:ENDPOINT_URL = "https://api.explorer.provable.com/v1" }
if (-not $env:PRIORITY_FEE) { $env:PRIORITY_FEE = "1000" }
if (-not $env:BROADCAST)    { $env:BROADCAST = "1" }

if (-not $env:MSG_ID) { $env:MSG_ID = "1field" }
if (-not $env:DATA0)  { $env:DATA0  = "10field" }
if (-not $env:DATA1)  { $env:DATA1  = "20field" }
if (-not $env:DATA2)  { $env:DATA2  = "30field" }

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$crateDir = Join-Path $repoRoot "rust\simple_messaging_cli"

Write-Host "== Aleo Simple Messaging: run_send_message ==" -ForegroundColor Cyan
Write-Host "crateDir     : $crateDir"
Write-Host "leo_dir (WSL): $env:LEO_DIR"
Write-Host "sender       : $env:SENDER"
Write-Host "recipient    : $env:RECIPIENT"
Write-Host "msg_id       : $env:MSG_ID"
Write-Host "data0        : $env:DATA0"
Write-Host "data1        : $env:DATA1"
Write-Host "data2        : $env:DATA2"
Write-Host "network      : $env:NETWORK_ID"
Write-Host "endpoint     : $env:ENDPOINT_URL"
Write-Host "priority_fee : $env:PRIORITY_FEE"
Write-Host "broadcast    : $env:BROADCAST"
Write-Host ""

Push-Location $crateDir
try {
  $argsList = @(
    "--leo-dir",      "$env:LEO_DIR",
    "--sender",       "$env:SENDER",
    "--recipient",    "$env:RECIPIENT",
    "--msg-id",       "$env:MSG_ID",
    "--data0",        "$env:DATA0",
    "--data1",        "$env:DATA1",
    "--data2",        "$env:DATA2",
    "--private-key",  "$env:PRIVATE_KEY",
    "--network",      "$env:NETWORK_ID",
    "--endpoint",     "$env:ENDPOINT_URL",
    "--priority-fee", "$env:PRIORITY_FEE"
  )

  if ($env:BROADCAST -eq "1") { $argsList += "--broadcast" }
  $argsList += "--run"

  cargo run -- @argsList
  if ($LASTEXITCODE -ne 0) { throw "cargo run failed" }
}
finally {
  Pop-Location
}

Write-Host ""
Write-Host "Done." -ForegroundColor Green
