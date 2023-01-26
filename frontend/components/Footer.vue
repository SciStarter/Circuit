<template>
<footer class="snm-wrapper" :class="{'homepage': $route.path == '/'}">
    <div class="snm-container">
    <ul>
      <li><h1>For Everyone</h1></li>
      <li>
        <nuxt-link to="/about">
          About Us
        </nuxt-link>
      </li>
      <li>
        <nuxt-link to="/contact">
          Contact Us
        </nuxt-link>
      </li>
    </ul>

    <ul>
      <li><h1>For Providers</h1></li>
      <li>
        <nuxt-link to="/add-opportunities">
          Add Your Opportunities to Science Near Me
        </nuxt-link>
      </li>
      <li>
        <nuxt-link to="/display-opportunities">
          Display Science Near Me Opportunities on Your Website
        </nuxt-link>
      </li>
      <li>
        <external-link href="/api/docs/v1.html" content="footer-link">
          API Documentation
        </external-link>
      </li>
    </ul>

    <div class="partner">
      <div class="nsf-logo">
        <img src="~assets/img/NSF-small.png">
      </div>
      <div class="description">
        This project is based upon work supported, in part, by the
        National Science Foundation under Grant DRL-1906998. Any
        opinions, findings, and conclusions or recommendations
        expressed in this material are those of the authors and do not
        necessarily reflect the view of the National Science
        Foundation.
      </div>
    </div>
    </div>
  </footer>
</template>

<style lang="scss" scoped>

/* Increase if wider menu items are added */
$user-menu-width: 10rem;

/* Increase if more menu items are added */
$user-menu-height: 2rem;

* {
  box-sizing: border-box;
}

#page {
    width: 100vw;
    display:flex;
    flex-direction:column;
    min-height:100vh;
}
#main {
  flex:1;
}

.full-only {
  display: none;
}

header {
    height: 52px;
    background-color: $snm-color-background-medlight;
    border-top: 2px solid $snm-color-background-dark;
    padding: 0 1rem;
    position: fixed;
    top:0;
    left:0;
    right:0;
    z-index: 100;

    .centered-row {
        display: flex;
        flex-direction: row;
        justify-content: space-evenly;
        margin-bottom: 0.75rem;
    }

    .button-icon {
        @include svg-fill;
        width: auto;
        height: 1.2em;
        vertical-align: middle;
    }

    .menu-icon {
        @include svg-fill;

        color: $snm-color-background-medium;
        width: auto;
        height: 1.2em;
        vertical-align: middle;
        margin: 0px 1rem;
        path {
          @include svg-fill;
        }
    }

    .no-icon {
        margin-left: 1rem;
    }

    .logo {
        height: 39px;
        width: auto;
        margin: 6px 0;

        img {
            width: 100%;
            height: 100%;
        }
    }

    .toggle-menu {
        background-color: transparent;
        border: 0px;
        width: rem(24px);
        height: rem(24px);
        padding:0;
    }

    .toggle-search {
        background-color: $snm-color-action;
        border: 1px solid $snm-color-action-border;
        border-radius: 5px;
        padding: 5px;
        height: 24px;
        box-shadow: 0px 3px 6px $snm-color-shadow;
        position: relative;
        top: -1px;
        white-space: nowrap;
        color: var(--primary-color, $snm-color-element-dark);
        display: flex;
        align-items: center;
        flex-shrink: 0;
        font-weight:bold;

        img {
            display: block;
            width: 14px;
            height: 14px;
            // margin-left: 6px;
        }
    }

    .menu-box {
        display: none;
        position: absolute;
        top:50px;
        left: 0px;
        right: 0px;
        z-index: 20;
        background-color: var(--primary-color, $snm-color-element-dark);
        text-align: left;
        box-shadow: 0px 3px 6px $snm-color-shadow;
        width: 100%;

        .not-authenticated {
            display: flex;
            justify-content: space-evenly;
            background-color: var(--secondary-color, $snm-color-element-med);
            padding: .75rem 0;

            button, a {
              position: relative;
              font-family: $snm-font-content;
              font-size: 0.85rem;
              border: 1px solid var(--border);
              border-radius: 6px;
              margin: 10px 0.5rem;
              padding: 10px;
              box-shadow: 0px 3px 6px $snm-color-shadow;
              font-weight: bold;
              color: var(--foreground);
              background-color: var(--background);
              flex-shrink: 0;
              flex-grow: 0;
              cursor: pointer;
              display: inline-flex;
              align-items: center;
              justify-content: center;
              padding: 0.75rem 1.5rem;
              box-sizing: border-box;
              height: rem(40px);
              line-height: 1;
                display: inline-flex;
                align-items: center;
                justify-content: center;
                width: calc(50% - 3rem);
                box-shadow: 0px 3px 6px $snm-color-shadow;
                margin:0;
                background-color: $snm-color-background-meddark;
                border: 1px solid var(--background-color, #fff);

                &:hover,&:active {
                    color: var(--secondary-color, $snm-color-element-med);
                    background-color: $snm-color-element-light;
                }

                &.primary {
                    --border: #{$snm-color-background-dark};
                    --background: #{$snm-color-background-meddark};
                    --foreground: #{$snm-color-element-light};

                    &:hover,&:active {
                        --background: #{$snm-color-background-dark};
                    }

                }
            }
        }

        .authenticated {
            li {
                border: 1px solid $snm-color-border-ondark;
                line-height: 48px;

                a {
                    color: $snm-color-element-ondark;

                    img {
                        color: $snm-color-background-medium;
                    }
                }
            }
        }
    }

    .search-box {
        display: none;
        position: absolute;
        top: 52px;
        left: 0px;
        right: 0px;
        box-shadow: 0px 3px 6px $snm-color-shadow;
        background-color: $snm-color-background;
        min-height: 4rem;
        z-index: 99;
        box-sizing: border-box;
        padding: 1rem;

        input[type="text"] {
          height: auto;
          padding-top: 0.5rem;
          padding-bottom: 0.5rem;
        }

        input[type="date"] {
            padding: 0.75rem;
            border-radius: 10px;
            border: 1px solid $snm-color-border;

        }

        .centered-row.flex-justify-sb {
          justify-content: space-between;

          .field {
            width: calc(50% - 0.5rem);
            input {
              width: 100%;
            }
          }
        }





    }

    .toggled {
        display: block;
        // max-width: 25rem;
        margin: 0px auto;
    }
}

#cookie-notice {
    position: fixed;
    bottom: 0;
    left: 0;
    z-index: 500;
    width: 100vw;
    min-width: 300px;
    max-height: 55vh;
    overflow-y: auto;
    overflow-x: hidden;
    background-color: $snm-color-background;
    border-radius: 0;
    border: 1px solid $snm-color-background-medium;
    padding: 1rem;
    box-shadow: 0px 0px 6px $snm-color-shadow;
    display: flex;
    flex-direction: column;

    h1 {
        font-family: $snm-font-heading;
        font-weight: bold;
        font-size: 1.25rem;
        margin:0;
    }

    p {
      margin:0 0 0.5rem 0;
    }

    h2 {
        font-family: $snm-font-heading;
        font-weight: bold;
        font-size: 1rem;
    }
    button {
      margin-bottom: 1rem;
    }

    >div {
        display: flex;
        flex-direction: column;
        align-items: flex-start;

        // >* {
        //     margin: 1rem;
        // }
    }
}

#main {
  padding-top: 52px;
}

.mobile-only .nav-separate {
    color: #7cb4bf;
    padding-left: 3px;
}

footer {
    background-color: $snm-color-background-dark;

    li {
        line-height: 1.2;
        padding-left: 1rem;
        border-bottom: 1px solid $snm-color-border-ondark;
        padding: 0.5rem 0;

        h1 {
            color: $snm-color-heading-ondark;
            font-family: $snm-font-heading;
            font-size: $snm-font-smaller;
            font-weight: bold;
            letter-spacing: 0.7px;
        }

        a {
            color: $snm-color-element-ondark;
            font-family: $snm-font-content;
            font-size: $snm-font-small;
            letter-spacing: 0px;
        }
    }

    .partner {
        display: flex;
        flex-direction: row;


        .nsf-logo {
            margin-right: 1rem;
            flex-shrink: 0;

            img {
                height: auto;
                width: 56px;
                vertical-align: top;
            }
        }

        .description {
            color: $snm-color-element-ondark;
            font-family: $snm-font-content;
            font-size: $snm-font-smaller;
            line-height: 22px;
            letter-spacing: 0px;
            margin: 16px 16px 16px 0px;
        }
    }
}

.subfooter {
  background-color: #1d3a40;

  li a {
    color: #F2C04B;
  }
}

#main {
    >nav,>.logo {
        display: none;
    }
}

@media only screen and (min-width: $fullsize-screen) {
    .full-only {
      display: block;
    }

    #main {
      padding-top: 0;
    }

    #cookie-notice {
      position: fixed;
      bottom: 1rem;
      left: 1rem;
      z-index: 500;
      width: 75vw;
      min-width: 300px;
      max-height: 50vh;
      overflow-y: auto;
      overflow-x: hidden;
      background-color: $snm-color-background;
      border-radius: 1rem;
      border: 1px solid $snm-color-background-medium;
      padding: 2rem;
      box-shadow: 0px 0px 6px $snm-color-shadow;
        flex-direction: row;
    }

  header .logo {
        width: 180px;
        height: auto;
        margin: 1rem;
        margin-top: 2rem;
        margin-bottom: -1.2rem;
    }

    header {
        height: 90px;
        border-top: 10px solid $snm-color-prehead;
        position: relative;
        overflow:visible;

        header a {
          display: block;
        }

        .toggle-menu {
          display: none;
        }



        .toggle-search {
            position: relative;
            // top: 30px;
            // right: 10px;
            order: 3;
            padding: 10px;
            min-width: 40px;
            height: 40px;
            margin-right: 1rem;

            &:hover,&:active {
              background-color: #F2C04B;
            }

            img {
                width: 20px;
                height: 20px;
                margin-right: 6px;
            }
        }

        .menu-box {
            display: inline-block;
            background-color: transparent;
            box-shadow: none;
            position:relative;
            left: unset;
            right: unset;
            top: unset;
            padding:0;

            .not-authenticated {
                display: inline-block;
                background-color: transparent;
                padding: 0;

                button,a {
                    width: auto;
                    box-shadow: 0px 3px 6px $snm-color-shadow;
                    background-color: $snm-color-background-meddark;
                    border: $snm-color-background-dark;
                    margin-right: 1rem;

                    &:hover,&:active {
                        color: var(--secondary-color, $snm-color-element-med);
                        background-color: $snm-color-element-light;
                    }
                }
            }

            .authenticated {
                display: flex;
                flex-wrap: wrap;
                align-items: center;
                justify-content: right;
                margin-right: 1rem;
                height: 40px;
                border-radius: 6px;
                background-color: $snm-color-background-meddark;
                color: $snm-color-element-light;
                box-sizing: border-box;
                border: 1px solid $snm-color-background-dark;
                transition: border-bottom-right-radius 0.5s, botder-bottom-left-radius 0.5s;
                padding: 0px 1rem;
                min-width: $user-menu-width;
                position: relative;

                > span {
                    font-family: $snm-font-content;
                    font-weight: bold;
                    font-size: $snm-font-small;
                    cursor: pointer;

                    &::after {
                        content: url(~assets/img/down-arrow.svg);
                        display: inline-flex;
                        align-items: center;
                        justify-content: center;
                        margin-left: 1rem;
                        width: 1.5em;
                        height: 1.5em;
                        border-radius: 1em;
                        color: var(--primary-color, $snm-color-element-dark);
                        background-color: $snm-color-element-light;
                        transform: rotate(0);
                        transition: transform;
                    }
                }

                > ul {
                    position: absolute;
                    top: 40px;
                    right: 0;
                    width: $user-menu-width;
                    overflow: hidden;
                    max-height: 0px;
                    max-width: 0px;
                    transition: max-height 0.5s, max-width 0.5s, right 0.5s;
                    background-color: $snm-color-background-meddark;
                    box-sizing: border-box;
                    border: 1px solid $snm-color-background-dark;
                    border-radius: 6px;
                    border-top-left-radius: 0px;
                    border-top-right-radius: 0px;
                    opacity: 0;

                    li {
                        border: 0px;
                        line-height: 24px;
                        display: flex;
                        align-items: center;
                        justify-content: right;
                        margin: 3px 1rem;

                        a {
                            color: $snm-color-element-ondark;
                        }
                    }
                }
            }

            &.toggled .authenticated {
                border-bottom-left-radius: 0px;
                border-bottom-right-radius: 0px;
                border-bottom: 0px;

                > span {
                    &::after {
                        transform: rotate(-0.5turn);
                        transition: transform;
                    }
                }

                ul {
                    max-width: $user-menu-width;
                    max-height: $user-menu-height;
                    top: 40px;
                    right: -1px;
                    opacity: 1;
                    transition: max-height 0.5s, max-width 0.5s, right 0.5s, opacity 0.5s;
                }
            }
        }
    }

    .authenticated #main {


        nav {
            // margin-top: 20px;
            display: flex;
            flex-direction: column;

            // clip-path: inset(0px -2rem 0px 0px);

            strong {
                font-family: $snm-font-content;
                font-weight: bold;
                font-size: $snm-font-smallest;
                line-height: 15px;
                letter-spacing: 0.39px;
                text-transform: uppercase;
                padding: 2rem 0px 0px 1rem;

                &.nav-separate {
                    margin-top: 2rem;
                    border-top: 1px solid var(--background-color, $snm-color-background-light);
                }
            }

            a {
                display: flex;
                color: var(--primary-color, $snm-color-element-dark);
                background-color: transparent;
                align-items: center;
                font-family: $snm-font-content;
                font-weight: normal;
                font-size: $snm-font-small;
                line-height: 0.9rem;
                padding-left: 1rem;
                position: relative;
                box-sizing: border-box;
                height: 48px;
                margin-top: 0;
                padding-right: 1rem;

                &.nuxt-link-active, &.nuxt-link-active:active {
                    background-color: $snm-color-background-meddark;
                    color: $snm-color-element-light!important;

                    // &::after {
                    //     position: absolute;
                    //     right: -26px;
                    //     content: "";
                    //     width: 0px;
                    //     height: 52px;
                    //     border-left: 26px solid $snm-color-background-meddark;
                    //     border-top: 26px solid transparent;
                    //     border-bottom: 26px solid transparent;
                    // }
                }

                svg {
                    margin-right: 1rem;
                    min-width: 20px;

                    * {
                        fill: currentColor;
                    }
                }
            }
            a:hover,a:active {
              color: $snm-color-background-meddark;
            }
        }
    }

    footer > .snm-container {
        display: flex;
        justify-content: space-between;
        margin: 0 auto;

        > ul {
          width: calc(25% - 0.5rem);

            li {
                border: 0px;

                h1 {
                    text-decoration: underline;
                }
            }
        }

        .partner {
          width: calc(50% - 0.5rem);
        }


    }


}

.not-authenticated #authenticated-nav, .authenticated #authenticated-nav {
  display: none;
}

@media (min-width: $fullsize-screen) {
  .authenticated {
    header {
      justify-content: flex-end!important;
      height: 80px;
      position: fixed;
    }

    #content {
      padding-top:80px;
    }

    header .logo {
      display: none!important;
    }

    .logo {
      width: 180px;
      margin: 0 auto;
      display: block;
      margin-top: 1rem;
      img {
        width: 180px;
      }
    }

  }

  .authenticated #authenticated-nav {
    display: block;
    position: fixed;
    top: 0;
    left: 0;
    bottom: 0;
    width: 200px;
    background-color: $snm-color-background-medium;
    box-shadow: 0px 0px 6px $snm-color-shadow;
    z-index:101;

    .an-overflow {
      overflow-x: visible;
      height: calc(100vh - 105px);
      scrollbar-width: none;
      overflow-y: auto;
      &::-webkit-scrollbar {
        display: none; /* for Chrome, Safari, and Opera */
      }
    }

    nav {
      margin-bottom: 2rem;
    }

  }



  .authenticated #content {
    margin-left: 200px;
  }

  .authenticated footer {
    padding-left: 200px;
    .snm-container {
      padding: 1rem;
    }
  }
  .authenticated .subfooter {
    padding-left: 200px;
  }

  .not-authenticated  .just-content {
    padding-top: 2rem;
  }

  .not-authenticated footer {
    padding-top:2rem;
  }

  .not-authenticated footer.homepage {
    padding-top:5rem;
  }

  .subfooter ul {
    display: flex;

    li {
      margin-right: 1rem;
    }
    li:not(:first-child)::before {
      content: '|';
      color: #087a91;
      margin-right: 8px;
    }
  }
  .authenticated .subfooter ul {
    padding-left: 2rem;
  }

}

@media (min-width: 1200px) {
  .authenticated #authenticated-nav {
    width: 280px;
  }
  .authenticated #content {
    margin-left: 280px;
  }

  .authenticated footer, .authenticated .subfooter {
    padding-left: 280px;
  }
}


.modal .card {
    margin: 1rem;
    padding: 1rem;
    max-height: 100%;
    overflow: auto;
}

@media (max-width: $mobile-screen) {
    .no-mobile {
        display: none !important;
    }
}

@media (min-width: $tablet-screen) {
  .no-tablet {
    display: none;
  }
}

@media (min-width: $fullsize-screen) {
    .mobile-only {
        display: none !important;
    }
    .no-tablet {
      display: block;
    }
}

.bubble {
    position: relative;
    top: -0.25em;
    display: inline-flex;
    font-size: 8pt;
    color: $snm-color-element-light;
    background-color: $snm-color-info;
    border-radius: 50%;
    align-items: center;
    justify-content: center;
    min-width: 1rem;
    height: 1rem;
    line-height: 1rem;
    margin-left: 0.5em;
}

@media (min-width:$fullsize-screen) {
  header .search-box.toggled {
    background-color: rgba(0,0,0,0.8);
    position: fixed;
    top:0;
    left: 0;
    right: 0;
    bottom: 0;
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;

    .search-box-container {
      background-color: var(--background-color, #fff);
      padding: 1rem;
      border-radius: 6px;
    }

    .sbc-header {
      display: flex;
      color: $snm-color-background-meddark;

      h2 {
        font-size: 1.4rem;
        font-weight: bold;
      }
      button {
        border:0;
        background-color: transparent;
        font-size: 1.8rem;
        font-weight: bold;
      }
    }

  }

}

.beta-banner {
  text-align: center;
  padding: 0.5rem!important;
  border-bottom: 1px solid #efefef;
  img {
    height: 20px;
    vertical-align:middle;
    position:relative;
    top: -2px;
    left: -2px;
  }
}
.authenticated .beta-banner, .not-authenticated .beta-banner {
  margin-top: 52px;
  margin-bottom: -52px;
}

@media (min-width:$fullsize-screen) {
  .not-authenticated .beta-banner {
    margin-top: 0;
    margin-bottom: 0;
  }
  .authenticated .beta-banner {
    margin-top: 80px;
    padding-left: 200px!important;
    margin-bottom: -80px;
  }
}



@media (min-width:1200px) {
  .authenticated .beta-banner {
    padding-left: 280px!important;
  }
}

.my-submit-opportunity, .my-edit-opportunity{
  footer, .subfooter {
    display:none;
  }
}

</style>

<style type="scss">
  .b-tabs .tab-content {
    padding:0;
  }
  .modal {
    z-index:999!important;

  }
  .modal .animation-content {
    overflow: visible;
  }
  @media (min-width:768px){
    .modal .animation-content {
      min-width: 600px;
    }
  }
</style>