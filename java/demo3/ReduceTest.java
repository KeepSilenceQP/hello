package demo3;

import java.util.List;
// import java.util.Optional;
import java.util.Arrays;

public class ReduceTest {
    public static void main(String[] args) {
        List<Integer> i = Arrays.asList(1, 2, 12, 34, 35);
        // Optional<Integer> o = i.stream() // Optional 为了便于处理 null 情形, 代表一种可能性.
        //     .filter(e -> e > 10)
        //     .findAny();
        // System.out.println(o.get());

        // i.stream()
        //     .filter(e -> e > 10)
        //     .findAny()
        //     .ifPresent(e -> System.out.println(e));

        // 归约.
        // int res = i.stream()
        //     .reduce(0, (subTotal, e) -> subTotal + e);
        // System.out.println("res: " + res);

        int res1 = i.stream()
            .filter( v -> v > 40)
            .findFirst()
            .orElse(100); // 找不到就用 100 默认值返回.
        System.out.println(res1);
    }
}
