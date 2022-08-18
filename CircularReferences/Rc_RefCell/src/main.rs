

use core::cell::RefCell;
use std::rc::Rc;

struct Student {
    name: String,
    courses: Vec<Rc<RefCell<Course>>>
}


impl Student {
    fn new(name: &str) -> Student {
        Student {
            name: name.into(),
            courses: Vec::new()
        }
    }
}

struct Course {
    name: String,
    students: Vec<Rc<RefCell<Student>>>
}

impl Course {
    fn new(name: &str) -> Course {
        Course {
            name: name.into(),
            students: Vec::new()
        }
    }
}

fn add_student(
    course: Rc<RefCell<Course>>,
    student: Rc<RefCell<Student>>
) {
    student.borrow_mut().courses.push(course.clone());
    course.borrow_mut().students.push(student);
}

fn main() {

    /*
        Doing this we invalidate some of the protection
        Of the Rust compiler

        Also it is ugly as fuck
    */

    let john = Rc::new(
        RefCell::new(
            Student::new("John")
        )
    );

    let jane = Rc::new(
        RefCell::new(
            Student::new("Jane")
        )
    );

    let course = Course::new("Rust Course");
    let magic_course = Rc::new(RefCell::new(course));
    //You have to clone magic_course all the time
    //because otherwise it will be moved
    add_student(magic_course.clone(), john);
    add_student(magic_course, jane);

    //Having to manage the lifetimes all the time and
    //Ownerships is a pain in the ass so we have a third option
}
