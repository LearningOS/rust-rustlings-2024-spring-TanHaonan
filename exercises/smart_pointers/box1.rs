// box1.rs
//
// At compile time, Rust needs to know how much space a type takes up. This
// becomes problematic for recursive types, where a value can have as part of
// itself another value of the same type. To get around the issue, we can use a
// `Box` - a smart pointer used to store data on the heap, which also allows us
// to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the `cons list` - a
// data structure frequently found in functional programming languages. Each
// item in a cons list contains two elements: the value of the current item and
// the next item. The last item is a value called `Nil`.
//
// Step 1: use a `Box` in the enum definition to make the code compile
// Step 2: create both empty and non-empty cons lists by replacing `todo!()`
//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` or use the `hint` watch subcommand for a hint.

// 1.
// let arr1 = arr
// 栈数据所有权转移是深拷贝，重新复制一份数据并让原来的失效
// 堆数据所有权转移是指针转移，不会拷贝数据
#[derive(PartialEq, Debug)]
pub enum List {
    // Cons(数据，下一节点)
    Cons(i32, Box<List>),
    Nil,
}
// 2.
// 堆主要的用途是处理一些动态大小数据类型，比如上面的单向链表，可以通过Cons找到当下数据和下一个
// 这种递归数据类型大小未知，因此应该把成员List放到堆
// 

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    List::Nil
}

pub fn create_non_empty_list() -> List {
    // 创建一个只有一个数据的列表，其中下一个指针
    List::Cons(1125, Box::new(List::Nil))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
