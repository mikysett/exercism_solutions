// @ts-check

/**
 * Determine how many cards of a certain type there are in the deck
 *
 * @param {number[]} stack
 * @param {number} card
 *
 * @returns {number} number of cards of a single type there are in the deck
 */
export function cardTypeCheck(stack, card) {
  let count = 0;
  stack.forEach((nb, _) => {
    if (nb == card) count++;
  });
  return count;
}

/**
 * Determine how many cards are odd or even
 *
 * @param {number[]} stack
 * @param {boolean} type the type of value to check for - odd or even
 * @returns {number} number of cards that are either odd or even (depending on `type`)
 */
export function determineOddEvenCards(stack, type) {
  let count = 0;
  let check = type ? (nb) => nb % 2 == 0 : (nb) => nb % 2 != 0;
  for (const card of stack) {
    if (check(card)) {
      count++;
    }
  }
  return count;
}
