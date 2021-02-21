pub fn while_for_statement() {
    // whileループ
    // 条件付きでループができる
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");

    // 配列の中身をすべて吐き出すwhile
    // 動作が遅いし変数indexが配列以上の数だった場合パニックを起こすためあまり推奨されない書き方
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }

    // for文
    // 変数は上の使いまわし
    // こっちは要素の数だけ回すようにするので安全だし、動作が早い
    for element in a.iter() {
        // 値は{}です
        println!("the value is: {}", element);
    }
}