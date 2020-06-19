package demo2;

import java.util.Arrays;
import java.util.Comparator;
import java.util.List;
// import java.util.stream.Collectors;

public class StreamTest2 {
    public static void main(String[] args) {
        List<Student> l = Arrays.asList(
            new Student("mike", 15, "M", false),
            new Student("dancy", 16, "F", false),
            new Student("elton", 22, "M", true),
            new Student("allo", 12, "M", false),
            new Student("julia", 30, "F", true),
            new Student("summer", 18, "F", false),
            new Student("nancy", 16, "F", false)
        );

        // l.stream()
        //     .filter(i -> i.getGender().equals("F") && i.getIsGraduate())
        //     .forEach(System.out::println);

        // 使用方法引用 + 谓词表达式.
        // negate代表对前一个谓词的否定.
        // l.stream()
        //     .filter(Student.ageAbove15.and(Student.isFemale).negate().and(Student::getIsGraduate))
        //     .forEach(System.out::println);

        // l.stream()
        //     .map(s -> {
        //         s.setAge(s.getAge() + 1);
        //         if (s.getAge() > 18) {
        //             s.setIsGraduate(true);
        //         }
        //         s.setGender(s.getGender().equals("F") ? "Female" : "Male");
        //         return s;
        //     })
        //     .collect(Collectors.toList())
        //     .forEach(System.out::println);

        // peek 是 map 的特例.
        // l.stream()
        //     .peek(s -> {
        //         s.setAge(s.getAge() + 1);
        //         if (s.getAge() > 18) {
        //             s.setIsGraduate(true);
        //         }
        //         s.setGender(s.getGender().equals("F") ? "Female" : "Male");
        //     })
        //     .collect(Collectors.toList())
        //     .forEach(System.out::println);

        // 排序.
        // reversed 会影响到之前操作.
        l.stream()
            .sorted(Comparator.comparing(Student::getGender)
                                .reversed()
                                .thenComparing(Student::getAge)
                                .reversed())
            .forEach(System.out::println);
    }
}
