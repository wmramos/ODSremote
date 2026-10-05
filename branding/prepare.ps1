param([ValidateSet('qs', 'agent')][string]$Edition)
$ErrorActionPreference = 'Stop'
$repo = Split-Path -Parent $PSScriptRoot
foreach ($name in @('icon.png', 'icon.ico', 'logo.png', 'logo_light.png', 'logo_dark.png')) {
    Copy-Item -LiteralPath (Join-Path $PSScriptRoot $name) -Destination (Join-Path $repo "flutter/assets/$name") -Force
}
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'icon.ico') -Destination (Join-Path $repo 'res/icon.ico') -Force
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'icon.ico') -Destination (Join-Path $repo 'flutter/windows/runner/resources/app_icon.ico') -Force
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'icon.png') -Destination (Join-Path $repo 'libs/portable/src/res/label.png') -Force
$resource = Join-Path $repo 'flutter/windows/runner/Runner.rc'
$text = [IO.File]::ReadAllText($resource)
$product = if ($Edition -eq 'qs') { 'ODSremote Quick Support' } else { 'ODSremote Agent' }
$text = $text.Replace('"Purslane Tech Pte. Ltd."', '"OneDot Systems"')
$text = $text.Replace('"RustDesk Remote Desktop"', "`"$product`"")
$text = $text.Replace('"ProductName", "RustDesk"', "`"ProductName`", `"$product`"")
[IO.File]::WriteAllText($resource, $text, [Text.UTF8Encoding]::new($false))
