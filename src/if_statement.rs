pub fn if_statement() {
     // if
     let number = 8;
     if number < 5 {
         println!("5よりは低いよ");
     } else {
         println!("5ではないよ");
     }
 
     // 評価値がboolernだった場合
     let boolern_if = true;
     if boolern_if {
         println!("trueだよ");
     }
 
     // elseifも書ける(変数はnumber使いまわし) 
     if number % 4 == 0 {
         // 数値は4で割り切れます
         println!("number is divisible by 4");
     } else if number % 3 == 0 {
         // 数値は3で割り切れます
         println!("number is divisible by 3");
     } else if number % 2 == 0 {
         // 数値は2で割り切れます
         println!("number is divisible by 2");
     } else {
         // 数値は4、3、2で割り切れません
         println!("number is not divisible by 4, 3, or 2");
     }
 
     let condition: bool = false;
 
     let if_number = if condition {
         5
     } else {
         6
     };
 
     println!("{}",if_number);
}