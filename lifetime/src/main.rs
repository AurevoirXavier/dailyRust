struct ImportantExcerpt<'a> {
    part: &'a str
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
//    {
//        let r;                 // -------+-- 'a
//                               //        |
//        {                      //        |
//            let x = 5;        // -+-----+-- 'b
//            r = &x;           //  |     |
//        }                     // -+     |
//                              //        |
//        println!("r: {}", r); // |
//                              //        |
//                              // -------+
//    }

    {
        let x = 5;            // -----+-- 'b
                                  //      |
        let r = &x;          // --+--+-- 'a
                                  //   |  |
        println!("r: {}", r);     //   |  |
                                  // --+  |
    }                             // -----+

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);

    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());

        println!("The longest string is {}", result);
    }

    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result;

    {
        result = longest(string1.as_str(), string2.as_str());
    }

    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago..");
    let first_sentence = novel.split('.').next().expect("Could not fina a '.'");
    let i = ImportantExcerpt { part: first_sentence };

    println!("{}", i.part);

    longest_with_an_announcement(string1.as_str(), string2.as_str(), 'A');
}