pub fn varisble() {
    // 変数宣言
    // letは書き換え不可
    let x = 5;
    println!("{}",x);
    
    // mutをつけることで可変になる
    // Rustは一回も使われずに値を変更するとコンパイルエラーになる
    let mut y = 5;
    println!("{}",y);
    y = 6;
    println!("{}",y);

    // シャドーイング
    // Rustではmutを付けていない変数の値を覆い隠すことが出来る
    // 同じ変数名を使いまわせる
    // 設定を切らないとこの機能は使えないっぽい
    // let num = 0;
    // num + 2;
    // num + 3;
    // println!("{}",num);
}