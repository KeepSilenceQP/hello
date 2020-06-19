package demo1.test;

// 函数接口注解.
// java 8 允许接口中定义 static 方法, default 方法以及 Object 中的方法,
// 一定程度上缓解了接口拓展面临的大范围修改问题.
@FunctionalInterface
public interface IPrinter {
    boolean print(String s);
}
