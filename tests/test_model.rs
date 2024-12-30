use packrat::model::task::{Task, TaskManager, BytesConvertible};

#[test]
fn test_task() {
    
    // create task
    let task = Task::new(String::from("clean"), String::from("do the dishes"));
    assert_eq!(task.name, "clean");
    assert_eq!(task.desc, "do the dishes");
        
    // serialize and deserialize
    let ser_task = task.to_bytes();
    let de_task = Task::from_bytes(&ser_task);
    assert_eq!(de_task, task);
}

#[test]
fn test_task_manager() {

    let tm = TaskManager::new("/tmp/test.db"); 
    

    let task1 = tm.create_task(
        String::from("run"),
        String::from("run around the block")
    );
     
    let task2 = tm.create_task(
        String::from("soccer"),
        String::from("play some footy mate")
    );
     
    assert_eq!(tm.get_task(task2.id).unwrap(), task2); 
    assert_eq!(tm.get_task(task1.id).unwrap(), task1);

}
