
//Many to many relationships are complicated in Rust

struct Student<'a>  {
    name: String,
    course: Vec<&'a Course>
}

impl<'a> Student<'a> {
    fn new(name: &str) -> Student<'a> {
        Student {
            name: name.into(),
            courses: Vec::new()
        }
    }

    fn add_student(&'a mut self,
        student: &'a mut Student<'a>) {
        student.courses.push(self);
        self.students.push(student);
    }
}

struct Course<'a> {
    name: String,
    students: Vec<&'a Student<'a>>
}

impl<'a> Course<'a> {
    fn new(name: &str) -> Course<'a> {
        Course {
            name: name.into(),
            students: Vec::new()
        }
    }
}

fn main() {
    let john = Student::new("John");
    let course = Course::new("Rust Course");

    //If we drop Course first we will be referencing a course
    //that doesn't exist anymore
    //If we drop john the course will reference a student
    //that no longer exists

    //This situation cannot be addresed by normal means
    //but it can be solved using Rc and RefCell
    course.add_student(john); // Rc

}
