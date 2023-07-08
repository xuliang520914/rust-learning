mod list{

    pub struct Tasks{
        pub item: String,
    }
}

mod things_todo;
use things_todo::add_activity;
use things_todo::items_completed;
use things_todo::items_completed::test::test;

fn let_add_task() {
    let task = list::Tasks{
        item: String::from("Tasks"),
    };

    add_activity(); // relative path
    // crate::add_activity(); // absolute path because we start a the root crate
    items_completed::remove_task();

    test();
}