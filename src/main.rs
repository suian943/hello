fn main() {
    let varname = 5 ;
    let varname_tolast = 6;
    another_function(varname,varname_tolast);
    println!();
    println!();

    // 值与函数并写
    let k ={
        five(varname,varname_tolast)
    };
    

    println!("k={}", k);
    // 函数返回写法
    fn five(varname: i32,varname_tolast: i32) -> i32{
        let res = 5;
        println!("{}",res);
        // return res;
        //当最后一行为公式且具备rust返回值时 具备return属性 return 无论何时都可返回一个值且终止当前函数
        varname+varname_tolast
    }
    println!("five{}", five(varname,varname_tolast));



    // 条件判断写法
    let number = 3;
    if number > 5 {
        println!("Niha");
    }else{
        println!("Ni2a");
    }
    
    let mut var = 1;

    // while 循环
    while var != 5 {
        print!("{} ",var);
        var += 1 ;
    }

    // 循环体与迭代器
    let a = [10,20,30,40,50,60,70,80,90,100];
    for a in a.iter() {
        print!("{}",a);
    }
    println!();
    // 循环体与下标
    let mut ai = 0;
    for i in 0..5 {
        println!("a{} = {}",i,a[i]);
        ai += a[i];
    }
    println!("{}",ai);

    // 无线循环
    loop {
        if ai==100 {
            break;
        }
        ai += a[ai];
        println!("无线循环{}",ai);
    }


}
// 一个函数
fn another_function(x: i32, y: i32) {
    println!("X={}",x);
    println!("Y={}",y);
}