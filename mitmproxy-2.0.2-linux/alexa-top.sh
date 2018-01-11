#!/bin/sh
n=1000
mkdir alexa-top$n
for URL in $(head -n $n top-1m.csv | cut -d, -f2-) ; do
	echo $URL
	./mitmdump -z -w "alexa-top$n/$URL.js-flows" --anticache ~t javascript &
	sleep 3s
	firefox -P mitmproxy $URL &
	# chromium-browser $URL &
	sleep 15s
	killall mitmdump
	pkill firefox
	# pkill chromium-browser
	sleep 5s
done
