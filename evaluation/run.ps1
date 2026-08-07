$ErrorActionPreference = "Stop"
if ($args.Count -gt 1 -or ($args.Count -eq 1 -and $args[0] -ne "--report")) {
  [Console]::Error.WriteLine("evaluation: only the optional --report flag is accepted")
  exit 2
}
$root = $PSScriptRoot
$arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString()
if (-not $IsWindows -or $arch -ne "X64") {
  Get-Content -LiteralPath (Join-Path $root "unsupported-transcript.txt")
  exit 3
}
$target = "win32-x64"
$tmp = Join-Path ([IO.Path]::GetTempPath()) ("terminal-jarvis-evaluation-" + [guid]::NewGuid())
New-Item -ItemType Directory -Path $tmp | Out-Null
try {
  $binary = Join-Path $root "payloads/win32-x64/terminal-jarvis.exe"
  if (-not (Test-Path -LiteralPath $binary -PathType Leaf)) { throw "verified payload missing for $target" }
  $env:TERMINAL_JARVIS_HOME = Join-Path $tmp "home"
  $env:TERMINAL_JARVIS_CATALOG = Join-Path $root "catalogs/harnesses"
  $env:TERMINAL_JARVIS_GATES = Join-Path $root "catalogs/gates"
  $env:TERMINAL_JARVIS_GATE = "off"
  Write-Output "SIMULATED EVALUATION — no coding-agent harness will be executed"
  & $binary --plain version --verbose
  & $binary --plain list
  & $binary --plain show codex
  & $binary --plain plan codex version
  if ($args.Count -eq 1) {
    $digest = (Get-FileHash -Algorithm SHA256 (Join-Path $root "manifest-v1.json")).Hash.ToLower()
    $version = ((& $binary --version) -split " ")[1]
    $ref = ((& $binary --plain version --verbose | Select-String '^git commit: ').Line -replace '^git commit: ', '')
    [ordered]@{schema_version=1;kit_digest="sha256:$digest";selected_target=$target;terminal_jarvis_version=$version;terminal_jarvis_ref=$ref;scenario_results=@([ordered]@{code="TJ-EVAL-001";result="pass"})} | ConvertTo-Json -Compress -Depth 4
  }
} finally {
  Remove-Item -LiteralPath $tmp -Recurse -Force -ErrorAction SilentlyContinue
}
