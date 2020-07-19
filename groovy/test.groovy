//def x = false
//print x.getClass().getCanonicalName()

// def aList = [false, '''can
// new line''', 100]
// println aList

// assert aList[2] == 100

// aList[8] = true

// println aList.size()

// key = 'hhh'
// def aMap = [key : 'www']
// def aMap1 = [(key) : 'www']
// println aMap
// println aMap1

// aMap.ttt = 'kkk'
// println aMap

// def aRange = 1..3
// println aRange
// println aRange.from
// println aRange.to
// println aRange.size()

// def greeting = {
//     println "hello, $it!"
// }

// greeting("qp")

// def greeting1 = {
//     name ->
//     println "hello, $name!"
// }

// greeting1("qp")

def testClosure(String s, int i, Closure closure) {
    closure(s, i)
}

testClosure "test", 1, {
    s, i ->
    println "$s hhh $i"
}

// groovy test.groovy
// gradle 使用 groovy 作为开发语言, 配合 plugin 提供 dsl "黑话" api 方便构建.
