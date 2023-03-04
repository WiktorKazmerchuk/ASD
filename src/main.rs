use rand::Rng; //Использование библиотеки rand::Rng (для генерации чисел)
use std::io; //Использование библиотеки std::io (для ввода данных)

fn main() {
    //Начало первой функции
    let mut q = String::new();
    let mut w = String::new();
    let mut e = String::new();
    let mut r = String::new();

    let mut t = String::new();
    let mut y = String::new(); //Создание будуйщих переменных
    let mut u = String::new();
    let mut i = String::new();

    let mut o = String::new();
    let mut p = String::new();
    let mut a = String::new();
    let mut s = String::new();

    loop {
        println!(" ");
        println!("Введите количество необходимых переменных для пароля\nОт 2 до 12"); //Вывод текста
        println!(" ");

        let mut kol = String::new(); //Создание перемменой для указания количества переменных
        match io::stdin().read_line(&mut kol) {
            //Начало блока match и ввод переменной
            Ok(_) => {} //Если введеная переменая является типом i8 то проходит
            Err(e) => println!("ОШИБКА ВВОДА - {}", e), //Если введеная переменая неявляется типом i8 то происходит ошибка
        }
        let kl: usize = kol.trim().parse().unwrap(); //Обработка переменной

        if kl <= 1 {
            //Проверка равенства и начало блока
            println!("Ограничение до 12, потому что сообщение 'Hello world!' весит 12 байт");
            //Типо пасхалки
        }

        println!("Введите 1 переменную: "); //Вывод текста
        match io::stdin().read_line(&mut q) {
            //Начало блока match и ввод переменной
            Ok(_) => {} //Если введеная переменая является типом i8 то проходит
            Err(e) => println!("ОШИБКА ВВОДА - {}", e), //Если введеная переменая неявляется типом i8 то происходит ошибка
        }
        println!("Введите 2 переменную: "); //Вывод текста
        match io::stdin().read_line(&mut w) {
            //Начало блока match и ввод переменной
            Ok(_) => {} //Если введеная переменая является типом i8 то проходит
            Err(e) => println!("ОШИБКА ВВОДА - {}", e), //Если введеная переменая неявляется типом i8 то происходит ошибка
        }

        if kl >= 3 {
            //Проверка равенства и начало блока
            println!("Введите 3 переменную: "); //Вывод текста
            match io::stdin().read_line(&mut e) {
                //Начало блока match и ввод переменной
                Ok(_) => {} //Если введеная переменая является типом i8 то проходит
                Err(e) => println!("ОШИБКА ВВОДА - {}", e), //Если введеная переменая неявляется типом i8 то происходит ошибка
            }
        }
        if kl >= 4 {
            //Проверка равенства и начало блока
            println!("Введите 4 переменную: "); //Вывод текста
            match io::stdin().read_line(&mut r) {
                //Начало блока match и ввод переменной
                Ok(_) => {} //Если введеная переменая является типом i8 то проходит
                Err(e) => println!("ОШИБКА ВВОДА - {}", e), //Если введеная переменая неявляется типом i8 то происходит ошибка
            }
        }

        if kl >= 5 {
            //Проверка равенства и начало блока
            println!("Введите 5 переменную: "); //Вывод текста
            match io::stdin().read_line(&mut t) {
                //Начало блока match и ввод переменной
                Ok(_) => {} //Если введеная переменая является типом i8 то проходит
                Err(e) => println!("ОШИБКА ВВОДА - {}", e), //Если введеная переменая неявляется типом i8 то происходит ошибка
            }
        }
        if kl >= 6 {
            //Проверка равенства и начало блока
            println!("Введите 6 переменную: "); //Вывод текста
            match io::stdin().read_line(&mut y) {
                //Начало блока match и ввод переменной
                Ok(_) => {} //Если введеная переменая является типом i8 то проходит
                Err(e) => println!("ОШИБКА ВВОДА - {}", e), //Если введеная переменая неявляется типом i8 то происходит ошибка
            }
        }

        if kl >= 7 {
            //Проверка равенства и начало блока
            println!("Введите 7 переменную: "); //Вывод текста
            match io::stdin().read_line(&mut u) {
                //Начало блока match и ввод переменной
                Ok(_) => {} //Если введеная переменая является типом i8 то проходит
                Err(e) => println!("ОШИБКА ВВОДА - {}", e), //Если введеная переменая неявляется типом i8 то происходит ошибка
            }
        }
        if kl >= 8 {
            //Проверка равенства и начало блока
            println!("Введите 8 переменную: "); //Вывод текста
            match io::stdin().read_line(&mut i) {
                //Начало блока match и ввод переменной
                Ok(_) => {} //Если введеная переменая является типом i8 то проходит
                Err(e) => println!("ОШИБКА ВВОДА - {}", e), //Если введеная переменая неявляется типом i8 то происходит ошибка
            }
        }

        if kl >= 9 {
            //Проверка равенства и начало блока
            println!("Введите 9 переменную: "); //Вывод текста
            match io::stdin().read_line(&mut o) {
                //Начало блока match и ввод переменной
                Ok(_) => {} //Если введеная переменая является типом i8 то проходит
                Err(e) => println!("ОШИБКА ВВОДА - {}", e), //Если введеная переменая неявляется типом i8 то происходит ошибка
            }
        }
        if kl >= 10 {
            //Проверка равенства и начало блока
            println!("Введите 10 переменную: "); //Вывод текста
            match io::stdin().read_line(&mut p) {
                //Начало блока match и ввод переменной
                Ok(_) => {} //Если введеная переменая является типом i8 то проходит
                Err(e) => println!("ОШИБКА ВВОДА - {}", e), //Если введеная переменая неявляется типом i8 то происходит ошибка
            }
        }

        if kl >= 11 {
            //Проверка равенства и начало блока
            println!("Введите 11 переменную: "); //Вывод текста
            match io::stdin().read_line(&mut a) {
                //Начало блока match и ввод переменной
                Ok(_) => {} //Если введеная переменая является типом i8 то проходит
                Err(e) => println!("ОШИБКА ВВОДА - {}", e), //Если введеная переменая неявляется типом i8 то происходит ошибка
            }
        }
        if kl >= 12 {
            //Проверка равенства и начало блока
            println!("Введите 12 переменную: "); //Вывод текста
            match io::stdin().read_line(&mut s) {
                //Начало блока match и ввод переменной
                Ok(_) => {} //Если введеная переменая является типом i8 то проходит
                Err(e) => println!("ОШИБКА ВВОДА - {}", e), //Если введеная переменая неявляется типом i8 то происходит ошибка
            }
        }

        let q1: String = q.trim().parse().unwrap(); //Начало
        let w1: String = w.trim().parse().unwrap();
        let e1: String = e.trim().parse().unwrap();
        let r1: String = r.trim().parse().unwrap();

        let t1: String = t.trim().parse().unwrap();
        let y1: String = y.trim().parse().unwrap(); //Обработка введеных переменных
        let u1: String = u.trim().parse().unwrap(); //Удалиение проделов, запятых и т.д.
        let i1: String = i.trim().parse().unwrap();

        let o1: String = o.trim().parse().unwrap();
        let p1: String = p.trim().parse().unwrap();
        let a1: String = a.trim().parse().unwrap();
        let s1: String = s.trim().parse().unwrap(); //Конец

        let kkl = kl - 1;

        let mut ps0: Vec<String> = vec![q1, w1]; //Создание вспомогательного вектора

        if kl >= 3 {
            ps0.push(e1);
        }
        if kl >= 4 {
            ps0.push(r1);
        }
        if kl >= 5 {
            ps0.push(t1);
        }
        if kl >= 6 {
            ps0.push(y1);
        }
        if kl >= 7 {
            ps0.push(u1);
        }
        if kl >= 8 {
            ps0.push(i1);
        }
        if kl >= 9 {
            ps0.push(o1);
        }
        if kl >= 10 {
            ps0.push(p1);
        }
        if kl >= 11 {
            ps0.push(a1);
        }
        if kl >= 12 {
            ps0.push(s1);
        }
        let mut ps1 = ps0.to_owned();
        let mut ps2 = ps0.to_owned();
        let mut ps3 = ps0.to_owned();

        let mut ps4 = ps0.to_owned();
        let mut ps5 = ps0.to_owned();
        let mut ps6 = ps0.to_owned();
        let mut ps7 = ps0.to_owned();

        let mut ps8 = ps0.to_owned();
        let mut ps9 = ps0.to_owned();
        let mut ps10 = ps0.to_owned();
        let mut ps11 = ps0.to_owned();

        let mut sp: Vec<String> = Vec::new(); //Создание основного вектора
        println!("{:?}", ps0); //Вывод вспомогательного вектора

        let mut p0i: usize = 13;
        let mut p1i: usize = 13;
        let mut p2i: usize = 13;
        let mut p3i: usize = 13;

        let mut p4i: usize = 13;
        let mut p5i: usize = 13;
        let mut p6i: usize = 13;
        let mut p7i: usize = 13;

        let mut p8i: usize = 13;
        let mut p9i: usize = 13;
        let mut p10i: usize = 13;
        let mut p11i: usize = 13;

        p0i = rand::thread_rng().gen_range(0..=kkl); //Создание рандомного число
        ps0[0] = ps0[p0i].to_owned(); //Перемещение индекса элемента 0
        let v0 = ps0[0].to_owned(); //Создание переменной
        sp.push(v0); //Добавление к основному вектору созданую переменную

        p1i = rand::thread_rng().gen_range(0..=kkl); //Создание рандомного числа
        if p1i == p0i {
            loop {
                p1i = rand::thread_rng().gen_range(0..=kkl); //Создание рандомного числа
                if p1i != p0i {
                    break;
                }
            }
        }
        ps1[1] = ps1[p1i].to_owned(); //Перемещение индекса элемента 1
        let v1 = ps1[1].to_owned(); //Создание переменной
        sp.push(v1); //Добавление к основному вектору созданую переменную

        if kl >= 3 {
            p2i = rand::thread_rng().gen_range(0..=kkl); //Создание рандомного числа
            if p2i == p1i || p2i == p0i {
                loop {
                    p2i = rand::thread_rng().gen_range(0..=kkl); //Создание рандомного числа
                    if p2i != p1i && p2i != p0i {
                        break;
                    }
                }
            }
            ps2[2] = ps2[p2i].to_owned(); //Перемещение индекса элемента 2
            let v2 = ps2[2].to_owned(); //Создание переменной
            sp.push(v2); //Добавление к основному вектору созданую переменную
        }

        if kl >= 4 {
            p3i = rand::thread_rng().gen_range(0..=kkl); //Создание рандомного числа
            if p3i == p2i || p3i == p1i || p3i == p0i {
                loop {
                    p3i = rand::thread_rng().gen_range(0..=kkl); //Создание рандомного числа
                    if p3i != p2i && p3i != p1i && p3i != p0i {
                        break;
                    }
                }
            }
            ps3[3] = ps3[p3i].to_owned(); //Перемещение индекса элемента3
            let v3 = ps3[3].to_owned(); //Создание переменной
            sp.push(v3); //Добавление к основному вектору созданую переменную
        }


        if kl >= 5 {
            p4i = rand::thread_rng().gen_range(0..=kkl); //Создание рандомного числа
            if p4i == p3i || p4i == p2i || p4i == p1i || p4i == p0i {
                loop {
                    p4i = rand::thread_rng().gen_range(0..=kkl); //Создание рандомного числа
                    if p4i != p3i && p4i != p2i && p4i != p1i && p4i != p0i {
                        break;
                    }
                }
            }
            ps4[4] = ps4[p4i].to_owned(); //Перемещение индекса элемента 4
            let v4 = ps4[4].to_owned(); //Создание переменной
            sp.push(v4); //Добавление к основному вектору созданую переменную
        }

        if kl >= 6 {
            p5i = rand::thread_rng().gen_range(0..=kkl); //Создание рандомного числа
            if p5i == p4i || p5i == p3i || p5i == p2i || p5i == p1i || p5i == p0i {
                loop {
                    p5i = rand::thread_rng().gen_range(0..=kkl); //Создание рандомного числа
                    if p5i != p4i && p5i != p3i && p5i != p2i && p5i != p1i && p5i != p0i {
                        break;
                    }
                }
            }
            ps5[5] = ps5[p5i].to_owned(); //Перемещение индекса элемента 5
            let v5 = ps5[5].to_owned(); //Создание переменной
            sp.push(v5); //Добавление к основному вектору созданую переменную
        }

        if kl >= 7 {
            p6i = rand::thread_rng().gen_range(0..=kkl); //Создание рандомного числа
            if p6i == p5i || p6i == p4i || p6i == p3i || p6i == p2i || p6i == p1i || p6i == p0i {
                loop {
                    p6i = rand::thread_rng().gen_range(0..=kkl); //Создание рандомного числа
                    if p6i != p5i && p6i != p4i && p6i != p3i && p6i != p2i && p6i != p1i && p6i != p0i
                    {
                        break;
                    }
                }
            }
            ps6[6] = ps6[p6i].to_owned(); //Перемещение индекса элемента 6
            let v6 = ps6[6].to_owned(); //Создание переменной
            sp.push(v6); //Добавление к основному вектору созданую переменную
        }


        if kl >= 8 {
            p7i = rand::thread_rng().gen_range(0..=kkl); //Создание рандомного числа
            if p7i == p6i
                || p7i == p5i
                || p7i == p4i
                || p7i == p3i
                || p7i == p2i
                || p7i == p1i
                || p7i == p0i
            {
                loop {
                    p7i = rand::thread_rng().gen_range(0..=kkl); //Создание рандомного числа
                    if p7i != p6i
                        && p7i != p5i
                        && p7i != p4i
                        && p7i != p3i
                        && p7i != p2i
                        && p7i != p1i
                        && p7i != p0i
                    {
                        break;
                    }
                }
            }
            ps7[7] = ps7[p7i].to_owned(); //Перемещение индекса элемента 7
            let v7 = ps7[7].to_owned(); //Создание переменной
            sp.push(v7); //Добавление к основному вектору созданую переменную
        }

        if kl >= 9 {
            p8i = rand::thread_rng().gen_range(0..=kkl); //Создание рандомного числа
            if p8i == p7i
                || p8i == p6i
                || p8i == p5i
                || p8i == p4i
                || p8i == p3i
                || p8i == p2i
                || p8i == p1i
                || p8i == p0i
            {
                loop {
                    p8i = rand::thread_rng().gen_range(0..=kkl); //Создание рандомного числа
                    if p8i != p7i
                        && p8i != p6i
                        && p8i != p5i
                        && p8i != p4i
                        && p8i != p3i
                        && p8i != p2i
                        && p8i != p1i
                        && p8i != p0i
                    {
                        break;
                    }
                }
            }
            ps8[8] = ps8[p8i].to_owned(); //Перемещение индекса элемента 8
            let v8 = ps8[8].to_owned(); //Создание переменной
            sp.push(v8); //Добавление к основному вектору созданую переменную
        }

        if kl >= 10 {
            p9i = rand::thread_rng().gen_range(0..=kkl); //Создание рандомного числа
            if p9i == p8i
                || p9i == p7i
                || p9i == p6i
                || p9i == p5i
                || p9i == p4i
                || p9i == p3i
                || p9i == p2i
                || p9i == p1i
                || p9i == p0i
            {
                loop {
                    p9i = rand::thread_rng().gen_range(0..=kkl); //Создание рандомного числа
                    if p9i != p8i
                        && p9i != p7i
                        && p9i != p6i
                        && p9i != p5i
                        && p9i != p4i
                        && p9i != p3i
                        && p9i != p2i
                        && p9i != p1i
                        && p9i != p0i
                    {
                        break;
                    }
                }
            }
            ps9[9] = ps9[p9i].to_owned(); //Перемещение индекса элемента 9
            let v9 = ps9[9].to_owned(); //Создание переменной
            sp.push(v9); //Добавление к основному вектору созданую переменную
        }

        if kl >= 11 {
            p10i = rand::thread_rng().gen_range(0..=kkl); //Создание рандомного числа
            if p10i == p9i
                || p10i == p8i
                || p10i == p7i
                || p10i == p6i
                || p10i == p5i
                || p10i == p4i
                || p10i == p3i
                || p10i == p2i
                || p10i == p1i
                || p10i == p0i
            {
                loop {
                    p10i = rand::thread_rng().gen_range(0..=kkl); //Создание рандомного числа
                    if p10i != p9i
                        && p10i != p8i
                        && p10i != p7i
                        && p10i != p6i
                        && p10i != p5i
                        && p10i != p4i
                        && p10i != p3i
                        && p10i != p2i
                        && p10i != p1i
                        && p10i != p0i
                    {
                        break;
                    }
                }
            }
            ps10[10] = ps10[p10i].to_owned(); //Перемещение индекса элемента 10
            let v10 = ps10[10].to_owned(); //Создание переменной
            sp.push(v10); //Добавление к основному вектору созданую переменную
        }

        if kl >= 12 {
            p11i = rand::thread_rng().gen_range(0..=kkl); //Создание рандомного числа
            if p11i == p10i
                || p11i == p9i
                || p11i == p8i
                || p11i == p7i
                || p11i == p6i
                || p11i == p5i
                || p11i == p4i
                || p11i == p3i
                || p11i == p2i
                || p11i == p1i
                || p11i == p0i
            {
                loop {
                    p11i = rand::thread_rng().gen_range(0..=kkl); //Создание рандомного числа
                    if p11i != p10i
                        && p11i != p9i
                        && p11i != p8i
                        && p11i != p7i
                        && p11i != p6i
                        && p11i != p5i
                        && p11i != p4i
                        && p11i != p3i
                        && p11i != p2i
                        && p11i != p1i
                        && p11i != p0i
                    {
                        break;
                    }
                }
            }
            ps11[11] = ps11[p11i].to_owned(); //Перемещение индекса элемента 10
            let v11 = ps11[11].to_owned(); //Создание переменной
            sp.push(v11); //Добавление к основному вектору созданую переменную
        }

        print!(" {}", p0i);
        print!(" {}", p1i);
        print!(" {}", p2i);
        print!(" {}", p3i);

        print!(" {}", p4i);
        print!(" {}", p5i);
        print!(" {}", p6i);
        print!(" {}", p7i);

        print!(" {}", p8i);
        print!(" {}", p9i);
        print!(" {}", p10i);
        println!(" {}", p11i);

        for sp in &sp{
            print!("{}", sp);
        }

        q = String::from("");
        w = String::from("");
        e = String::from("");
        r = String::from("");

        t = String::from("");
        y = String::from("");
        u = String::from("");
        i = String::from("");

        o = String::from("");
        p = String::from("");
        a = String::from("");
        s = String::from("");
    }
}
/*
let m0 = rand::thread_rng().gen_range(0..=11);
cargo build --release  .to_owned()
let mo = 9;
ps(0) = ps(1);
 */