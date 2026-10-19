# Look for unused dependencies
check :
	cargo +nightly check -Zcargo-lints --workspace --all-targets

# Prune unused dependencies
prune:
	cargo shear --fix
	cargo check --all-targets

# Install python dependencies
python-setup:
	python3 -m pip install --target ./scripts/libs/ lxml svgpathtools scour pyyaml
