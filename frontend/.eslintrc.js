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
    "import/no-duplicates": "off",
    "vue/no-v-html": "off",
    "vue/html-indent": "off",
    "vue/one-component-per-file": "off",
    "vue/singleline-html-element-content-newline": "off",
    "vue/multiline-html-element-content-newline": "off",
    "vue/mustache-interpolation-spacing": "off",
    "vue/attributes-order": "off",
    "vue/html-self-closing": "off",
    "vue/html-closing-bracket-newline": "off",
    "vue/html-closing-bracket-spacing": "off",
    "vue/no-multi-spaces": "off",
  },
};
