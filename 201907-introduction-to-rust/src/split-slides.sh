#!/bin/sh

echo '
# Summary

- [Slide 1](slide-1.md)' > SUMMARY.md

rm slide-*.md

awk -F/ '
 BEGIN{file=1}
 /^-----/{close(("slide-" file ".md"));file = file+1; print("- [Slide " file "](slide-" file ".md)") >> "SUMMARY.md"; next}
 //{print >> ("slide-" file ".md"); }
' all-slides.md
