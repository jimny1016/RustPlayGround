fn main() {
    let verses = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let lyricses = ["A partridge in a pear tree", "Two turtle doves", "Three french hens", "Four calling birds", "Five golden rings", "Six geese a-laying", "Seven swans a-swimming", "Eight maids a-milking", "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"];

    for (versesIndex, verse) in verses.iter().enumerate() {
        println!("On the {} day of Christmas, my true love sent to me", verse);

        if versesIndex > 0 {
            for i in (1..=versesIndex).rev() {
                println!("{}{}", lyricses[i], if i == 1 { ", and " } else { ", " });
            }
        }

        println!("{}", lyricses[0]);
        println!();
    }
}