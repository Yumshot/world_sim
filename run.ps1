# run_cargo.ps1
for ($i = 1; $i -le 5; $i++) {
    cargo run
    Start-Sleep -Seconds 0.5
}

# Prevent the window from closing
Write-Host "Press any key to exit..."
$x = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")