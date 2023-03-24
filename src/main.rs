extern crate rand;

use rand::Rng;
//use core::num::dec2flt::number;
use std::io::{stdout, stdin, Write, BufRead};
use std::fs::File;
use std::process::{Command};

fn main() {
    let all_n: i32 = 5;
    let number_of_problem : usize = input_text("問題数".to_string()).parse().unwrap();
    let variable1 : String = input_text("文字1".to_string());
    let variable2 : String = input_text("文字2".to_string());
    progress(all_n, 0);

    // 問題・解答を生成してvecに入れる
    let mut v: Vec<(String, String)> = Vec::new();
    for _i in 0 .. number_of_problem {
        v.push(equ(&variable1, &variable2));
    }
    progress(all_n, 1);

    // 問題・解答を文字列化
    let mut ques: String = String::new();
    let mut ans: String = String::new();
    for i in 0 .. number_of_problem {
        ques += &format!("\t\t\t{}\n", v[i].0);
        ans += &format!("\t\t\t{}\n", v[i].1);
    }
    progress(all_n, 2);

    // texファイルに書き込み
    write_tex("./main.tex".to_string(), ques, ans, number_of_problem);
    progress(all_n, 3);

    //コンパイル
    shell(all_n);
    progress(all_n, 5);
}

// 問題・解答を生成
fn equ(variable1: &str, variable2: &str) -> (String, String) {
    let mut res: (String, String) = (String::new(), String::new());
    // 因数分解時の係数
    let mut a = get_rand(1, 10);
    let mut b = get_rand(-9, 10);
    let mut c = get_rand(1, 10);
    let mut d = get_rand(-9, 10);

    // 展開後の係数
    let p: i32 = a * c;
    let q: i32 = a*d + b*c;
    let r: i32 = b * d;

    // 共通因数でくくる
    let mut k = 1;
    print!("{} {} {} ", a, b, k);
    (a, b, k)= yakubun(a, b, k);
    print!("{} {} {} ", a, b, k);
    print!("{} {} {} ", c, d, k);
    (c, d, k)= yakubun(c, d, k);
    println!("{} {} {} ", c, d, k);
    let k_text: String;
    if k == 1 {
        k_text = String::from("");
    } else {
        k_text = k.to_string();
    }
    

    let x2: String = kou(p, 2, variable1);
    let x1: String = kou(q, 1, &format!("{}{}", variable1, variable2));
    let x0: String = kou(r, -2, variable2); // -2を指定すると2を指定したときと違い頭に+がつく

    res.0 = format!("\\item $\\displaystyle {}{}{}$", x2, x1, x0);
    res.1 = format!("\\item $\\displaystyle {6}({0}{4}{1}{5})({2}{4}{3}{5})$", a, kou(b, 0, variable1), c, kou(d, 0, variable2), variable1, variable2, k_text);
    return res;
}

fn yakubun(a: i32, b: i32, k: i32) -> (i32, i32, i32) {
    let r: i32 = gcm(a, b);
    (a/r, b/r, k*r)
}
fn gcm(a: i32, b: i32) -> i32 {
    if a % b == 0 {
        if b < 0 {
            return -b;
        }
        return b;
    }
    gcm(b, a%b)
}

// a以上b未満の乱数を取得
fn get_rand(a: i32, b :i32) -> i32 {
    let mut res: i32 = rand::thread_rng().gen_range(a, b-1);
    if res >= 0 { // 0は出ないようにする
        res += 1;
    }
    return res;
}
// 展開後の式の項を作成 e:x^eの項についての処理をさせる
fn kou(pqr: i32, e: i32, variable: &str) -> String {
    if pqr == 0 {
        return "".to_string();
    }

    let mut res: String = String::new();
    // x^2のみ+は省略
    if e != 2 && pqr > 0 {
        res += "+";
    }
    res += &pqr.to_string();

    // x^nを追記
    match e {
        2 => res += &format!("{}^2", variable),
        1 => res += &format!("{}", variable),
        -2 => res += &format!("{}^2", variable),
        _ => {},
    }

    return res;
}

// texファイルに書き込み
fn write_tex(path: String, ques: String, ans: String, number_of_problem: usize) {
       let mut file = File::create(path)
           .expect("file not found.");
        writeln!(file, "\\documentclass[11pt,a4paper,dvipdfmx]{{jsarticle}}").expect("cannot write.");
        writeln!(file, "\\usepackage{{amsmath,amssymb, minijs, pxfonts, multicol, enumerate}}").expect("cannot write.");
        writeln!(file, "\\usepackage[top=25.4truemm,bottom=25.4truemm,left=19.05truemm,right=19.05truemm]{{geometry}}").expect("cannot write.");
        writeln!(file, "\\begin{{document}}").expect("cannot write.");

        writeln!(file, "\t\\subsection*{{問題}}").expect("cannot write.");
        writeln!(file, "\t\\begin{{multicols*}}{{3}}").expect("cannot write.");
        writeln!(file, "\t\t\\begin{{enumerate}}[(1)]").expect("cannot write.");
        write!(file, "{}", ques).expect("cannot write.");
        writeln!(file, "\t\t\\end{{enumerate}}").expect("cannot write.");
        writeln!(file, "\t\\end{{multicols*}}").expect("cannot write.");

        writeln!(file, "\t\\newpage").expect("cannot write.");

        writeln!(file, "\t\\subsection*{{解答}}").expect("cannot write.");
        writeln!(file, "\t\\begin{{multicols*}}{{3}}").expect("cannot write.");
        writeln!(file, "\t\t\\begin{{enumerate}}[(1)]").expect("cannot write.");
        write!(file, "{}", ans).expect("cannot write.");
        writeln!(file, "\t\t\\end{{enumerate}}").expect("cannot write.");
        writeln!(file, "\t\\end{{multicols*}}").expect("cannot write.");

        writeln!(file, "\t\\subsection*{{正答率}}").expect("cannot write.");
        writeln!(file, "\t\\Huge\\hspace{{1cm}} /{}", number_of_problem).expect("cannot write.");
        writeln!(file, "\\end{{document}}").expect("cannot write.");
}

fn shell(all_n: i32) {
    let mut _output;
    _output = Command::new("platex")
        .arg("main.tex")
        .output()
        .expect("failed");

    progress(all_n, 4);
    _output = Command::new("dvipdfmx")
        .arg("main.dvi")
        .output()
        .expect("failed");
}

// k/n
fn progress(n: i32, k: i32) {
    print!("\x1b[2K");
    print!("\r");
    stdout().flush().unwrap();

    for _i in 0 .. k {
        print!("■");
    }
    for _i in k+1 ..= n {
        print!("□");
    }
    print!(" ");
    print!("{:.1}%", k as f64 / n as f64 * 100.0);
    stdout().flush().unwrap();
}


// s:って出して入力文字列を返す
fn input_text(s: String) -> String {
    print!("{}: ", s);
    stdout().flush().unwrap();
    let stdin = stdin();
    let mut buffer = String::new();
    stdin.lock().read_line(&mut buffer).unwrap();  // 標準入力から行を読み取る
    return buffer.trim().to_string();
}