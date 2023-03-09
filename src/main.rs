use rand::Rng; //Использование библиотеки rand::Rng (для генерации чисел)
use std::io; //Использование библиотеки std::io (для ввода данных)

fn main() {
    loop {
        let mut sp: Vec<String> = Vec::new(); //Создание основной переменной
        let mut wv_sp: Vec<String> = Vec::new(); //Вспомогательный вектор sp

        println!(" ");
        println!("Выберете что вы хотите из перечисленых в таблице вариантов."); //Вывод текста
        println!("И введите номер выбраного варианта."); //Вывод текста
        println!("Перемешивание переменных......................................1"); //Вывод текста
        println!("Генерация пароля..............................................2"); //Вывод текста
        println!("Открытие базы с сохранеными паролями..........................3"); //Вывод текста
        println!(" ");

        let mut wariant = String::new(); //Создание перемменой для открытия блока перемешивания
        match io::stdin().read_line(&mut wariant) {
            //Начало блока match и ввод переменной
            Ok(_) => {} //Если введеная переменая является типом i8 то проходит
            Err(e) => println!("ОШИБКА ВВОДА - {}", e), //Если введеная переменая неявляется типом i8 то происходит ошибка
        }
        let wari: usize = wariant.trim().parse().unwrap(); //Обработка переменной

        if wari == 1 {
            println!(" ");
            println!("Введите количество необходимых переменных для перемешивания."); //Вывод текста
            println!("Без ограничений. Главное правило не вводите переменную: 'uzed'"); //Вывод текста
            println!(" ");

            let mut kol = String::new(); //Создание перемменой для указания количества переменных
            match io::stdin().read_line(&mut kol) {
                //Начало блока match и ввод переменной
                Ok(_) => {} //Если введеная переменая является типом i8 то проходит
                Err(e) => println!("ОШИБКА ВВОДА - {}", e), //Если введеная переменая неявляется типом i8 то происходит ошибка
            }

            let kl: usize = kol.trim().parse().unwrap(); //Обработка переменной
            let mut vp0: Vec<String> = Vec::new(); //Создание вектора для введеных переменых

            for l in 1..=kl {
                let mut q = String::new();

                println!(" "); //Вывод текста
                println!("Введите {} переменную: ", l); //Вывод текста
                println!(" "); //Вывод текста

                match io::stdin().read_line(&mut q) {
                    //Начало блока match и ввод переменной
                    Ok(_) => {} //Если введеная переменая является типом i8 то проходит
                    Err(e) => println!("ERROR - {}", e), //Если введеная переменая неявляется типом i8 то происходит ошибка
                }

                let q1: String = q.trim().parse().unwrap(); //Обработка введеных переменных. Удалиение проделов, запятых и т.д.
                vp0.push(q1);
            }

            let kkl = kl - 1;

            println!(" ");
            print!("Вы ввели                       : ");
            for vp0 in &vp0 {
                print!("{}", vp0);
            }
            println!(" ");

            let mut pi1: usize = 0;
            let mut i: usize = 0;

            loop {
                let uzed = String::from("uzed");
                let vp1 = vp0.to_owned();
                loop {
                    pi1 = rand::thread_rng().gen_range(0..=kkl);

                    if vp0[pi1] != uzed.to_owned() {
                        i += 1;
                        break;
                    }
                }
                sp.push(vp1[pi1].to_owned());
                vp0[pi1] = uzed.to_owned();

                if sp != vp0 && i == kl {
                    break;
                }
            }

            print!("Ваш пароль                     : ");
            for sp in &sp {
                print!("{}", sp);
            }
            println!(" ");

            /*for p in 1..kl {
                let _removed_element = sp.remove(p);
            } */
        }

        if wari == 2 {
            let mut vnum: Vec<String> = Vec::new();
            let mut vabb: Vec<String> = Vec::new();
            let mut vamb: Vec<String> = Vec::new();
            let mut vsz: Vec<String> = Vec::new();

            println!(" ");
            println!("Введите количество необходимых цифр (без ограничений)."); //Вывод текста
            println!(" ");

            let mut numbers = String::new(); //Создание перемменой для открытия блока перемешивания
            match io::stdin().read_line(&mut numbers) {
                //Начало блока match и ввод переменной
                Ok(_) => {} //Если введеная переменая является типом i8 то проходит
                Err(e) => println!("ОШИБКА ВВОДА - {}", e), //Если введеная переменая неявляется типом i8 то происходит ошибка
            }

            let numb: i64 = numbers.trim().parse().unwrap(); //Обработка переменной

            for _i in 1..=numb {
                let pi = rand::thread_rng().gen_range(0..=9); //Создание рандомного число
                vnum.push(pi.to_string()); //Добавление к основному вектору созданую переменную
            }

            println!(" ");
            println!("Введите количество больших английских букв (без ограничений)."); //Вывод текста
            println!(" ");

            let mut sslow = String::new(); //Создание перемменой для открытия блока перемешивания
            match io::stdin().read_line(&mut sslow) {
                //Начало блока match и ввод переменной
                Ok(_) => {} //Если введеная переменая является типом i8 то проходит
                Err(e) => println!("ОШИБКА ВВОДА - {}", e), //Если введеная переменая неявляется типом i8 то происходит ошибка
            }
            let slov: usize = sslow.trim().parse().unwrap(); //Обработка переменной

            let mut gns0: usize;

            for _i in 1..=slov {
                let abb = vec![
                    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P",
                    "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
                ];
                gns0 = rand::thread_rng().gen_range(0..=25); //Создание рандомного число
                let x1 = abb[gns0].to_owned(); //Создание переменной
                vabb.push(x1); //Добавление к основному вектору созданую переменную
            }

            println!(" ");
            println!("Введите количество маленьких английских букв (без ограничений)."); //Вывод текста
            println!(" ");

            let mut ssslow = String::new(); //Создание перемменой для открытия блока перемешивания
            match io::stdin().read_line(&mut ssslow) {
                //Начало блока match и ввод переменной
                Ok(_) => {} //Если введеная переменая является типом i8 то проходит
                Err(e) => println!("ОШИБКА ВВОДА - {}", e), //Если введеная переменая неявляется типом i8 то происходит ошибка
            }
            let slow: usize = ssslow.trim().parse().unwrap(); //Обработка переменной

            let mut gns1: usize;

            for _i in 1..=slow {
                let amb = vec![
                    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p",
                    "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
                ];
                gns1 = rand::thread_rng().gen_range(0..=25); //Создание рандомного число
                let e1 = amb[gns1].to_owned(); //Создание переменной
                vamb.push(e1); //Добавление к основному вектору созданую переменную
            }

            println!(" ");
            println!("Введите количество спец-знаков (без ограничений)."); //Вывод текста
            println!(" ");

            let mut sssslow = String::new(); //Создание перемменой для открытия блока перемешивания
            match io::stdin().read_line(&mut sssslow) {
                //Начало блока match и ввод переменной
                Ok(_) => {} //Если введеная переменая является типом i8 то проходит
                Err(e) => println!("ОШИБКА ВВОДА - {}", e), //Если введеная переменая неявляется типом i8 то происходит ошибка
            }
            let slob: usize = sssslow.trim().parse().unwrap(); //Обработка переменной

            let mut gns2: usize;

            for _i in 1..=slob {
                let sz = vec![
                    "!", "@", "#", "$", "%", "^", "&", "*", "№", "?", ".", ",", "<", ">", "(", ")",
                    "{", "}", "[", "]", "/",
                ];
                gns2 = rand::thread_rng().gen_range(0..=20); //Создание рандомного число
                let k1 = sz[gns2].to_owned(); //Создание переменной
                vsz.push(k1); //Добавление к основному вектору созданую переменную
            }

            println!(" ");
            print!("Сгенерированые числа           : ");
            for vnum in &vnum {
                print!("{}", vnum);
            }
            println!(" ");

            print!("Сгенерированые больши буквы    : ");
            for vabb in &vabb {
                print!("{}", vabb);
            }
            println!(" ");

            print!("Сгенерированые маленькие буквы : ");
            for vamb in &vamb {
                print!("{}", vamb);
            }
            println!(" ");

            print!("Сгенерированые спец-знаки      : ");
            for vsz in &vsz {
                print!("{}", vsz);
            }
            println!(" ");

            for vnum in &vnum {
                wv_sp.push(vnum.to_string());
            }
            for vabb in &vabb {
                wv_sp.push(vabb.to_string());
            }
            for vamb in &vamb {
                wv_sp.push(vamb.to_string());
            }
            for vsz in &vsz {
                wv_sp.push(vsz.to_string());
            }

            let mut pi1: usize;
            let mut i: usize = 0;
            let len_wv_sp = wv_sp.len();
            let lw_wv_sp: usize = len_wv_sp - 1;

            loop {
                let uzed = String::from("uzed");
                let wv_sp1: Vec<String> = wv_sp.to_owned();
                loop {
                    pi1 = rand::thread_rng().gen_range(0..=lw_wv_sp);

                    if wv_sp1[pi1] != uzed.to_owned() {
                        i += 1;
                        break;
                    }
                }
                sp.push(wv_sp1[pi1].to_owned());
                wv_sp[pi1] = uzed.to_owned();

                if sp != wv_sp && i == len_wv_sp {
                    break;
                }
            }

            println!(" ");
            print!("Ваш пароль                     : ");
            for sp in &sp {
                print!("{}", sp);
            }
            println!(" ");
        }

        if wari == 3 {
            println!(" ");
            println!("К сожалению данный отсек программы находится в разработке");
        }
    }
}
