package demo2;

import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

// 典型的 stream 应用,
// 切流 -> 过滤/转换/收集 -> 聚集.
public class StreamTest {

    public static void main(String[] args) {
        List<String> l = Arrays.asList("apple", "pencil", "earth", "moon", "ear");
        List<String> res = l.stream()
            .filter(s -> s.startsWith("e"))
            .map(String::toUpperCase)
            .sorted()
            .collect(Collectors.toList());

        res.stream().forEach(System.out::println);
    }

}
