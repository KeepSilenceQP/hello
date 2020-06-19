package demo3;

import java.util.Arrays;
import java.util.List;

public class ParallelTest {
    public static void main(String[] args) {
        List<String> l = Arrays.asList("hello", "hi", "world", "morning", "moon");
        l.stream()
            // .parallel() // 并发处理, 顺序会变.
            .filter(s -> s.startsWith("h"))
            .map(String::toUpperCase) // 方法引用.
            .forEach(System.out::println);
    }
}
