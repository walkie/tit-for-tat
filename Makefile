.PHONY: readme

readme: t4t/README.md

t4t/README.md: t4t/src/lib.rs
	@ cargo readme -r t4t --no-title --no-indent-headings > t4t/README.md
