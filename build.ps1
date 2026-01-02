$ErrorActionPreference = 'Stop'

# Ensure PyO3 ABI3 for Python 3.10+ compatibility
$env:PYO3_USE_ABI3 = '1'

# Build release wheel
maturin build --release

# Create dist folder if it doesn't exist
if (-Not (Test-Path -Path "dist")) { New-Item -ItemType Directory -Path "dist" }

# Copy wheels to dist
Get-ChildItem -Path "target\wheels\*.whl" | ForEach-Object {
    Copy-Item -Path $_.FullName -Destination "dist\" -Force
}

# Uninstall existing version
pip uninstall serdejsonpy -y

# Install the wheel
$wheel = Get-ChildItem -Path "dist\*.whl" | Select-Object -First 1
pip install $wheel.FullName

# Run example
python .\examples\example.py
