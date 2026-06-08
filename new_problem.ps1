param(
    [Parameter(Mandatory=$true)]
    [ValidateSet("cf","lc","gpu")]
    [string]$Platform,

    [Parameter(Mandatory=$true)]
    [string]$Name,

    [Parameter(Mandatory=$false)]
    [ValidateSet("cpp","rs","py","cu")]
    [string]$Lang = "cpp"
)

$root = $PSScriptRoot

$platformMap = @{
    "cf"  = "codeforces"
    "lc"  = "leetcode"
    "gpu" = "leetgpu"
}
$langDir = @{
    "cpp" = "cpp"
    "rs"  = "rust"
    "py"  = "python"
    "cu"  = "cuda"
}
$templateMap = @{
    "cf-cpp" = "cf_template.cpp"
    "cf-rs"  = "cf_template.rs"
    "cf-py"  = "cf_template.py"
    "lc-cpp" = "lc_template.cpp"
    "lc-rs"  = "lc_template.rs"
    "lc-py"  = "lc_template.py"
    "gpu-cu" = "gpu_template.cu"
}

$key = "$Platform-$Lang"
if (-not $templateMap.ContainsKey($key)) {
    Write-Error "Combinacion no soportada: $Platform + $Lang"
    exit 1
}

$destDir = Join-Path $root "$($platformMap[$Platform])\$($langDir[$Lang])"
if (-not (Test-Path $destDir)) {
    New-Item -ItemType Directory -Force $destDir | Out-Null
}

$destFile = Join-Path $destDir "$Name.$Lang"
if (Test-Path $destFile) {
    Write-Warning "Ya existe: $destFile"
    exit 0
}

$templateFile = Join-Path $root "templates\$($templateMap[$key])"
Copy-Item $templateFile $destFile

Write-Host "Creado: $destFile" -ForegroundColor Green
