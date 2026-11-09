# scripts/validate.ps1 — Full Quality Gate & Pre-Commit Validator
# Runs Rust fmt, tests, frontend tsc, and production build

Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host " Running Full Platform Quality Gate" -ForegroundColor Cyan
Write-Host "==========================================================" -ForegroundColor Cyan

$allPassed = $true

# 1. Rust Backend Tests
Write-Host "`n[1/3] Running Rust workspace tests..." -ForegroundColor Yellow
Push-Location "backend"
cargo test --workspace
if ($LASTEXITCODE -ne 0) {
    Write-Host "  ✗ Rust tests failed!" -ForegroundColor Red
    $allPassed = $false
} else {
    Write-Host "  ✓ All Rust tests passed!" -ForegroundColor Green
}
Pop-Location

# 2. Frontend Strict TypeScript Typecheck
Write-Host "`n[2/3] Running frontend TypeScript typecheck..." -ForegroundColor Yellow
Push-Location "frontend"
npx tsc --noEmit
if ($LASTEXITCODE -ne 0) {
    Write-Host "  ✗ TypeScript compilation failed!" -ForegroundColor Red
    $allPassed = $false
} else {
    Write-Host "  ✓ TypeScript check passed (0 errors)!" -ForegroundColor Green
}

# 3. Frontend Production Build
Write-Host "`n[3/3] Testing frontend Vite production build..." -ForegroundColor Yellow
npm run build
if ($LASTEXITCODE -ne 0) {
    Write-Host "  ✗ Vite build failed!" -ForegroundColor Red
    $allPassed = $false
} else {
    Write-Host "  ✓ Vite production bundle built successfully!" -ForegroundColor Green
}
Pop-Location

Write-Host "`n==========================================================" -ForegroundColor Cyan
if ($allPassed) {
    Write-Host " ✅ ALL QUALITY GATES PASSED! Ready for merge / release." -ForegroundColor Green
} else {
    Write-Host " ❌ ONE OR MORE QUALITY GATES FAILED. Review errors above." -ForegroundColor Red
}
Write-Host "==========================================================" -ForegroundColor Cyan
