/// <reference path="./global.d.ts" />
//
// @ts-check

/**
 * Determine the prize of the pizza given the pizza and optional extras
 *
 * @param {Pizza} pizza name of the pizza to be made
 * @param {Extra[]} extras list of extras
 *
 * @returns {number} the price of the pizza
 */

const PIZZA = {
  Margherita: 7,
  Caprese: 9,
  Formaggio: 10
}

const EXTRAS = {
  ExtraSauce: 1,
  ExtraToppings: 2
}

export function pizzaPrice(pizza, ...extras) {
  return PIZZA[pizza] + extrasPrice(...extras)
}

function extrasPrice(current, ...others) {
  // console.log('current: ', current)
  // console.log('others: ', others)
  if (current === undefined) {
    return 0
  }
  return EXTRAS[current] + extrasPrice(...others)
}

/**
 * Calculate the prize of the total order, given individual orders
 *
 * @param {PizzaOrder[]} pizzaOrders a list of pizza orders
 * @returns {number} the price of the total order
 */
export function orderPrice(pizzaOrders) {
  return pizzaOrders.reduce((total, order) => total + pizzaPrice(order.pizza, ...order.extras), 0)
}
