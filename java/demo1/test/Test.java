package demo1.test;

public class Test {
    public void print(String s, IPrinter printer) {
        boolean res = printer.print(s);
        System.out.println("res: " + res);
    }
}
