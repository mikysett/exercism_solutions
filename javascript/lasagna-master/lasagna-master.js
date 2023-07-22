/// <reference path="./global.d.ts" />
// @ts-check

/**
 * Implement the functions needed to solve the exercise here.
 * Do not forget to export them so they are available for the
 * tests. Here an example of the syntax as reminder:
 *
 * export function yourFunction(...) {
 *   ...
 * }
 */

export function cookingStatus(timeLeft) {
  if (timeLeft === undefined) {
    return "You forgot to set the timer.";
  }
  if (timeLeft === 0) {
    return "Lasagna is done.";
  } else {
    return "Not done, please wait.";
  }
}

export function preparationTime(layers, timePerLayer = 2) {
  return layers.length * timePerLayer;
}

export function quantities(layers) {
  return {
    noodles: layers.filter(layer => layer === "noodles").length * 50,
    sauce: layers.filter(layer => layer === "sauce").length * 0.2,
  };
}

export function addSecretIngredient(friendList, myList) {
  myList.push(friendList[friendList.length - 1]);
}

export function scaleRecipe(originalRecipe, portions) {
  let newRecipe = {};

  for (const ingredient in originalRecipe) {
    newRecipe[ingredient] = (originalRecipe[ingredient] / 2) * portions;
  }
  return newRecipe;
}
