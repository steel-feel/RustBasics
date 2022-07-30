use std::ops::BitOr;

#[derive(Debug)]
struct Player {
    name: String,
    runs: u32,
}

impl BitOr for Player {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        if self.runs > rhs.runs {
            self
        } else {
            rhs
        }
    }
}

fn operator_overloading(){
    let player1 = Player {
        name: String::from("Sachin"),
        runs: 74,
    };
    let player2 = Player {
        name: String::from("Sehwag"),
        runs: 109,
    };
    let better_player = player1 | player2;
    println!("Better player {}", better_player.name);
}

#[derive(Clone)]
struct Bird(String);

//Clone trait
fn clone_trait(){

    let mut bird1 = Bird(String::from("Sparrow"));
    let bird2 = bird1.clone();

    bird1 = Bird(String::from("King-fisher")) ;

    println!("Two bird's are {} and {}", bird1.0, bird2.0);


}

fn main() {
    // operator_overloading();
    clone_trait();
}
