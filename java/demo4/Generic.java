package demo4;

import java.util.ArrayList;
import java.util.List;

class Generic {
    public static void main(String[] args) {
        List<Bag<? extends Fruite>> f = new ArrayList<>();
        f.add(new Bag<Fruite>(new Fruite()));
        f.add(new Bag<Apple>(new Apple()));
        f.add(new Bag<Banana>(new Banana()));

        f.stream().forEach(t -> t.sayHi());

        // 运行时泛型信息被擦除, JVM 感知不到;
        // 其所感知到的 Bag 类相当于将 T 替换为 Fruit 的效果,
        // 所以运行时类型转换没有报错.
        Bag<Apple> b = (Bag<Apple>) f.get(2);
        b.sayHi();

        Bag<? extends Fruite> b1 = f.get(0);
        b1.sayHi();
    }
}

class Bag<T extends Fruite> {

    T mT;

    Bag(T t) {
        mT = t;
    }

    void sayHi() {
        mT.sayHi();
    }
}

class Fruite {
    public void sayHi() {
        System.out.println("Fruite");
    }
}

class Apple extends Fruite {
    @Override
    public void sayHi() {
        System.out.println("Apple");
    }
}

class Banana extends Fruite {
    @Override
    public void sayHi() {
        System.out.println("Banana");
    }
}
