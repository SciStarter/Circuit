module.exports = {
  root: true,
  env: {
    browser: true,
    node: true,
  },
  parserOptions: {
    parser: "babel-eslint",
  },
  extends: [
    "@nuxtjs",
    "plugin:nuxt/recommended",
  ],
  plugins: [],
  // add your custom rules here
  rules: {
    indent: "off",
    camelcase: "off",
    semi: "off",
    "keyword-spacing": "off",
    "space-before-function-paren": "off",
    "no-console": "off",
    "vue/no-v-html": "off",
    "vue/html-indent": "off",
    "vue/one-component-per-file": "off",
  },
};
