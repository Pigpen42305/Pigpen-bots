foreach ($line in Get-Content -Path '.\.env') {
    if ($line.Length -eq 0) {
        continue
    }
    ($name, $value) = $line.Split('=')
    New-Item -Path "Env:\$name" -Value $value
    Remove-Variable -Name 'line','name','value'
}

cargo.exe run --release
