// USally this is we use fns in every programming lang but due to a concept
// called ownership in rust we cant use a variable outside of its scope \
// bcoz unused variables are instanly dropped

// fn main() {
//     let s = String::from("Manan Arora");
//     let len: usize = calculate_len(s);
 
//     println!("the size of {s} is {len}")
//  }
 
//  fn calculate_len(s: String)-> usize {
//      s.len()
//  }
 


 // this is how we can do the same thing but taking ownership into consideration

 fn main() {
    let s = String::from("Manan Arora");
    let (s,len)= calculate_len(s);
 
    println!("the size of {s} is {len}")
 }
 
 fn calculate_len(s: String)-> (String ,usize ) {
     let result = s.len();
     (s ,result)
 }
 
//  By returning the string along with its length, you maintain ownership of s 
//in main. This allows you to use s later in the println! statement without any 
//errors. In short, your solution effectively sidesteps the ownership issue by
//explicitly returning the ownership back to the calling scope.