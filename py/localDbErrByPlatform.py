import csv

with open(
    '/Users/bytedance/Downloads/本地 db 获取错误统计/aeolus_sql_toutiao-26000874_1595475675_gb18030.csv',
        'r') as csvFile:
    reader = csv.DictReader(csvFile)
    # [
    #   '70834793603',
    #   7,
    #   '{"err":"Error(LibCache(Msg(\\"get_datas failed: Error(Diesel(DatabaseError(__Unknown, \\\\\\"no such table: search_index\\\\\\")), State { next_error: None, backtrace: InternalBacktrace { backtrace: None } })\\")), State { next_error: None, backtrace: InternalBacktrace { backtrace: None } })","key":"9186d553","realTime":"2020-07-14T16:40:47Z","table":"Chat"}'
    # ]
    deviceIdSet = set()
    bf = 'get_datas failed: '
    ef = ', State'
    reasonMap = {}  # {platformStr : {errmsg : counter}}
    commonKey4NoTbl = 'no such table'
    commonKey4SyntaxErr = 'fts5: syntax error near'
    commonKey4NoClm = 'no such column'
    pv = 0

    mac = '7'
    windows = '8'
    ios = '2'
    android = '3'

    for row in reader:
        pv += 1
        # print(row['device_id'], row['params'])
        deviceIdSet.add(row['device_id'])
        platform = row['platform']
        errmsg = row['params']
        b = errmsg.find(bf)
        # print(b)
        # print(item[b])
        e = errmsg.find(ef)
        # print(e)
        # print(item[e])
        # print(item[b+len(bf): e])
        target = errmsg[b+len(bf): e]
        if -1 != target.find(commonKey4NoTbl):
            # print('bingo!!!')
            target = commonKey4NoTbl
        elif -1 != target.find(commonKey4SyntaxErr):
            target = commonKey4SyntaxErr
        elif -1 != target.find(commonKey4NoClm):
            target = commonKey4NoClm
        
        platformStr = ''
        if mac == platform:
            platformStr = 'mac'
        elif windows == platform:
            platformStr = 'windows'
        elif ios == platform:
            platformStr = 'ios'
        elif android == platform:
            platformStr = 'android'
        else:
            raise Exception('invalid platform!!!')

        errMsgMap = {}
        if True == reasonMap.__contains__(platformStr):
            errMsgMap = reasonMap[platformStr]
        else:
            reasonMap[platformStr] = errMsgMap
        
        if True == errMsgMap.__contains__(target):
            errMsgMap[target] = errMsgMap[target] + 1
        else:
            errMsgMap[target] = 1
        # break

    print('近一周上报 uv: ' + str(len(deviceIdSet)))
    print('近一周上报 pv: ' + str(pv))
    print('错误分布:')
    print(reasonMap)
