fn main() {
    // これで1行のコメント
    // 変更不能な変数 n を宣言
    let n = 0;
    /*
    このようにすると複数行にわたるコメントアウトが可能。
    変更可能な変数 m を宣言する
     */
    let mut m = 0;
    println!("n:{}, m:{}",n,m);
    /*
    ここで、m に 1 を代入
     */
    m = 1;
    println!("m に1を代入");
    println!("n:{}, m:{}",n,m);

    /* 
    n をもう一度宣言することで、n の値を変更することもできる
    また、まったく別の n となるため、別の型の値を入れても大丈夫
    */
    let n = "abc";
    println!("n:{}, m:{}",n,m);

    // ----------------------------------------------------

    // i32 型(符号付32bit整数)の変数 i を宣言
    let i:i32 = 5;
    println!("i:{}, type of i:{}",i,std::any::type_name_of_val(&i));
    // i32型の変数 i に"abc"を入れてみる
    // i = "abc";
    // コンパイルが通らないのでコメントアウト
    
    // ----------------------------------------------------
    // 型変換を行ってみる
    // ここでは、i32 型の i を i64 型の j に代入
    let j: i64 = i as i64;
    println!("j:{}, type of j:{}",i,std::any::type_name_of_val(&j));

    // ----------------------------------------------------
    // 構造体を作る
    struct Point {
        x: i32,
        y:i32,
    }

    let p = Point {x: 100, y:200};
    println!("p.x:{}, p.y:{}",p.x,p.y);

    // ---------------------------------------------------
    // 共用体を作る
    union MyUnion {
        f1: u32,
        f2: u32,
    }

    // 構造体では x , y でメモリを別々に確保するが、unionは共用する。
    let u = MyUnion{f1:123};

    // 共用しているため、f1 に代入した値はf2にも代入される
    // また、union は安全でないため、非安全コードを実行するためのキーワード unsafe でくくる
    unsafe{
        println!("f1:{}",u.f1);
        println!("f2:{}",u.f2);
    }

    // -------------------------------------------------
    // 列挙体 enum を作る
    enum Color {
        Red,
        Green,
        Blue,
    }
    let color = Color::Red;
    println!("type of color:{}",std::any::type_name_of_val(&color));

    // -----------------------------------------------
    // タプル (tup) を作る

    let tup = (10, "20", 30);
    // tupはこのように異なる型を混ぜることができる
    println!("{} , {} , {}",tup.0,tup.1,tup.2);
    println!("{} , {} , {}",std::any::type_name_of_val(&tup.0),std::any::type_name_of_val(&tup.1),std::any::type_name_of_val(&tup.2))
}
