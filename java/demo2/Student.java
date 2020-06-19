package demo2;

import java.util.function.Predicate;

public class Student {
    private String name;
    private int age;
    private String gender;
    private boolean isGraduate;

    public Student(String name, int age, String gender, boolean isGraduate) {
        this.name = name;
        this.age = age;
        this.gender = gender;
        this.isGraduate = isGraduate;
    }

    public String getName() {
        return name;
    }

    public int getAge() {
        return age;
    }

    public String getGender() {
        return gender;
    }

    public boolean getIsGraduate() {
        return isGraduate;
    }

    public void setName(String name) {
        this.name = name;
    }

    public void setAge(int age) {
        this.age = age;
    }

    public void setGender(String gender) {
        this.gender = gender;
    }

    public void setIsGraduate(boolean isGraduate) {
        this.isGraduate = isGraduate;
    }
    
    @Override
    public String toString() {
        return "name: " + name + 
                " age: " + age + 
                " gender: " + gender +
                " isGraduate: " + isGraduate;
    }

    public static Predicate<Student> isFemale = s -> s.getGender().equals("F");
    public static Predicate<Student> ageAbove15 = s -> s.getAge() > 15;
}
