use std::io::{repeat, stdin};
use std::io;
use std::fs;
pub use std::sync::atomic::AtomicU64;
use std::sync::atomic::AtomicUsize;
pub use std::sync::atomic::Ordering;
//use bitvec::array::BitArray;

//static FLAG_KW: AtomicUsize = AtomicUsize::new(0);

static KEYWORDS:  [&str; 15] = ["if","else","void","return","while","then","for","do", "int","char","double","float","case","cin","cout"];
static INDEX_KW:  [i32; 15]  = [1, 2, 3, 4, 5, 6, 7, 8, 9,10 ,11, 12, 13, 14, 15];

static OPERATOR:  [&str; 9]  = ["+", "-", "*", "/", "%",">","<","=","!"];
static INDEX_OP:  [i32; 9]   = [16, 17, 18, 19, 20, 21, 22, 23, 24];

static SEPARATER: [&str; 8]  = [";", "," ,"{", "}" ,"[", "]", "(", ")"];
static INDEX_SEP: [i32; 8]   = [25, 26, 27, 28, 29, 30, 31, 32];

static FILTER:    [&str; 4]  = [" ", "\t", "\r", "\\n"];
static INDEX_FIL: [i32; 4]   = [33, 34, 34, 36];

fn main() {
    //let mut holes = BitArray::default();
    //holes.set(0, true);
    let text = GetText();
    TextAnalyse(text.clone());
    let filter_text = KnownElementsFilter(text.clone());
    CustomNumsAnalyse(filter_text.clone());
    let nums_filter_text = KnownNumsFilter(filter_text.clone());
    CustomElementsAnalyse(nums_filter_text.clone());

    println!("---------------------------\n");
    println!("          分析完毕\n          ");
    println!("---------------------------\n");
    //println!("{}", nums_filter_text);
    //println!("{}",filter_text);
}

fn GetText() -> String {
    let mut str_buf = String::new();

    println!("---------------------------\n");
    println!("      键入读取文件方式:\n     ");
    println!("1:从键盘输入   2:从文本取流\n");
    println!("---------------------------\n");

    stdin().read_line(&mut str_buf)
        .expect("Failed to read line.");

    let read_mode_flag: u32 = str_buf.trim().parse()
        .expect("Please type a number!");

    str_buf = String::new();

    if read_mode_flag == 1 {
        println!("---------------------------\n");
        println!("       键入目标文本:\n        ");
        println!("---------------------------\n");

        stdin().read_line(&mut str_buf)
            .expect("Failed to read line.");
        let text = str_buf;
        str_buf = String::new();
        //println!("Your input line is \n{}", text);
        return text;
    }
    else if read_mode_flag == 2 {
        println!("---------------------------\n");
        println!("      键入目标文本路径:\n      ");
        println!("---------------------------\n");
        stdin().read_line(&mut str_buf)
            .expect("Failed to read line.");
        let text = fs::read_to_string( "/home/sleepingmachine/CLionProjects/LexicalAnalyzer/test.txt").unwrap();
        str_buf = String::new();
        //println!("Your input line is \n{}", text);
        return text;
    }
    else{
        println!("---------------------------\n");
        println!("          无效输入:\n         ");
        println!("---------------------------\n");
        return String::new();
    }
}

fn TextAnalyse(text: String){
    println!("---------------------------\n");
    println!("Your input text is \n{}", text);
    println!("---------------------------\n");

    KeyWordsJudge(text.clone());
    OperatorJudge(text.clone());
    SeparaterJudge(text.clone());
    FilterJudge(text.clone());

    //let text_bytes = text.as_bytes();

    //let mut keywords_search_list= [0, 1, 2, 3, 4, 5, 6, 7, 8, 9,10 ,11, 12, 13, 14];
    /*
    for (i, &item) in text_bytes.iter().enumerate() {
        println!("{} {}", i, item as char);

        OperatorJudge(item as char);
        SeparaterJudge(item as char);
    }*/
}
fn KeyWordsJudge(text:String){
    let mut search_text = text.clone();
    let mut final_text = text.clone();
    for (index,val) in KEYWORDS.iter().enumerate() {
        let mut text_position_correction = 0;
        loop {
            if search_text.contains(val) {
                println!("在第 {} 位置找到关键字 \"{}\",其index为 {}",search_text.find(val).unwrap() + text_position_correction, val, INDEX_KW[index]);
                //final_text = final_text.remove(search_text.find(val).unwrap() + text_position_correction, search_text.find(val).unwrap() + text_position_correction + )
                text_position_correction += search_text.find(val).unwrap();
                search_text = search_text[search_text.find(val).unwrap()+val.len()..].to_string();
                println!("{}", search_text);
            }
            else {
                search_text = text.clone();
                break
            }
        }
        //final_text = final_text.replace(val, "~");
    }
    //println!("{}", final_text);
    //return final_text;
}

fn OperatorJudge(text:String){
    let mut search_text = text.clone();
    let mut final_text = text.clone();
    for (index,val) in OPERATOR.iter().enumerate() {
        let mut text_position_correction = 0;
        loop {
            if search_text.contains(val) {
                println!("在第 {} 位置找到运算符 \"{}\",其index为 {}",search_text.find(val).unwrap() + text_position_correction, val, INDEX_OP[index]);
                //final_text = final_text.remove(search_text.find(val).unwrap() + text_position_correction, search_text.find(val).unwrap() + text_position_correction + )
                text_position_correction += search_text.find(val).unwrap();
                search_text = search_text[search_text.find(val).unwrap()+val.len()..].to_string();
                println!("目前的搜索范围: {}", search_text);
            }
            else {
                search_text = text.clone();
                break
            }
        }
        //final_text = final_text.replace(val, "~");
    }
    //println!("{}", final_text);
    //return final_text;
}

fn SeparaterJudge(text:String){
    let mut search_text = text.clone();
    let mut final_text = text.clone();
    for (index,val) in SEPARATER.iter().enumerate() {
        let mut text_position_correction = 0;
        loop {
            if search_text.contains(val) {
                println!("在第 {} 位置找到分隔符 \"{}\",其index为 {}",search_text.find(val).unwrap() + text_position_correction, val, INDEX_SEP[index]);
                //final_text = final_text.remove(search_text.find(val).unwrap() + text_position_correction, search_text.find(val).unwrap() + text_position_correction + )
                text_position_correction += search_text.find(val).unwrap();
                search_text = search_text[search_text.find(val).unwrap()+val.len()..].to_string();
                println!("目前的搜索范围: {}", search_text);
            }
            else {
                search_text = text.clone();
                break
            }
        }
        //final_text = final_text.replace(val, "~");
    }
    //println!("{}", final_text);
    //return final_text;
}

fn  FilterJudge(text:String){
    let mut search_text = text.clone();
    let mut final_text = text.clone();
    for (index,val) in FILTER.iter().enumerate() {
        let mut text_position_correction = 0;
        loop {
            if search_text.contains(val) {
                println!("在第 {} 位置找到转义符 \"{}\",其index为 {}",search_text.find(val).unwrap() + text_position_correction, val, INDEX_FIL[index]);
                //final_text = final_text.remove(search_text.find(val).unwrap() + text_position_correction, search_text.find(val).unwrap() + text_position_correction + )
                text_position_correction += search_text.find(val).unwrap();
                search_text = search_text[search_text.find(val).unwrap()+val.len()..].to_string();
                println!("目前的搜索范围: {}", search_text);
            }
            else {
                search_text = text.clone();
                break
            }
        }
        //final_text = final_text.replace(val, "~");
    }
    //println!("{}", final_text);
    //return final_text;
}

fn KnownElementsFilter(text:String) -> String{
    let mut filter_text = text.clone();
    for (index,val) in FILTER.iter().enumerate() {
        let mut space_string_vec: Vec<char> = Vec::new();
        for i in 0..val.len() {
            space_string_vec.push(' ');
        }
        let mut space_string: &str = &space_string_vec.iter().collect::<String>();
        filter_text = filter_text.replace(val, space_string);
    }
    for (index,val) in KEYWORDS.iter().enumerate() {
        let mut space_string_vec: Vec<char> = Vec::new();
        for i in 0..val.len() {
            space_string_vec.push(' ');
        }
        let mut space_string: &str = &space_string_vec.iter().collect::<String>();
        filter_text = filter_text.replace(val, space_string);
    }
    for (index,val) in SEPARATER.iter().enumerate() {
        let mut space_string_vec: Vec<char> = Vec::new();
        for i in 0..val.len() {
            space_string_vec.push(' ');
        }
        let mut space_string: &str = &space_string_vec.iter().collect::<String>();
        filter_text = filter_text.replace(val, space_string);
    }
    for (index,val) in OPERATOR.iter().enumerate() {
        let mut space_string_vec: Vec<char> = Vec::new();
        for i in 0..val.len() {
            space_string_vec.push(' ');
        }
        let mut space_string: &str = &space_string_vec.iter().collect::<String>();
        filter_text = filter_text.replace(val, space_string);
    }
    return filter_text;
}

fn CustomNumsAnalyse(text:String){
    let text_bytes = text.as_bytes();
    let mut num_position_correction = 0;
    let mut num = 0;

    for i in 0..text_bytes.len()-1 {
        if text_bytes[i] >= 48 && text_bytes[i] <= 57 {

            //println!("{}",text_bytes[i] as i32 - 48);
            if text_bytes[i+1] >= 48 && text_bytes[i] <= 57 {
                num = num * 10 + (text_bytes[i] as i32 - 48);
                num_position_correction += 1;
            }
            else {
                num = num * 10 + (text_bytes[i] as i32 - 48);
                num_position_correction += 1;
                println!("在第{}位置找到int {}",i-1,num);
                num_position_correction = 0;
                num = 0;
                }
            }
/*
            if text_bytes[i+1] != 32{
                num_counter.push(text_bytes[i] as char);
            }
            else if text_bytes[i+1] == 32 {

                num_counter.push(text_bytes[i] as char);
                for i in 0..num_counter.len()-1 {
                    println!("{}", num_counter[i] as i32 - 48);
                    num = num * 10 + (num_counter[i] as i32 - 48);
                    //println!("~ {}",num);
                    num = 0;
                    num_counter = Vec::new();
                }
            }*/

    }
}

fn KnownNumsFilter(text:String) -> String{
    let mut filter_text = text.clone();
    filter_text = filter_text.replace("0", " ");
    filter_text = filter_text.replace("1", " ");
    filter_text = filter_text.replace("2", " ");
    filter_text = filter_text.replace("3", " ");
    filter_text = filter_text.replace("4", " ");
    filter_text = filter_text.replace("5", " ");
    filter_text = filter_text.replace("6", " ");
    filter_text = filter_text.replace("7", " ");
    filter_text = filter_text.replace("8", " ");
    filter_text = filter_text.replace("9", " ");
    println!("");
    return filter_text;
}

fn CustomElementsAnalyse(text:String){
    let text_bytes = text.as_bytes();
    let mut num_position_correction = 0;
    let mut num = 0;
    let mut custom_elemens:Vec<char> = Vec::new();
    for i in 0..text_bytes.len()-1 {

        if (text_bytes[i] >= 65 && text_bytes[i] <= 90) || (text_bytes[i] >= 97 && text_bytes[i] <= 122) {
            if (text_bytes[i+1] >= 65 && text_bytes[i] <= 90) || (text_bytes[i+1] >= 97 && text_bytes[i] <= 122) {
                custom_elemens.push(text_bytes[i] as char);
            }
            else {
                custom_elemens.push(text_bytes[i] as char);
                let custom_elemens_string: String = custom_elemens.iter().collect::<String>();
                println!("在第{}位置找到自声明元素 {}",i-custom_elemens_string.len()+1,custom_elemens_string);
                custom_elemens = Vec::new();
            }
            //println!("{}",text_bytes[i] as char);
        }
    }
}
/*
fn GetSpaceString (target_string: &str) -> &str {
    // 起始：Vec
    let mut space_string_vec: Vec<char> = Vec::new();
    for i in 0..target_string.len()-1 {
        space_string_vec.push(' ');
    }
    let mut space_string: &str = &space_string_vec.iter().collect::<String>();
    return space_string;
}*/
/*

    for (index,val) in OPERATOR.iter().enumerate() {
        if test_char == *val {
            //print!("{} ", index);
            println!("~ Get Operator : {} Index : {}", val, INDEX_OP[index]);
        }
    }
}

fn SeparaterJudge(test_char: char) {
    for (index,val) in SEPARATER.iter().enumerate() {
        if test_char == *val {
            //print!("{} ", index);
            println!("~ Get Separater: {} Index : {}", val, INDEX_SEP[index]);
        }
    }
}

fn KeyWordJudge(test_char: char, keywords_search_list: & mut [i32;15]) {
    for (index,val) in KEYWORD.iter().enumerate() {
        let mut seach_flag = 0;
        for i in *keywords_search_list {
            //print!("{}",keywords_search_list[i as usize].to_string());
            if index as i32 == i {
                seach_flag = 1;
            }
        }
        if seach_flag != 1 {
            continue
        }
        let val_bytes: String = (**val).to_string();
        let val_char: Vec<char> = val_bytes.chars().collect::<Vec<_>>();
        //println!("{}",FLAG_KW.load(Ordering::SeqCst));
        //println!("Debug: Searching text: {} Searching Target:{}", test_char, val_char[FLAG_KW.load(Ordering::SeqCst)]);

        if FLAG_KW.load(Ordering::SeqCst) > val_char.len()-1 {

            continue;
        }
        println!("Debug: Searching text: {} Searching Target:{} now target size = : {}", test_char, val_char[FLAG_KW.load(Ordering::SeqCst)],val_char.len()-1);

        if test_char != val_char[FLAG_KW.load(Ordering::SeqCst)] && FLAG_KW.load(Ordering::SeqCst) <= val_char.len()-1{
            for i in (0..keywords_search_list.iter().count()){
                if(keywords_search_list[i] == index as i32){
                    keywords_search_list[i] = -1;
                }
                else {

                }
            }
            continue
        }
        if test_char == val_char[FLAG_KW.load(Ordering::SeqCst)] && FLAG_KW.load(Ordering::SeqCst) != val_char.len()-1 {
            println!("Match Keywords..");
            FLAG_KW.fetch_add(1, Ordering::SeqCst);
            //break;
            //return;
        }
        if test_char == val_char[FLAG_KW.load(Ordering::SeqCst)] && FLAG_KW.load(Ordering::SeqCst) == val_char.len()-1 {
            println!("~ Get Keywords: {}", val);
            FLAG_KW.fetch_sub(FLAG_KW.load(Ordering::SeqCst), Ordering::SeqCst);
            break;
        }
    }
*/