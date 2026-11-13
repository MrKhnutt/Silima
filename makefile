# Look for unused dependencies
check :
	cargo +nightly check -Zcargo-lints --workspace --all-targets

# Prune unused dependencies
prune:
	cargo shear --fix
	cargo check --all-targets

# Install python dependencies
python-setup:
	python3 -m pip install --target ./scripts/libs/ lxml svgpathtools scour pyyaml pillow cairosvg

.PHONY: security-distributable
.PHONY: usage-distributable

# distributable pdf generation through LaTeX
security-distributable:
	@set -eu; \
		build_dir=$$(mktemp -d); \
		trap 'rm -rf "$$build_dir"' EXIT; \
		cd documentation/security; \
		latexmk -pdf -interaction=nonstopmode -halt-on-error \
			-jobname=SECURITY \
			-outdir="$$build_dir" main.tex; \
		cp "$$build_dir/SECURITY.pdf" ../../Silima-Information-Security-Reference.pdf

# distributable pdf generation through LaTeX
usage-distributable:
	@set -eu; \
		build_dir=$$(mktemp -d); \
		trap 'rm -rf "$$build_dir"' EXIT; \
		cd documentation/fair-use; \
		latexmk -pdf -interaction=nonstopmode -halt-on-error \
			-jobname=FAIRUSE \
			-outdir="$$build_dir" main.tex; \
		cp "$$build_dir/FAIRUSE.pdf" ../../Silima-Game-Content-Usage.pdf
