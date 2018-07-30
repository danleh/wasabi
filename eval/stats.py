#!/usr/bin/python
import sys
import numpy
data = sys.stdin.read().splitlines()
data = numpy.array(data).astype(numpy.float)
print "median",numpy.median(data)
print "mean",numpy.mean(data)
print "stdev",numpy.std(data)
