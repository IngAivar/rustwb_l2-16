fn as_chan(vs: &[i32]) -> std::sync::mpsc::Receiver<i32> {
    // Создаем канал для передачи целых чисел.
    let (tx, rx) = std::sync::mpsc::channel();

    // Создаем новый поток, который будет отправлять значения из `vs` в канал.
    let handle = std::thread::spawn({
        let vs = vs.to_owned(); // Клонируем `vs`, чтобы он был доступен в замыкании.
        move || {
            for v in vs {
                // Отправляем значение в канал.
                tx.send(v).unwrap();

                // Ждем 1 секунду перед отправкой следующего значения.
                std::thread::sleep(std::time::Duration::from_secs(1));
            }

            // Закрываем передатчик, чтобы сигнализировать о завершении.
            drop(tx);
        }
    });

    // Ожидаем завершения потока.
    handle.join().unwrap();

    // Возвращаем приемник канала.
    rx
}

fn merge(a: std::sync::mpsc::Receiver<i32>, b: std::sync::mpsc::Receiver<i32>) -> std::sync::mpsc::Receiver<i32> {
    // Создаем новый канал для слияния значений из `a` и `b`.
    let (tx, rx) = std::sync::mpsc::channel();

    // Флаги для отслеживания завершения каналов `a` и `b`.
    let mut a_done = false;
    let mut b_done = false;

    // Цикл, который читает значения из `a` и `b` и отправляет их в `tx`.
    loop {
        match a.try_recv() {
            Ok(i) => {
                // Если удалось прочитать значение из `a`, отправляем его в `tx`.
                tx.send(i).unwrap();
            }
            Err(_) => {
                // Если чтение из `a` не удалось, значит канал `a` пуст.
                a_done = true;
            }
        }

        match b.try_recv() {
            Ok(i) => {
                // Если удалось прочитать значение из `b`, отправляем его в `tx`.
                tx.send(i).unwrap();
            }
            Err(_) => {
                // Если чтение из `b` не удалось, значит канал `b` пуст.
                b_done = true;
            }
        }

        // Если оба канала пустые, то цикл завершается.
        if a_done && b_done {
            break;
        }
    }

    // Возвращаем приемник канала слияния.
    rx
}

fn main() {
    // Создаем два канала с значениями 1, 3, 5, 7 и 2, 4, 6, 8 соответственно.
    let a = as_chan(&vec![1, 3, 5, 7]);
    let b = as_chan(&vec![2, 4, 6, 8]);

    // Сливаем значения из каналов `a` и `b в новый канал `c`.
    let c = merge(a, b);

    // Печатаем значения из канала `c`.
    for v in c.iter() {
        println!("{v:?}");
    }
}