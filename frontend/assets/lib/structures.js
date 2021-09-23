import Vue from "vue";

export default {
  // Vue need to be kept informed of changes to arrays, even though
  // there's no way to add a getter or setter for array items, so we
  // need to use Vue.set to update the array elements.
  random_order(array) {
    for (const [i, x] of array.entries()) {
      const j = Math.floor(Math.random() * (i + 1));
      Vue.set(array, i, array[j]);
      Vue.set(array, j, x);
    }
  },
};
