
# need to be install : pandoc & asciidoc
# apt install pandoc asciidoc


asciidoc -b docbook specs.asciidoc
pandoc -f docbook -t markdown_strict specs.xml -o specs.md
rm -f specs.xml