cd doc
#rmdir --ingnore-fail-on-non-empty ./output
mkdir -p ./output
pdflatex -output-directory=./output -output-format=pdf main.tex
biber --input-directory=./output --output-directory=./output main.bcf
pdflatex -output-directory=./output -output-format=pdf main.tex
pdflatex -output-directory=./output -output-format=pdf main.tex