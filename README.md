# durak-analysis
Analyzing the win rate for strategies in the Russian game Durak (Fool)

## Introduction

As a fond player of the Russian game Durak, I wanted to test how much of the game is Strategy versus Chance.

I paired a "Beginner" and an "Expert" against each other in a series of simulated millions of games.

The Beginner simply attacked and defended with the smallest playable card.

The Expert used every single strategy possible to win.

After much trial and error, I found strategies that increased the chances of winning, and ones that didn't.

This game was "Reversable Durak", meaning the defender can reverse the cards back at the attacker, if the defender has a matching card.

Unlike most online Durak games, I made sure that the computer never "cheated".

The players were only provided the following information:

* The players' own cards
* The cards that are currently being played on the table
* The Trump/Kozer
* How many cards are left in the deck

The players had no ability to look into the opponent's cards, the deck's cards, or the beaten "out" deck.

The players also had no ability to remember old played cards.

I did this because human players don't have the ability to remember all old cards, so it's unfair for a computer to do it.

## Results
With all of the strategies, the Expert only won about 71% of the time.

| Expert | Beginner | Tie |
| --- | --- | --- |
| 71% | 25% | 4% |

Advantage of going first at the beginning of the game:

| First | Second | Tie |
| --- | --- | --- |
| +1% | -1% | 0% |

### Expert Strategies

Below are the different strategies used by the expert.

Ordered from beginning strategies, to end game strategies.

Percents are rounded to the nearest 0.5% for readability.

#### Reverse using non-trump cards

Conditions:
* Defender has a matching non-trump card against the attacker.
Action: Reverse the cards.

| Strategy | No Strategy | Tie |
| --- | --- | --- |
| +3% | -2.5% | +0.5% |

Note: This is advantageous regardless of how many cards are in the deck.

#### Don't defend with trump cards

Conditions:
* More than 12 cards in the deck.
* Can't beat the attack with non-trump cards.
Action: Take the cards. Don't defend them with a trump.

| Strategy | No Strategy | Tie |
| --- | --- | --- |
| 0% | -0.5% | +0.5% |

#### Don't defend if you just have 1 trump card

Conditions:
* More than 9 cards in the deck.
* Can't beat the attack with non-trump cards.
* You only have a single trump card, and it's a high card.
Action: Take the cards. Don't defend them with the one trump card.

| Strategy | No Strategy | Tie |
| --- | --- | --- |
| +0.5% | -0.5% | 0% |

#### Take any trump cards given early in the game

Conditions:
* More than 6 cards in the deck.
* Attacking with a trump card.
Action: Take the cards. Don't defend them with a higher trump.

| Strategy | No Strategy | Tie |
| --- | --- | --- |
| +1% | -1% | 0% |

#### Don't attack with trumps until the end

Conditions:
* More than 4 cards in the deck. (Attacker is not going to get the last trump deck card at the end of the game.)
Action: Don't attack with with trumps. Attack with the smallest card that's not a trump.

| Strategy | No Strategy | Tie |
| --- | --- | --- |
| +9.5% | -10% | +0.5% |

#### Defensively stop attacker from getting the last card in deck

Conditions:
* Two cards left in the deck.
* Defender has trump a card that matches the attacker's card.
* Especially if defender's trump card is smaller than the trump card at the end of the deck.
Action: Defend using the matching trump card.

| Strategy | No Strategy | Tie |
| --- | --- | --- |
| +0.5% | -0.5% | 0% |

#### Attack with doubles at the end

Conditions:
* Two or less cards in the deck. (Or three or so cards in the deck and lots of cards are already on the table.)
* Attacker has more than four cards.
* Attacker has two or more cards with the same number.
Action: Attack with the card that has the same number. Including trumps.

| Strategy | No Strategy | Tie |
| --- | --- | --- |
| +4.5% | -3.5% | -1% |

#### Give Trump Ace First at the end

Conditions:
* No cards in the deck.
* Only two cards in the attacker's hands.
* Attacker has a Trump Ace.
Action: Attack with the Trump Ace first.

| Strategy | No Strategy | Tie |
| --- | --- | --- |
| +0.5% | 0% | -0.5% |

Note: this percentage includes other similar tricks, like using a trump King before using a trump Ace.

## Conclusion

If an absolute fool can still win 25% of the time against a master, this game is definitely more Chance than Strategy.

The most interesting strategy finding here has been Attacking with Doubles at the end of the game.

I'll be adjusting my play accordingly.

I otherwise wonder: If the player can remember one single piece of information throughout gameplay, what information would be most useful to win more?
