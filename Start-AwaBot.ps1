foreach ($line in Get-Content -Path '.\.env') {
    if ($line.Length -eq 0) {
        continue
    }
    ($name, $value) = $line.Split('=')
    $null = New-Item -Path "Env:\$name" -Value $value -Force
    Remove-Variable -Name 'line','name','value'
}

cargo run --release
