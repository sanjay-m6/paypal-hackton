# scripts/setup.ps1 — Platform Setup & Environment Initializer
# Team Lead: Sanjay · AI Engineer: Kaviya

Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host " Multi-Agent Payment Security Platform — Setup & Init" -ForegroundColor Cyan
Write-Host "==========================================================" -ForegroundColor Cyan

# 1. Check Toolchain Versions
Write-Host "`n[1/4] Checking prerequisites..." -ForegroundColor Yellow

$rustVersion = rustc --version 2>$null
if ($rustVersion) {
    Write-Host "  ✓ Rust toolchain: $rustVersion" -ForegroundColor Green
} else {
    Write-Error "Rust is not installed. Please install Rust via rustup.rs"
}

$nodeVersion = node --version 2>$null
if ($nodeVersion) {
    Write-Host "  ✓ Node.js: $nodeVersion" -ForegroundColor Green
} else {
    Write-Error "Node.js is not installed. Please install Node.js LTS"
}

# 2. Check / Copy .env file
Write-Host "`n[2/4] Initializing environment configuration..." -ForegroundColor Yellow
if (-not (Test-Path ".env")) {
    Copy-Item ".env.example" ".env"
    Write-Host "  ✓ Created .env from .env.example template" -ForegroundColor Green
} else {
    Write-Host "  ✓ Existing .env file detected" -ForegroundColor Green
}

# 3. Backend Dependency Check
Write-Host "`n[3/4] Checking Rust workspace crates..." -ForegroundColor Yellow
Push-Location "backend"
cargo check --workspace
if ($LASTEXITCODE -eq 0) {
    Write-Host "  ✓ All 7 backend crates compiled cleanly" -ForegroundColor Green
} else {
    Write-Host "  ✗ Backend compilation failed" -ForegroundColor Red
}
Pop-Location

# 4. Frontend Dependency Check
Write-Host "`n[4/4] Checking frontend dependencies..." -ForegroundColor Yellow
Push-Location "frontend"
npm install --silent
if ($LASTEXITCODE -eq 0) {
    Write-Host "  ✓ Frontend packages verified" -ForegroundColor Green
} else {
    Write-Host "  ✗ Frontend npm install failed" -ForegroundColor Red
}
Pop-Location

Write-Host "`n==========================================================" -ForegroundColor Cyan
Write-Host " Setup complete! Ready for development." -ForegroundColor Cyan
Write-Host " Run 'scripts/validate.ps1' to run all verification checks." -ForegroundColor Cyan
Write-Host "==========================================================" -ForegroundColor Cyan

