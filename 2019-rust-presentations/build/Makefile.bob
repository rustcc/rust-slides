BOBS:=$(wildcard *.bob)
SVGS:=$(patsubst %.bob,%.svg,$(BOBS))

.PHONY: all clean

all: $(SVGS)

clean:
	rm -f $(SVGS)

$(SVGS): %.svg: %.bob
	svgbob $< > $@
