pub fn type_system() {
        // 型システム
        let boolern: bool = true;
        println!("{}",boolern);
    
        // タプルで表すこともできる
        // 値を取り出す場合はかく要素の数の変数を用意すると、かってに当てはまる
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let (x, y, z) = tup;
    
        println!("{} {} {}", x , y , z);
    
        // インデックスみたいに添え字で取り出すことも出来る
        println!("{} {} {}", tup.0,tup.1,tup.2);
}