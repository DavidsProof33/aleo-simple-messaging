# examples/deploy_testnet.ps1
# Deploy the Leo program using WSL.
#
# Required env vars:
#   PRIVATE_KEY = "APrivateKey1..."
#   LEO_DIR     = "/home/<user>/aleo-simple-messaging/leo/simple_messaging"   # WSL path
#
# Optional:
#   NETWORK_ID     = "testnet"
#   ENDPOINT_URL   = "https://api.explorer.provable.com/v1"
#   PRIORITY_FEES  = "1000"
#   BROADCAST      = "1"   (set to 0 for dry run deploy without broadcast)

if (-not $env:PRIVATE_KEY) { throw "Missing env var: PRIVATE_KEY" }
if (-not $env:LEO_DIR)     { throw "Missing env var: LEO_DIR (WSL path, e.g. /home/<user>/...)" }

if (-not $env:NETWORK_ID)    { $env:NETWORK_ID = "testnet" }
if (-not $env:ENDPOINT_URL)  { $env:ENDPOINT_URL = "https://api.explorer.provable.com/v1" }
if (-not $env:PRIORITY_FEES) { $env:PRIORITY_FEES = "1000" }
if (-not $env:BROADCAST)     { $env:BROADCAST = "1" }

Write-Host "== Aleo Simple Messaging: Deploy ==" -ForegroundColor Cyan
Write-Host "LEO_DIR       : $env:LEO_DIR"
Write-Host "NETWORK_ID    : $env:NETWORK_ID"
Write-Host "ENDPOINT_URL  : $env:ENDPOINT_URL"
Write-Host "PRIORITY_FEES : $env:PRIORITY_FEES"
Write-Host "BROADCAST     : $env:BROADCAST"
Write-Host ""

# Build
wsl.exe bash -lc "cd '$env:LEO_DIR' && leo build"
if ($LASTEXITCODE -ne 0) { throw "leo build failed" }

# Deploy
$deploy = "cd '$env:LEO_DIR' && leo deploy --private-key `"$env:PRIVATE_KEY`" --network $env:NETWORK_ID --endpoint `"$env:ENDPOINT_URL`" --priority-fees `"$env:PRIORITY_FEES`""
if ($env:BROADCAST -eq "1") {
  $deploy += " --broadcast --yes"
}
wsl.exe bash -lc $deploy
if ($LASTEXITCODE -ne 0) { throw "leo deploy failed" }

Write-Host ""
Write-Host "Deploy finished." -ForegroundColor Green
