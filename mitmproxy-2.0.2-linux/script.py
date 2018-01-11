def response(flow):
	print(flow.request.method)
	print(flow.request.host)
	print(flow.request.path)
	print(flow.response.headers.get("Content-Type","none"))

	if flow.request.method == "GET" and "javascript" in flow.response.headers.get("Content-Type","none"):
		print("MATCHED")
	# print("headers:")
	# for k, v in flow.response.headers.items():
		# print("  %-20s: %s" % (k, v))

    # print("")
    # print("="*50)
    # #print("FOR: " + flow.request.url)
    # print(flow.request.method + " " + flow.request.path + " " + flow.request.http_version)

    # print("-"*50 + "request headers:")
    # for k, v in flow.request.headers.items():
    #     print("%-20s: %s" % (k.upper(), v))

    # print("-"*50 + "response headers:")
    # for k, v in flow.response.headers.items():
    #     print("%-20s: %s" % (k.upper(), v))
    #     print("-"*50 + "request headers:")

# def response(context, flow):
    # request_headers = [{"name": k, "value": v} for k, v in flow.request.headers]
#     response_headers = [{"name": k, "value": v} for k, v in flow.response.headers]
#     print(request_headers)
#     print(response_headers)
