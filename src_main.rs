use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

mod array;
mod singly_list;
mod doubly_list;
mod stack;
mod queue;
mod full_binary_tree;

use array::Array;
use singly_list::SinglyList;
use doubly_list::DoublyList;
use stack::Stack;
use queue::Queue;
use full_binary_tree::FullBinaryTree;

// Глобальные структуры данных
struct DataStructures {
    array: Array,
    singly_list: SinglyList,
    doubly_list: DoublyList,
    stack: Stack,
    queue: Queue,
    tree: FullBinaryTree,
}

impl DataStructures {
    fn new() -> Self {
        DataStructures {
            array: Array::new(10),
            singly_list: SinglyList::new(),
            doubly_list: DoublyList::new(),
            stack: Stack::new(10),
            queue: Queue::new(),
            tree: FullBinaryTree::new(),
        }
    }
}

// Загрузка данных из файлов
fn load_from_files(base_filename: &str, ds: &mut DataStructures) {
    // Загрузка массива
    let array_file = format!("{}_array.txt", base_filename);
    if Path::new(&array_file).exists() {
        if let Ok(file) = File::open(&array_file) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    for value in line.split_whitespace() {
                        ds.array.add_back(value.to_string());
                    }
                }
            }
            println!("Loaded array from {}", array_file);
        }
    }

    // Загрузка односвязного списка
    let slist_file = format!("{}_singly_list.txt", base_filename);
    if Path::new(&slist_file).exists() {
        if let Ok(file) = File::open(&slist_file) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    for value in line.split_whitespace() {
                        ds.singly_list.add_tail(value.to_string());
                    }
                }
            }
            println!("Loaded singly list from {}", slist_file);
        }
    }

    // Загрузка двусвязного списка
    let dlist_file = format!("{}_doubly_list.txt", base_filename);
    if Path::new(&dlist_file).exists() {
        if let Ok(file) = File::open(&dlist_file) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    for value in line.split_whitespace() {
                        ds.doubly_list.add_tail(value.to_string());
                    }
                }
            }
            println!("Loaded doubly list from {}", dlist_file);
        }
    }

    // Загрузка стека
    let stack_file = format!("{}_stack.txt", base_filename);
    if Path::new(&stack_file).exists() {
        if let Ok(file) = File::open(&stack_file) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    for value in line.split_whitespace() {
                        ds.stack.push(value.to_string());
                    }
                }
            }
            println!("Loaded stack from {}", stack_file);
        }
    }

    // Загрузка очереди
    let queue_file = format!("{}_queue.txt", base_filename);
    if Path::new(&queue_file).exists() {
        if let Ok(file) = File::open(&queue_file) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    for value in line.split_whitespace() {
                        ds.queue.enqueue(value.to_string());
                    }
                }
            }
            println!("Loaded queue from {}", queue_file);
        }
    }

    // Загрузка дерева
    let tree_file = format!("{}_tree.txt", base_filename);
    if Path::new(&tree_file).exists() {
        if let Ok(file) = File::open(&tree_file) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    for value in line.split_whitespace() {
                        ds.tree.insert(value.to_string());
                    }
                }
            }
            println!("Loaded tree from {}", tree_file);
        }
    }
}

// Сохранение данных в файлы
fn save_to_files(base_filename: &str, ds: &DataStructures) {
    // Сохранение массива
    let array_file = format!("{}_array.txt", base_filename);
    if let Ok(mut file) = OpenOptions::new().write(true).create(true).truncate(true).open(&array_file) {
        for value in ds.array.iter() {
            write!(file, "{} ", value).ok();
        }
        println!("Saved array to {}", array_file);
    }

    // Сохранение односвязного списка
    let slist_file = format!("{}_singly_list.txt", base_filename);
    if let Ok(mut file) = OpenOptions::new().write(true).create(true).truncate(true).open(&slist_file) {
        for value in ds.singly_list.iter() {
            write!(file, "{} ", value).ok();
        }
        println!("Saved singly list to {}", slist_file);
    }

    // Сохранение двусвязного списка
    let dlist_file = format!("{}_doubly_list.txt", base_filename);
    if let Ok(mut file) = OpenOptions::new().write(true).create(true).truncate(true).open(&dlist_file) {
        for value in ds.doubly_list.iter() {
            write!(file, "{} ", value).ok();
        }
        println!("Saved doubly list to {}", dlist_file);
    }

    // Сохранение стека
    let stack_file = format!("{}_stack.txt", base_filename);
    if let Ok(mut file) = OpenOptions::new().write(true).create(true).truncate(true).open(&stack_file) {
        let values: Vec<&String> = ds.stack.iter().collect();
        for value in values.iter().rev() {
            write!(file, "{} ", value).ok();
        }
        println!("Saved stack to {}", stack_file);
    }

    // Сохранение очереди
    let queue_file = format!("{}_queue.txt", base_filename);
    if let Ok(mut file) = OpenOptions::new().write(true).create(true).truncate(true).open(&queue_file) {
        for value in ds.queue.iter() {
            write!(file, "{} ", value).ok();
        }
        println!("Saved queue to {}", queue_file);
    }

    // Сохранение дерева
    let tree_file = format!("{}_tree.txt", base_filename);
    if let Ok(mut file) = OpenOptions::new().write(true).create(true).truncate(true).open(&tree_file) {
        for value in ds.tree.collect_inorder() {
            write!(file, "{} ", value).ok();
        }
        println!("Saved tree to {}", tree_file);
    }
}

// Обработка команд
fn process_command(query: &str, ds: &mut DataStructures) {
    let parts: Vec<&str> = query.split_whitespace().collect();
    if parts.is_empty() {
        return;
    }

    let command = parts[0].to_uppercase();

    match command.as_str() {
        // Команды для массива
        "MPUSH_BACK" => {
            if parts.len() > 1 {
                ds.array.add_back(parts[1].to_string());
                println!("Added \"{}\" to array", parts[1]);
            }
        }
        "MPUSH_INDEX" => {
            if parts.len() > 2 {
                if let Ok(index) = parts[1].parse::<usize>() {
                    match ds.array.add_index(index, parts[2].to_string()) {
                        Ok(_) => println!("Added \"{}\" at index {}", parts[2], index),
                        Err(e) => println!("Error: {}", e),
                    }
                }
            }
        }
        "MDEL" => {
            if parts.len() > 1 {
                if let Ok(index) = parts[1].parse::<usize>() {
                    match ds.array.remove_index(index) {
                        Ok(_) => println!("Removed element at index {}", index),
                        Err(e) => println!("Error: {}", e),
                    }
                }
            }
        }
        "MGET" => {
            if parts.len() > 1 {
                if let Ok(index) = parts[1].parse::<usize>() {
                    match ds.array.get_index(index) {
                        Ok(value) => println!("Array[{}] = \"{}\"", index, value),
                        Err(e) => println!("Error: {}", e),
                    }
                }
            }
        }
        "MREPLACE" => {
            if parts.len() > 2 {
                if let Ok(index) = parts[1].parse::<usize>() {
                    match ds.array.replace_index(index, parts[2].to_string()) {
                        Ok(_) => println!("Replaced array[{}] with \"{}\"", index, parts[2]),
                        Err(e) => println!("Error: {}", e),
                    }
                }
            }
        }
        "MLEN" => {
            println!("Array length: {}", ds.array.len());
        }
        "MPRINT" => {
            print!("Array: ");
            ds.array.print();
        }

        // Команды для односвязного списка
        "FPUSH_HEAD" => {
            if parts.len() > 1 {
                ds.singly_list.add_head(parts[1].to_string());
                println!("Added \"{}\" to list head", parts[1]);
            }
        }
        "FPUSH_TAIL" => {
            if parts.len() > 1 {
                ds.singly_list.add_tail(parts[1].to_string());
                println!("Added \"{}\" to list tail", parts[1]);
            }
        }
        "FDEL_HEAD" => {
            match ds.singly_list.remove_head() {
                Ok(_) => println!("Removed list head"),
                Err(e) => println!("Error: {}", e),
            }
        }
        "FDEL_TAIL" => {
            match ds.singly_list.remove_tail() {
                Ok(_) => println!("Removed list tail"),
                Err(e) => println!("Error: {}", e),
            }
        }
        "FDEL_VALUE" => {
            if parts.len() > 1 {
                match ds.singly_list.remove_value(parts[1]) {
                    Ok(_) => println!("Removed value \"{}\" from list", parts[1]),
                    Err(e) => println!("Error: {}", e),
                }
            }
        }
        "FSEARCH" => {
            if parts.len() > 1 {
                match ds.singly_list.find(parts[1]) {
                    Some(index) => println!("Value \"{}\" found at index {}", parts[1], index),
                    None => println!("Value \"{}\" not found", parts[1]),
                }
            }
        }
        "FPRINT" => {
            print!("Singly List: ");
            ds.singly_list.print();
        }

        // Команды для двусвязного списка
        "LPUSH_HEAD" => {
            if parts.len() > 1 {
                ds.doubly_list.add_head(parts[1].to_string());
                println!("Added \"{}\" to doubly list head", parts[1]);
            }
        }
        "LPUSH_TAIL" => {
            if parts.len() > 1 {
                ds.doubly_list.add_tail(parts[1].to_string());
                println!("Added \"{}\" to doubly list tail", parts[1]);
            }
        }
        "LDEL_HEAD" => {
            match ds.doubly_list.remove_head() {
                Ok(_) => println!("Removed doubly list head"),
                Err(e) => println!("Error: {}", e),
            }
        }
        "LDEL_TAIL" => {
            match ds.doubly_list.remove_tail() {
                Ok(_) => println!("Removed doubly list tail"),
                Err(e) => println!("Error: {}", e),
            }
        }
        "LSEARCH" => {
            if parts.len() > 1 {
                let found = ds.doubly_list.find(parts[1]);
                println!("Value \"{}\" {}", parts[1], if found { "found" } else { "not found" });
            }
        }
        "LPRINT_FORWARD" => {
            print!("Doubly List (forward): ");
            ds.doubly_list.print_forward();
        }
        "LPRINT_BACKWARD" => {
            print!("Doubly List (backward): ");
            ds.doubly_list.print_backward();
        }

        // Команды для стека
        "SPUSH" => {
            if parts.len() > 1 {
                ds.stack.push(parts[1].to_string());
                println!("Pushed \"{}\" to stack", parts[1]);
            }
        }
        "SPOP" => {
            match ds.stack.pop() {
                Ok(value) => println!("Popped from stack: \"{}\"", value),
                Err(e) => println!("Error: {}", e),
            }
        }
        "SPEEK" => {
            match ds.stack.peek() {
                Ok(value) => println!("Stack top: \"{}\"", value),
                Err(e) => println!("Error: {}", e),
            }
        }
        "SEMPTY" => {
            println!("Stack is {}", if ds.stack.is_empty() { "empty" } else { "not empty" });
        }
        "SPRINT" => {
            print!("Stack: ");
            for value in ds.stack.iter() {
                print!("{} ", value);
            }
            println!();
        }

        // Команды для очереди
        "QPUSH" => {
            if parts.len() > 1 {
                ds.queue.enqueue(parts[1].to_string());
                println!("Enqueued \"{}\"", parts[1]);
            }
        }
        "QPOP" => {
            match ds.queue.dequeue() {
                Ok(value) => println!("Dequeued: \"{}\"", value),
                Err(e) => println!("Error: {}", e),
            }
        }
        "QPEEK" => {
            match ds.queue.peek() {
                Ok(value) => println!("Queue front: \"{}\"", value),
                Err(e) => println!("Error: {}", e),
            }
        }
        "QEMPTY" => {
            println!("Queue is {}", if ds.queue.is_empty() { "empty" } else { "not empty" });
        }
        "QPRINT" => {
            print!("Queue: ");
            for value in ds.queue.iter() {
                print!("{} ", value);
            }
            println!();
        }

        // Команды для дерева
        "TINSERT" => {
            if parts.len() > 1 {
                ds.tree.insert(parts[1].to_string());
                println!("Inserted \"{}\" into tree", parts[1]);
            }
        }
        "TSEARCH" => {
            if parts.len() > 1 {
                let found = ds.tree.search(parts[1]);
                println!("Value \"{}\" {} in tree", parts[1], if found { "found" } else { "not found" });
            }
        }
        "TISFULL" => {
            println!("Tree is {}", if ds.tree.is_full() { "full" } else { "not full" });
        }
        "TPRINT" => {
            ds.tree.print_inorder();
        }

        _ => {
            println!("Unknown command: {}", command);
            println!("Available commands:");
            println!("Array: MPUSH_BACK, MPUSH_INDEX, MDEL, MGET, MREPLACE, MLEN, MPRINT");
            println!("Singly List: FPUSH_HEAD, FPUSH_TAIL, FDEL_HEAD, FDEL_TAIL, FDEL_VALUE, FSEARCH, FPRINT");
            println!("Doubly List: LPUSH_HEAD, LPUSH_TAIL, LDEL_HEAD, LDEL_TAIL, LSEARCH, LPRINT_FORWARD, LPRINT_BACKWARD");
            println!("Stack: SPUSH, SPOP, SPEEK, SEMPTY, SPRINT");
            println!("Queue: QPUSH, QPOP, QPEEK, QEMPTY, QPRINT");
            println!("Tree: TINSERT, TSEARCH, TISFULL, TPRINT");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        println!("Usage: {} --file <filename> --query <command>", args[0]);
        println!("Example: {} --file data --query 'MPUSH_BACK hello'", args[0]);
        return;
    }

    let mut base_filename = String::new();
    let mut query = String::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--file" => {
                if i + 1 < args.len() {
                    base_filename = args[i + 1].clone();
                    i += 1;
                }
            }
            "--query" => {
                if i + 1 < args.len() {
                    query = args[i + 1].clone();
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    if base_filename.is_empty() || query.is_empty() {
        println!("Error: Missing required arguments");
        return;
    }

    // Инициализация структур
    let mut ds = DataStructures::new();

    // Загрузка данных из файлов
    load_from_files(&base_filename, &mut ds);

    // Выполнение команды
    process_command(&query, &mut ds);

    // Сохранение данных в файлы
    save_to_files(&base_filename, &ds);
}