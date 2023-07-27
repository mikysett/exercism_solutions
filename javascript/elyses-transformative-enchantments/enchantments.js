// @ts-check

/**
 * Double every card in the deck.
 *
 * @param {number[]} deck
 *
 * @returns {number[]} deck with every card doubled
 */
export function seeingDouble(deck) {
  return deck.map((v) => v * 2);
}

/**
 *  Creates triplicates of every 3 found in the deck.
 *
 * @param {number[]} deck
 *
 * @returns {number[]} deck with triplicate 3s
 */
export function threeOfEachThree(deck) {
  // let lastIndex = deck.findIndex((v) => v == 3);
  // let currIndex = lastIndex;

  // while (currIndex != -1) {
  //   currIndex += lastIndex;
  //   deck.splice(currIndex, 0, 3, 3);

  //   lastIndex = currIndex + 3;
  //   currIndex = deck.slice(lastIndex).findIndex((v) => v == 3);
  // }
  // return deck;

  // More concise solution
  // return deck.flatMap(card => card == 3 ? [3, 3, 3] : [card])

  return deck.reduce((res, card) => {
    res.push(card)
    if (card == 3) {
      res.push(3, 3)
    }
    return res
  }, [])
}

/**
 * Extracts the middle two cards from a deck.
 * Assumes a deck is always 10 cards.
 *
 * @param {number[]} deck of 10 cards
 *
 * @returns {number[]} deck with only two middle cards
 */
export function middleTwo(deck) {
  return deck.splice(deck.length / 2 - 1, 2);
}

/**
 * Moves the outside two cards to the middle.
 *
 * @param {number[]} deck with even number of cards
 *
 * @returns {number[]} transformed deck
 */

export function sandwichTrick(deck) {
  const first = deck.shift()
  const last = deck.pop()

  deck.splice(deck.length / 2, 0, last, first);
  return deck;
}

/**
 * Removes every card from the deck except 2s.
 *
 * @param {number[]} deck
 *
 * @returns {number[]} deck with only 2s
 */
export function twoIsSpecial(deck) {
  return deck.filter((v) => v == 2);
}

/**
 * Returns a perfectly order deck from lowest to highest.
 *
 * @param {number[]} deck shuffled deck
 *
 * @returns {number[]} ordered deck
 */
export function perfectlyOrdered(deck) {
  return deck.sort((a, b) => a - b);
}

/**
 * Reorders the deck so that the top card ends up at the bottom.
 *
 * @param {number[]} deck
 *
 * @returns {number[]} reordered deck
 */
export function reorder(deck) {
  return deck.reverse();
}
