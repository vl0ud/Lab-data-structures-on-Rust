# Lab-data-structures-on-Rust

Lab work for the 2nd semester of programming at NSTU (Rust version).

Это Rust-версия проекта [Lab-data-structures](https://github.com/vl0ud/Lab-data-structures), сохраняющая архитектуру оригинального C++ проекта.

## Структуры данных

Проект реализует следующие структуры данных:
- **Динамический массив** (array)
- **Односвязный список** (singly_list)
- **Двусвязный список** (doubly_list)
- **Стек** (stack)
- **Очередь** (queue)
- **Полное бинарное дерево** (full_binary_tree)

## Сборка и запуск

```bash
# Сборка проекта
cargo build --release

# Запуск
cargo run -- --file data --query "MPUSH_BACK hello"
```

## Примеры команд

### Массив
- `MPUSH_BACK value` - добавить в конец
- `MPUSH_INDEX index value` - добавить по индексу
- `MDEL index` - удалить по индексу
- `MGET index` - получить элемент
- `MREPLACE index value` - заменить элемент
- `MLEN` - длина массива
- `MPRINT` - вывести массив

### Односвязный список
- `FPUSH_HEAD value` - добавить в начало
- `FPUSH_TAIL value` - добавить в конец
- `FDEL_HEAD` - удалить из начала
- `FDEL_TAIL` - удалить из конца
- `FDEL_VALUE value` - удалить по значению
- `FSEARCH value` - найти элемент
- `FPRINT` - вывести список

### Двусвязный список
- `LPUSH_HEAD value` - добавить в начало
- `LPUSH_TAIL value` - добавить в конец
- `LDEL_HEAD` - удалить из начала
- `LDEL_TAIL` - удалить из конца
- `LSEARCH value` - найти элемент
- `LPRINT_FORWARD` - вывести вперед
- `LPRINT_BACKWARD` - вывести назад

### Стек
- `SPUSH value` - добавить в стек
- `SPOP` - извлечь из стека
- `SPEEK` - посмотреть вершину
- `SEMPTY` - проверить пустоту
- `SPRINT` - вывести стек

### Очередь
- `QPUSH value` - добавить в очередь
- `QPOP` - извлечь из очереди
- `QPEEK` - посмотреть первый элемент
- `QEMPTY` - проверить пустоту
- `QPRINT` - вывести очередь

### Дерево
- `TINSERT value` - вставить в дерево
- `TSEARCH value` - найти в дереве
- `TISFULL` - проверить полноту
- `TPRINT` - вывести дерево

## Лицензия

MIT License
```