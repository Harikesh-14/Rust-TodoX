# Rust TodoX


Rust Todo List App is a command-line tool written in Rust that allows users to manage their tasks efficiently. Whether you need to add, mark as done, edit, or clear tasks from your todo list, this app provides essential functionalities to streamline your task management process. Additionally, I have integrated sqlite3 using the `rusqlite` crate. The database stores the data and will persist indefinitely until you manually delete it.

## Features
- **List tasks:** Display all tasks in the todo list.
- **Add task:** Add a new task to the todo list.
- **Complete task:** Mark a task as completed.
- **Edit task:** Modify an existing task.
- **Clear list:** Clear the todo list at once.

## How to Use

1. To use Rust File Operations, you need to have Rust installed on your system. You can install Rust from [here](https://www.rust-lang.org/tools/install).

2. Clone the repository using this command.
  ``` bash
    git clone https://github.com/Harikesh-14/Rust-TodoX.git
  ```
3. To generate the target folder, enter the given below command
  ``` bash
    cargo build
  ```
4. To run the program, use the given below rule
  ``` bash
    cargo run <operation> [task_id]
  ```

## Operations

### 1. Add Task
To add a new task to the todo list, use the following command:

``` rust
cargo run add
```
Now, you will be asked to enter the task name. Write it and press ```enter```.

### 2. Mark as done
To mark the command as 'mark as done', use the following command:

``` rust
cargo run done <task number>
```

### 3. Show the list of tasks
To show the list of tasks, use the following command:

``` rust
cargo run show
```

### 4. Update the task name
To update the tasks name, use the following command:

``` rust
cargo run edit <task number>
```
Now, you will be asked to enter the task name. Write it and press ```enter```.

### 5. Clear list
To clear the todo list at once, use the following command:

``` rust
cargo run clear all
```

## Conclusion
This TodoX provides a simple yet powerful way to manage your tasks using the Rust programming language. Whether you're tracking your daily to-dos or managing a project, this app has you covered.

If you encounter any issues or have suggestions for improvement, feel free to open an issue on our [GitHub repository](https://github.com/Harikesh-14/Rust-TodoX). Happy task managing with Rust! 🚀
