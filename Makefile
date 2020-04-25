PANDOC = pandoc

all: slidy

slidy: slides.md 
	$(PANDOC) -t slidy --standalone --section-divs --highlight-style pygments slides.md -o rendered/index.html 

