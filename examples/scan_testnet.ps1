# examples/scan_testnet.ps1
# Scan private records using snarkOS (WSL).
#
# Required env vars:
#   VIEW_KEY   = "AViewKey1..."
#   SNARKOS_BIN = "/home/<user>/snarkOS/target/release/snarkos"
#
# Optional:
#   NETWORK_ID    = "1"   (Aleo Testnet network id for snarkOS)
#   ENDPOINT_URL  = "https://api.explorer.provable.com/v1"
#   LAST_N_BLOCKS = "200"

if (-not $env:VIEW_KEY)   { throw "Missing env var: VIEW_KEY" }
if (-not $env:SNARKOS_BIN){ throw "Missing env var: SNARKOS_BIN (e.g. /home/<user>/snarkOS/target/release/snarkos)" }

if (-not $env:NETWORK_ID)    { $env:NETWORK_ID = "1" }
if (-not $env:ENDPOINT_URL)  { $env:ENDPOINT_URL = "https://api.explorer.provable.com/v1" }
if (-not $env:LAST_N_BLOCKS) { $env:LAST_N_BLOCKS = "200" }

Write-Host "== Aleo Simple Messaging: Scan ==" -ForegroundColor Cyan
Write-Host "NETWORK_ID    : $env:NETWORK_ID"
Write-Host "ENDPOINT_URL  : $env:ENDPOINT_URL"
Write-Host "LAST_N_BLOCKS : $env:LAST_N_BLOCKS"
Write-Host ""

# Use bash -lc with explicit quoting to avoid argument splitting.
$cmd = "`"$env:SNARKOS_BIN`" developer scan --view-key `"$env:VIEW_KEY`" --network $env:NETWORK_ID --endpoint `"$env:ENDPOINT_URL`" --last $env:LAST_N_BLOCKS"
wsl.exe bash -lc $cmd
if ($LASTEXITCODE -ne 0) { throw "snarkos developer scan failed" }

Write-Host ""
Write-Host "Scan finished." -ForegroundColor Green
