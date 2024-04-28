.PHONY: readme

readme: README.md

README.md: t4t/src/lib.rs
	@ cargo readme -r t4t --no-title --no-indent-headings > README.md
