# Look for unused dependencies
check :
	cargo +nightly check -Zcargo-lints --workspace --all-targets

# Prune unused dependencies
prune:
	cargo shear --fix
	cargo check --all-targets