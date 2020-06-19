import json

log_file = open('/Users/bytedance/log/ttt.log')
for line in log_file.readlines():
    data = json.loads(line)
    # print(data)
    # print(data['message'])
    # print(data['message'][2:5])
    msg = data['message']
    cost = msg[len("[LocalBenchmarkTimer] final_smart_search_result_total: "):len(msg)]
    # print(cost)
    if (int(cost) > 2000):
        print(data)
