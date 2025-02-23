use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {

   println!("Adivinhe o numero!!");
   let secret_number: u32 = rand::thread_rng().gen_range(1..=6);
   //println!("O numero secreto é:: {secret_number}");
   println!("Escreva seu chute!");
   println!("Dica: Roleta Russa.");

   loop{


      let mut guess = String::new();

      io::stdin()
          .read_line(&mut guess)
          .expect("Falha");

      let guess: u32 = match guess.trim().parse() {
         Ok(num) => num,
         Err(_) => continue,
      };
      //dá erro quando input é != número
      //let guess: u32 = guess.trim().parse().expect("Please type a number!");
      println!("Você adivinhou: {}", guess);

      match guess.cmp(&secret_number){
         Ordering::Less => println!("Pequeno demaise"),
         Ordering::Equal => {
            println!("Você ganhou!!");
            break;
         },
         Ordering::Greater => println!("Grande demaize"),
      }

   }
}