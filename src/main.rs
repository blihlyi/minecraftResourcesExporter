use std::fs::read_to_string;
use serde_json::Value;
fn getfiledir(dafn: &str)  -> String{
    let dirs1: Vec<&str> = dafn.split("/").collect();
    let dirs = &dirs1[..dirs1.len()-1];
    let mut resultdir = String::new();
    for i in dirs {
        resultdir = resultdir + i +"/";
    }
    return resultdir;
}
fn main(){
    let inputdir = "./assets/";
    let outputdir = "./output/";
    let indexfile = inputdir.to_owned() + "indexes/5.json";
    let text = match read_to_string(&indexfile) {Ok(x) => x,Err(y) => {println!("错误：{}\n操作失败",y);return}};
//转换JSON字符串
    let data: Value = match serde_json::from_str(&text) {Ok(x) => x,Err(y) => {println!("错误：{}\n操作失败",y);return}};
//获取objects对象
    let objs = match data["objects"].as_object() {Some(x) => x,None => {println!("错误：\"{}\"不是有效的minecraft资源索引文件\n操作失败（objects）",indexfile);return}};
//遍历每一个子对象
    for i in objs.iter() {
//获取散列资源文件原文件名
        let nostr = &i.1["hash"].as_str();
        let hashfilename = match nostr {Some(x) => x,None => {println!("错误：\"{}\"不是有效的minecraft资源索引文件\n操作失败（hash）",indexfile);return}};
//获取散列资源文件原路径
        let hashfiledir = inputdir.to_owned() + "objects/"+&hashfilename[..2]+"/";
//获取资源位置
        let resourcepath = outputdir.to_owned() + &i.0;
//创建文件夹
        let _ =std::fs::create_dir_all(&getfiledir(&(resourcepath)));
//将散列资源文件改名并移动到指定目录
        let _ =std::fs::copy(hashfiledir + hashfilename , resourcepath.clone());
        println!("已操作：{}", i.0);
    }
    println!("\n\n完成!");
    return;
}
