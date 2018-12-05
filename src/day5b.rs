fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let chain = stdin.lock().lines().next().unwrap().unwrap();

    println!("{:?}", (('a' as u32)..('z' as u32) + 1).map(|c| chain.chars()
                          .filter(|a| a.to_lowercase().to_string() != std::char::from_u32(c).unwrap().to_string())
                          .fold(String::from(""), |mut acc, c| {
                              let l = acc.chars().last();
                              match l {
                                  None => acc.push(c),
                                  Some(l) => if l.to_lowercase().to_string() == c.to_lowercase().to_string()
                                      && (l.is_lowercase() && c.is_uppercase()
                                          || l.is_uppercase() && c.is_lowercase()) {
                                          acc.pop();
                                      }
                                  else {
                                      acc.push(c);
                                  },

                              }
                              acc
                          })
                          .chars().count()).min());
}
