import xlrd

class SearchFrequency:

    def __init__(self, platform, scene, pv, uv):
        self.platform = platform
        self.scene = scene
        self.pv = pv
        self.uv = uv

    def __repr__(self):
        return 'scene:' + self.scene + ' pv:' + str(self.pv) + ' uv:' + self.uv

# book = xlrd.open_workbook(
    # '/Users/bytedance/Downloads/演练/7-20 演练/25807000-编辑框1-查询3.xlsx')

book = xlrd.open_workbook(
    '/Users/bytedance/Downloads/演练/7-20 演练/25853944-编辑框1-查询4.xlsx')

# for sheet in book.sheets():
#     print(sheet.name)

assert 1 == len(book.sheets())

sheets = book.sheets()
# print(sheets[0].name)

sheet = book.sheet_by_name(sheets[0].name)

# print(sheet.nrows)

groupByPlatform = {}

for i in range(1, sheet.nrows):
    # print(sheet.row_values(i))
    row = sheet.row_values(i)
    sf = []
    if (False == groupByPlatform.__contains__(row[0])):
        groupByPlatform[row[0]] = sf
    else:
        sf = groupByPlatform[row[0]]
    sf.append(SearchFrequency(row[0], row[1], int(row[2]), row[3]))

for key in groupByPlatform.keys():
    value = groupByPlatform[key]
    print("key:" + key)
    sv = sorted(value, key=lambda searchFrequency: searchFrequency.pv, reverse=True)
    for i in range(len(sv)):
        print(sv[i])

# print(sheet.row_values(1))

# row = sheet.row_values(1)
# print(row[0])
