#!/usr/bin/python
import sys
to_insert = sys.stdin.read().replace("'", "").replace("\n", "\\n")
pspdfkit = open(sys.argv[1]).read()
modified = pspdfkit.replace("return n(476)('", "return n(476)('" + to_insert)
print modified

