package demo1.main;

import demo1.test.*;

// lambda 表达式演示.
public class Hello {
    public static void main(final String[] args) {
        final Test t = new Test();

        // 可以一窥 lambda 表达式本质.
        // 有意思的是, 在 jvm 层, lambda 表达式是通过类内部方法实现的,
        // 也就是说, 表达式内部的 this 和匿名内部类并没有什么关系.
        IPrinter p = s -> {
            if (null == s || s.isEmpty()) {
                return false;
            }
            System.out.println(s);
            return true;
        };

        t.print("hi", p);

        t.print("", p);
    }
}
