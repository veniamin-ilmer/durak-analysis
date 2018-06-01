extern crate rand;

#[derive(Copy, Clone)]
struct Card {
    value: u8,
    suit: u8,
}

const NUM_LOOKUP: &'static [char] = &['0','1','2','3','4','5','6','7','8','9','T','J','Q','K','A'];
const SUIT_LOOKUP: &'static [&'static str] = &["Hearts", "Diamonds", "Clubs", "Spades"];

use rand::Rng;

static mut DEBUG: bool = false;

fn main() {

  let mut player1_count: u64 = 0;
  let mut player2_count: u64 = 0;
  let mut tie_count: u64 = 0;
  
  for _ in 0..1000000 { //Number of Games
    let mut deck: Vec<Card> = vec![
                                Card{value: 6, suit: 0}, Card{value: 7, suit: 0}, Card{value: 8, suit: 0}, Card{value: 9, suit: 0}, Card{value: 10, suit: 0}, Card{value: 11, suit: 0}, Card{value: 12, suit: 0}, Card{value: 13, suit: 0}, Card{value: 14, suit: 0}, 
                                Card{value: 6, suit: 1}, Card{value: 7, suit: 1}, Card{value: 8, suit: 1}, Card{value: 9, suit: 1}, Card{value: 10, suit: 1}, Card{value: 11, suit: 1}, Card{value: 12, suit: 1}, Card{value: 13, suit: 1}, Card{value: 14, suit: 1},
                                Card{value: 6, suit: 2}, Card{value: 7, suit: 2}, Card{value: 8, suit: 2}, Card{value: 9, suit: 2}, Card{value: 10, suit: 2}, Card{value: 11, suit: 2}, Card{value: 12, suit: 2}, Card{value: 13, suit: 2}, Card{value: 14, suit: 2},
                                Card{value: 6, suit: 3}, Card{value: 7, suit: 3}, Card{value: 8, suit: 3}, Card{value: 9, suit: 3}, Card{value: 10, suit: 3}, Card{value: 11, suit: 3}, Card{value: 12, suit: 3}, Card{value: 13, suit: 3}, Card{value: 14, suit: 3},
                               ];
    
    let mut rng = rand::thread_rng();
    rng.shuffle(&mut deck);
    
    let mut player1: Vec<Card> = Vec::new();
    let mut player2: Vec<Card> = Vec::new();
    let mut table: Vec<Card> = Vec::new();  //Cards on the table that either have already been beat, or are currently attacking.
    let mut pending_cards: Vec<Card> = Vec::new();  //Cards that are pending to be attacked. Used during reversal.

    let trump = deck[0].suit;
    let last_card_value = deck[0].value;
    unsafe { if DEBUG { println!("Trump: {}", SUIT_LOOKUP[trump as usize]); } }
    let mut turn = rng.gen_range(0,2);
//    let mut turn = 0;
    let mut who_gets_cards_first = turn;
    let mut debug_loop_counter = 0;
    loop {  //Loop for 1 game.
      debug_loop_counter += 1;
      if debug_loop_counter > 1000 { unsafe { DEBUG = true; } }
      
      if who_gets_cards_first == 0 {
        deal_cards(&mut deck, &mut player1);
        deal_cards(&mut deck, &mut player2);
      } else {
        deal_cards(&mut deck, &mut player2);
        deal_cards(&mut deck, &mut player1);
      }

      let mut deck_len = deck.len();

      unsafe { if DEBUG {
        print!("deck length: {}", deck_len);
      
        print!("\nPlayer1: ");
        print_cards(&player1);
        print!("\nPlayer2: ");
        print_cards(&player2);
      } }
      
      if player1.len() == 0 && player2.len() == 0 {
        tie_count += 1;
        unsafe { if DEBUG { println!("**Tie**\n\n\n"); } }
        break;
      }
      if player1.len() == 0 {
        player1_count += 1;
        unsafe { if DEBUG { println!("**Player1 won**\n\n\n"); } }
        break;  //Break from game
      }
      if player2.len() == 0 {
        player2_count += 1;
        unsafe { if DEBUG { println!("**Player2 won**\n\n\n"); } }
        break;  //Break from game
      }
      
      let mut first_move_in_round = true; //First move in round, (not first move in game), means there is ability to reverse.

      loop {  //Loop per round. Round = attack and defend, until someone takes or table cleared. (Multiple rounds in a game.) 
        let attacker: &mut Vec<Card>;
        let defender: &mut Vec<Card>;
        if turn == 0 {
          unsafe { if DEBUG { println!("\nPlayer 1's turn"); } }
          attacker = &mut player1;
          defender = &mut player2;
        } else {
          unsafe { if DEBUG { println!("\nPlayer 2's turn"); } }
          attacker = &mut player2;
          defender = &mut player1;
        }
        
        
        let attack_card: Option<Card>;

        //Pending attack cards from a reversal.
        if pending_cards.len() > 0 {
          attack_card = pending_cards.pop();  //We will continue to pop the cards until there are none left.
        }
        else {
          if turn == 0 {  //Player 1 Attack
            attack_card = attack_smart(attacker, &table, trump, deck_len);
          } else {  //Player 2 Attack
            attack_card = attack_smallest(attacker, &table, trump);
          }

          if attack_card.is_none() {
            //Attacker has no other cards. Clear table.
            unsafe { if DEBUG { println!("Attacker does not want to give any more cards. Round done."); } }
            table.clear();
            who_gets_cards_first = turn; //Attacker gets cards first, even though defender successfully defended.
            turn = (turn + 1) % 2;  //Go to next player.
            break;  //Done with round. Attacker has no cards.
          }
        }
        
        //Continuing here means attacker has successfully attacked with a card.
        table.push(attack_card.unwrap());
        unsafe { if DEBUG { println!("Attacking with {} {}", NUM_LOOKUP[attack_card.unwrap().value as usize], SUIT_LOOKUP[attack_card.unwrap().suit as usize]); } }

        //Reversal only possible if it's the first move of the round, and not the first round of the game, and the defender has enough cards to defend the reversal.
        if deck_len != 24 && first_move_in_round && pending_cards.len() + 2 <= defender.len() {
          if turn == 1 {  //Player 1 defense
            let mut reverse_card = None;
            for i in 0..defender.len() {
              if defender[i].value == attack_card.unwrap().value && defender[i].suit != trump {
                unsafe { if DEBUG { println!("Reversing cards using {} {}", NUM_LOOKUP[defender[i].value as usize], SUIT_LOOKUP[defender[i].suit as usize]); } }
                reverse_card = Some(defender.remove(i));
                break;
              }
            }
            if reverse_card.is_some() {
              pending_cards.push(reverse_card.unwrap());
              pending_cards.push(attack_card.unwrap());  //Add the attack card back into the pending cards queue, as the next person will have to fight it too.
              table.clear();  //Clear the table. There should only be the one attack card there.
              turn = (turn + 1) % 2;  //Go to next player.
              who_gets_cards_first = turn; //Reverser gets cards first.
              continue;
            }
          }
        }
        
        first_move_in_round = false;  //Can't reverse any more.
        
        let defend_card;
        if turn == 0 {  //Player 2 defense
          defend_card = defend_smallest(defender, &mut attack_card.unwrap(), trump);
        } else {  //Player 1 defense
          defend_card = defend_smart(defender, &mut attack_card.unwrap(), trump, deck_len, &table, last_card_value);
        }
        
        if defend_card.is_none() {
          unsafe { if DEBUG { println!("Unable to defend."); } }
          defender.append(&mut table);  //The append will automatically clear the table too.
          defender.append(&mut pending_cards);  //If there are any pending attack cards, add them to the defender's deck too.
          who_gets_cards_first = turn; //Attacker gets cards first.
          break;  //Done with round. Defender takes.
        }
        
        //Continuing here means defender has successfully defended an attacking card.
        table.push(defend_card.unwrap());
        unsafe { if DEBUG { println!("Defending with {} {}", NUM_LOOKUP[defend_card.unwrap().value as usize], SUIT_LOOKUP[defend_card.unwrap().suit as usize]); } }
        
        if defender.len() == 0 {
          //Defender ran out of cards. This means the defender automatically beat this round. Attacker can't go any more.
          unsafe { if DEBUG { println!("Defender has run out of cards. Round done."); } }
          table.clear();
          who_gets_cards_first = turn; //Attacker gets cards first, even though defender successfully defended.
          turn = (turn + 1) % 2;  //Go to next player.
          break;
        }
        
        //After this, we loop. Attacker can still try to attack with another card, but only using numbers that are already on the table.
      }
    }
  }
  
  let total = player1_count + player2_count + tie_count;
  
  println!("Player 1: {:.1}%", (player1_count * 1000 / total) as f32 / 10.0);
  println!("Player 2: {:.1}%", (player2_count * 1000 / total) as f32 / 10.0);
  println!("Tie: {:.1}%", (tie_count * 1000 / total) as f32 / 10.0);
}

fn deal_cards(deck: &mut Vec<Card>, player: &mut Vec<Card>) {
  for _ in player.len()..6 {
    if deck.len() > 0 {
      player.push(deck.pop().unwrap());
    }
  }
}


fn print_cards(player: &Vec<Card>) {  
  for i in 0..player.len() {
    print!("{} {}   ", NUM_LOOKUP[player[i].value as usize], SUIT_LOOKUP[player[i].suit as usize]);
  }
}



//When we have some cards on the table, we have to make sure to only attack with valid cards.
fn in_table(table: &Vec<Card>, card: Card) -> bool {
  if table.len() == 0 { //New table? Allow all cards.
    return true;
  }
  for i in 0..table.len() {
    if card.value == table[i].value {
      return true;
    }
  }
  return false;
}



fn attack_smart(player: &mut Vec<Card>, table: &Vec<Card>, trump: u8, deck_len: usize) -> Option<Card> {


  if deck_len == 0 && player.len() == 2 { //Last two cards!
    //Give the trump Ace first
    if player[0].suit == trump && player[0].value == 14 && in_table(table, player[0]) {
      return Some(player.remove(0));
    }
    if player[1].suit == trump && player[1].value == 14 && in_table(table, player[1]) {
      return Some(player.remove(1));
    }

  }


  //3 cards. Got a trump Ace, trump King, and something else? Go with the trump King first!
  if deck_len == 0 && player.len() == 3 {
    for i in 0..player.len() {
      //Got an ace.
      if player[i].suit == trump && player[i].value == 14 {
        for j in 0..player.len() {
          //Got an king.
          if player[j].suit == trump && player[j].value == 13 && in_table(table, player[j]) {
            return Some(player.remove(j));
          }
        }
      }
    }
  }


  //Deck <= 2 - Use the smallest double, if its available.
//  if deck_len <= 2 {
  if (deck_len == 2 && player.len() > 4) || (deck_len == 3 && table.len() == 2 && player.len() > 4) || deck_len <= 1 {
    //First try the smallest numbers that include a trump.
    let mut smallest = 255;
    let mut smallesti = 255;
    for i in 0..player.len() {
      if player[i].suit == trump && player[i].value < smallest && in_table(table, player[i]) {
        for j in 0..player.len() {
          if player[i].value == player[j].value && i != j { //Found 2 in a row!
            smallest = player[i].value;
            if player[i].suit != trump {
              smallesti = i;
            } else if player[j].suit != trump {
              smallesti = j;
            }
          }
        }
      }
    }
    if smallesti != 255 {
      return Some(player.remove(smallesti));
    } else {
      //Next try the smallest numbers that don't include a trump
      for i in 0..player.len() {
        if player[i].suit != trump && player[i].value < smallest && in_table(table, player[i]) {
          for j in 0..player.len() {
            if player[i].value == player[j].value && i != j && player[j].suit != trump { //Found 2 in a row!
              smallest = player[i].value;
              smallesti = i;
            }
          }
        }
      }
      if smallesti != 255 {
        return Some(player.remove(smallesti));
      }
    }
  }




  //Big deck. Don't use trumps! (Unless using the trump will get you the last card!)
  if deck_len > 5 || (deck_len == 0 && table.len() < 2 && player.len() > 4) || (deck_len == 2 && table.len() < 2 && player.len() > 4) || (deck_len == 3 && table.len() < 4 && player.len() > 4) || (deck_len == 4 && table.len() < 6 && player.len() > 4) || (deck_len == 5 && table.len() < 8 && player.len() > 4) {
    let attack_card = attack_smallest_without_trump(player, table, trump);
    if attack_card.is_some() {
      return attack_card;
    } else {
      if table.len() != 0 { //Are there already some cards played in this round?
        return None;  //Yes? Don't give any trumps.
      } else { //This is a new round and all my cards are trumps?
        //I must give at least one card. I'm forced to give a trump. That will be done below.
      }
    }
  }


/*
  if deck_len ==4 {
    let mut smallest = 255;
    for i in 0..player.len() {
      if player[i].value < smallest {
        for j in 0..player.len() {
          if player[i].value == player[j].value && i != j {
            smallest = player[i].value;
          }
        }
      }
    }
    if smallest != 255 {
      let attack_card = attack_smallest_except(player, table, trump, smallest);
      if attack_card.is_some() {
        return attack_card;
      }
    }
  }  
*/

  return attack_smallest(player, table, trump);
  
}



fn attack_smallest_except(player: &mut Vec<Card>, table: &Vec<Card>, trump: u8, exc_value: u8) -> Option<Card> {

  let mut smallest = 255;
  let mut smallesti = 255;

  //First try without trump
  for i in 0..player.len() {
    if player[i].value < smallest && in_table(table, player[i]) && player[i].suit != trump && player[i].value != exc_value {
      smallest = player[i].value;
      smallesti = i;
    }
  }

  if smallest == 255 {
    return None;
  } else {
    return Some(player.remove(smallesti));
  }
}




fn attack_smallest(player: &mut Vec<Card>, table: &Vec<Card>, trump: u8) -> Option<Card> {

  let mut smallest = 255;
  let mut smallesti = 0;

  //First try without trump
  for i in 0..player.len() {
    if player[i].value < smallest && in_table(table, player[i]) && player[i].suit != trump {
      smallest = player[i].value;
      smallesti = i;
    }
  }

  if smallest == 255 {
    //Second, try with trump
    for i in 0..player.len() {
      if in_table(table, player[i]) && player[i].suit == trump {
        if player[i].value < smallest {
          smallest = player[i].value;
          smallesti = i;
        }
      }
    }
  }
  
  if smallest == 255 {
    return None;
  } else {
    return Some(player.remove(smallesti));
  }
}


fn attack_smallest_without_trump(player: &mut Vec<Card>, table: &Vec<Card>, trump: u8) -> Option<Card> {

  let mut smallest = 255;
  let mut smallesti = 0;

  for i in 0..player.len() {
    if in_table(table, player[i]) && player[i].suit != trump {
      if player[i].value < smallest {
        smallest = player[i].value;
        smallesti = i;
      }
    }
  }

  if smallest == 255 {
    return None;
  } else {
    return Some(player.remove(smallesti));
  }
}




fn defend_smart(player: &mut Vec<Card>, attack_card: &mut Card, trump: u8, deck_len: usize, table: & Vec<Card>, last_card_value: u8) -> Option<Card> {

/*
  if deck_len == 5 {
    let mut smallest = 255;
    let mut smallesti = 255;
    for i in 0..player.len() {
      if player[i].value < smallest && attack_card.value < player[i].value {
        for j in 0..player.len() {
          if player[i].value == player[j].value && i != j && attack_card.suit == player[i].suit {
            smallest = player[i].value;
            smallesti = i;
          }
        }
      }
    }
    if smallesti != 255 {
      return Some(player.remove(smallesti));
    }
  }
*/

/*
  if deck_len == 24 {
    let mut smallest = 255;
    for i in 0..player.len() {
      if player[i].value < smallest {
        for j in 0..player.len() {
          if player[i].value == player[j].value && i != j {
            smallest = player[i].value;
          }
        }
      }
    }
    if smallest != 255 {
      let defend_card = defend_smallest_except(player, attack_card, trump, smallest);
      if defend_card.is_some() {
        return defend_card;
      }
    }
  }
*/

  //There's only two cards left in the deck? Try to get the bottom trump by matching the current card!
  if deck_len == 2 && table.len() == 1 && player.len() <= 6 && last_card_value > attack_card.value-2 {
    for i in 0..player.len() {
      if player[i].value == attack_card.value && player[i].suit == trump {
        return Some(player.remove(i));
      }
    }
  }


  if attack_card.suit == trump && deck_len >= 7 {
    return None;  //Gladly take a trump card that is early in the game.
  }


  //Big deck. Don't use trumps!
  if deck_len >= 13 {
    return defend_smallest_without_trump(player, attack_card, trump);
  }


  let mut single_ace = false;
  let mut anothertrump = false;
  for i in 0..player.len() {
    if player[i].suit == trump && player[i].value >= 8 {
      single_ace = true;
    } else if player[i].suit == trump {
      anothertrump = true;
    }
  }
  
  if deck_len>=9 && single_ace == true && anothertrump == false {
    return defend_smallest_without_trump(player, attack_card, trump);
  }


  return defend_smallest(player, attack_card, trump);

}



fn defend_smallest(player: &mut Vec<Card>, attack_card: &mut Card, trump: u8) -> Option<Card> {

  let mut smallest = 255;
  let mut smallesti = 0;

  //First try to just match attacker's suit. (This may include the trump)
  for i in 0..player.len() {
    if attack_card.suit == player[i].suit && attack_card.value < player[i].value {
      if player[i].value < smallest {
        smallest = player[i].value;
        smallesti = i;
      }
    }
  }
  
  if smallest == 255 && attack_card.suit != trump {
    //Second, try with trump. (If it wasn't the attacker's suit.)
    for i in 0..player.len() {
      if player[i].suit == trump {
        if player[i].value < smallest {
          smallest = player[i].value;
          smallesti = i;
        }
      }
    }
  }
  
  if smallest == 255 {
    return None;
  } else {
    return Some(player.remove(smallesti));
  }
}




fn defend_smallest_except(player: &mut Vec<Card>, attack_card: &mut Card, trump: u8, exc_value: u8) -> Option<Card> {

  let mut smallest = 255;
  let mut smallesti = 0;

  //First try to just match attacker's suit. (This may include the trump)
  for i in 0..player.len() {
    if attack_card.suit == player[i].suit && attack_card.value < player[i].value && player[i].value != exc_value {
      if player[i].value < smallest {
        smallest = player[i].value;
        smallesti = i;
      }
    }
  }
  
  if smallest == 255 && attack_card.suit != trump {
    //Second, try with trump. (If it wasn't the attacker's suit.)
    for i in 0..player.len() {
      if player[i].suit == trump && player[i].value != exc_value {
        if player[i].value < smallest {
          smallest = player[i].value;
          smallesti = i;
        }
      }
    }
  }
  
  if smallest == 255 {
    return None;
  } else {
    return Some(player.remove(smallesti));
  }
}



fn defend_smallest_without_trump(player: &mut Vec<Card>, attack_card: &mut Card, trump: u8) -> Option<Card> {

  let mut smallest = 255;
  let mut smallesti = 255;

  if attack_card.suit == trump {  //attacker attacking with a trump?
    return None;  //Take it.
  }

  //First to just match attacker's suit. (This may include the trump)
  for i in 0..player.len() {
    if attack_card.suit == player[i].suit && attack_card.value < player[i].value {
      if player[i].value < smallest {
        smallest = player[i].value;
        smallesti = i;
      }
    }
  }
  
  if smallest == 255 {
    return None;
  } else {
    return Some(player.remove(smallesti));
  }

}

