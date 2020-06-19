package demo2;

import java.util.Arrays;
import java.util.List;

// map 起到的作用是转换, flatMap 则能够处理高维度转换, 将
// 子管道中的数据平面展开到父管道中进行处理.
public class FlatMapTest {
    public static void main(String[] args) {
        List<String> ss = Arrays.asList("Hello", "World");
        ss.stream()
            .map(s -> s.split(""))
            .forEach(System.out::println);
        
        ss.stream()
            .flatMap(s -> Arrays.stream(s.split("")))
            .forEach(System.out::println);
    }
}
