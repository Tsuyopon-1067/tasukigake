extern crate rand;

use proconio::input;
use rand::Rng;
use std::io::{stdout, Write};
use std::fs::File;
use std::process::{Command};

fn main() {
    let all_n: i32 = 4;
    print!("num: ");
    stdout().flush().unwrap();
    input! {
        n: usize
    }
    progress(all_n, 0);

    // 問題・解答を生成してvecに入れる
    let mut v: Vec<(String, String)> = Vec::new();
    for _i in 0..n {
        v.push(equ());
    }
    progress(all_n, 1);

    // 問題・解答を文字列化
    let mut ques: String = String::new();
    let mut ans: String = String::new();
    for i in 0..n {
        ques += &format!("{}{}{}", "\t\t\t\\item $", v[i].0, "$ \\\\\n");
        ans += &format!("{}{}{}", "\t\t\t\\item $", v[i].1, "$ \\\\\n");
    }
    progress(all_n, 2);

    // texファイルに書き込み
    write_tex("./main.tex".to_string(), ques, ans, n);
    progress(all_n, 3);

    //コンパイル
    shell();
    progress(all_n, 4);
}

// 問題・解答を生成
fn equ() -> (String, String) {
    let mut res: (String, String) = (String::new(), String::new());
    let a = get_rand(1, 10);
    let b = get_rand(-9, 10);
    let c = get_rand(1, 10);
    let d = get_rand(-9, 10);

    let p: i32 = a * c;
    let q: i32 = a*d + b*c;
    let r: i32 = b * d;

    let x2: String = kou(p, 2);
    let x1: String = kou(q, 1);
    let x0: String = kou(r, 0);

    res.0 = x2 + &x1 + &x0;
    res.1 = String::new() 
    + "(" + &a.to_string() + "x" + &kou(b, 0) + ")"
    + "(" + &c.to_string() + "x" + &kou(d, 0) + ")";
    return res;
}
// a以上b未満の乱数を取得
fn get_rand(a: i32, b :i32) -> i32 {
    let mut res: i32 = rand::thread_rng().gen_range(a, b-1);
    if res >= 0 { // 0は出ないようにする
        res += 1;
    }
    return res;
}
// 展開後の式の項を作成
fn kou(pqr: i32, e: i32) -> String {
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
        2 => res += "x^2",
        1 => res += "x",
        _ => {},
    }

    return res;
}

// texファイルに書き込み
fn write_tex(path: String, ques: String, ans: String, n: usize) {
       let mut file = File::create(path)
           .expect("file not found.");
        writeln!(file, "\\documentclass[11pt,a4paper,dvipdfmx]{{jsarticle}}").expect("cannot write.");
        writeln!(file, "\\usepackage{{amsmath,amssymb, minijs, pxfonts, multicol, enumerate}}").expect("cannot write.");
        writeln!(file, "\\usepackage[top=25.4truemm,bottom=25.4truemm,left=19.05truemm,right=19.05truemm]{{geometry}}").expect("cannot write.");
        writeln!(file, "\\begin{{document}}").expect("cannot write.");

        writeln!(file, "\t\\subsection*{{問題}}").expect("cannot write.");
        writeln!(file, "\t\\begin{{multicols}}{{3}}").expect("cannot write.");
        writeln!(file, "\t\t\\begin{{enumerate}}[(1)]").expect("cannot write.");
        write!(file, "{}", ques).expect("cannot write.");
        writeln!(file, "\t\t\\end{{enumerate}}").expect("cannot write.");
        writeln!(file, "\t\\end{{multicols}}").expect("cannot write.");

        writeln!(file, "\t\\newpage").expect("cannot write.");

        writeln!(file, "\t\\subsection*{{解答}}").expect("cannot write.");
        writeln!(file, "\t\\begin{{multicols}}{{3}}").expect("cannot write.");
        writeln!(file, "\t\t\\begin{{enumerate}}[(1)]").expect("cannot write.");
        write!(file, "{}", ans).expect("cannot write.");
        writeln!(file, "\t\t\\end{{enumerate}}").expect("cannot write.");
        writeln!(file, "\t\\end{{multicols}}").expect("cannot write.");

        writeln!(file, "\t\\subsection*{{正答率}}").expect("cannot write.");
        writeln!(file, "\t\\Huge\\hspace{{1cm}} /{}", n).expect("cannot write.");
        writeln!(file, "\\end{{document}}").expect("cannot write.");
}

fn shell() {
    let mut _output;
    _output = Command::new("platex")
        .arg("main.tex")
        .output()
        .expect("failed");
    _output = Command::new("dvipdfmx")
        .arg("main.dvi")
        .output()
        .expect("failed");
}

// k/n
fn progress(n: i32, k: i32) {
    print!("\x1B[2K");
    print!("\r");
    stdout().flush().unwrap();
    for _i in 0 .. k {
        print!("■");
    }
    for _i in k+1 .. n {
        print!("□");
    }
    stdout().flush().unwrap();
}