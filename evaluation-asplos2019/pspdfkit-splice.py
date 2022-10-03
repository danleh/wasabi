#!/usr/bin/python
import sys

# this is just before the BLOB of JavaScript that the workers get initially (if it changes, you might find it by searching for the header comment, e.g., "* PSPDFKit for Web")
# we will prepend our own JS before theirs (but after this haystack)
HAYSTACK = "return n(382)('"

# escape single quotes and newlines
to_insert = sys.stdin.read().replace("'", "\\'").replace("\n", "\\n")

# pspdfkit.js library file from command line argument
pspdfkit = open(sys.argv[1]).read()

# insert our own code just before the worker JavaScript
if HAYSTACK not in pspdfkit:
	print >> sys.stderr, "could not find beginning of Worker JavaScript code, did not insert own code into PSPDFKit library, potentially broken!"
	exit(1)

modified = pspdfkit.replace(HAYSTACK, HAYSTACK + to_insert)
print modified
