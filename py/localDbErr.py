import csv

with open(
    '/Users/bytedance/Downloads/本地 db 获取错误统计/aeolus_sql_toutiao-25967804_1595415233_gb18030.csv',
        'r') as csvFile:
    reader = csv.DictReader(csvFile)
    # [
    #   '70834793603',
    #   '{"err":"Error(LibCache(Msg(\\"get_datas failed: Error(Diesel(DatabaseError(__Unknown, \\\\\\"no such table: search_index\\\\\\")), State { next_error: None, backtrace: InternalBacktrace { backtrace: None } })\\")), State { next_error: None, backtrace: InternalBacktrace { backtrace: None } })","key":"9186d553","realTime":"2020-07-14T16:40:47Z","table":"Chat"}'
    # ]
    deviceIdSet = set()
    bf = 'get_datas failed: '
    ef = ', State'
    reasonMap = {}
    commonKey4NoTbl = 'no such table'
    commonKey4SyntaxErr = 'fts5: syntax error near'
    commonKey4NoClm = 'no such column'
    pv = 0
    for row in reader:
        pv += 1
        # print(row['device_id'], row['params'])
        deviceIdSet.add(row['device_id'])
        item = row['params']
        b = item.find(bf)
        # print(b)
        # print(item[b])
        e = item.find(ef)
        # print(e)
        # print(item[e])
        # print(item[b+len(bf): e])
        target = item[b+len(bf): e]
        if -1 != target.find(commonKey4NoTbl):
            # print('bingo!!!')
            target = commonKey4NoTbl
        elif -1 != target.find(commonKey4SyntaxErr):
            target = commonKey4SyntaxErr
        elif -1 != target.find(commonKey4NoClm):
            target = commonKey4NoClm
        if True == reasonMap.__contains__(target):
            reasonMap[target] = reasonMap.get(target) + 1
        else:
            reasonMap[target] = 1
        # break

    print('近一周上报 uv: ' + str(len(deviceIdSet)))
    print('近一周上报 pv: ' + str(pv))
    print('错误分布:')
    print(reasonMap)
