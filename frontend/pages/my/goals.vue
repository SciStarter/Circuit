<template>
  <section v-if="mode == 'set'" class="goals set-goal" data-context="set-goal">
    <h1><atom-icon /> Set a goal</h1>
    <p>Setting a goal is a great way to challenge yourself to grow your science learning!</p>
    <div class="goal-card" data-context="goal-dabbler">
      <trophy-dabbler />
      <div>
        <strong>Science Dabbler</strong>
        <p>
          Take part in 5 science opportunities in one year
        </p>
        <action-button principal arrow @click="set_goal(5, 366, 'dabbler')">
          Set Goal
        </action-button>
      </div>
    </div>
    <div class="goal-card" data-context="goal-enthusiast">
      <trophy-enthusiast />
      <div>
        <strong>Science Enthusiast</strong>
        <p>
          Take part in 20 science opportunities in one year
        </p>
        <action-button principal arrow @click="set_goal(20, 366, 'enthusiast')">
          Set Goal
        </action-button>
      </div>
    </div>
    <div class="goal-card" data-context="goal-hero">
      <trophy-hero />
      <div>
        <strong>Science Hero</strong>
        <p>
          Take part in 50 science opportunities in one year
        </p>
        <action-button principal arrow @click="set_goal(50, 366, 'hero')">
          Set Goal
        </action-button>
      </div>
    </div>
  </section>
  <section v-else class="goals current-goal" data-context="current-goal">
    <div v-for="goal in succeeded" class="congrats">
      <atom-icon class="hydrogen" />
      <atom-icon class="helium" />
      <atom-icon class="lithium" />
      <span class="congratulations">Congratulations!</span>
      <trophy-dabbler v-if="goal.category == 'dabbler'" class="trophy" />
      <trophy-enthusiast v-if="goal.category == 'enthusiast'" class="trophy" />
      <trophy-hero v-if="goal.category == 'hero'" class="trophy" />
      <span class="you-are">You're a</span>
      <span class="category">science {{ goal.category }}</span>
      <span class="explain">Take part in {{ goal.target }} science opportunities in {{ duration(goal.begin, goal.end) }}!</span>
      <span class="share-prompt">Share your success!</span>
      <div class="share">
        <social-button mode="facebook" url="https://sciencenearme.org/" :title="'I\'m a science ' + goal.category + '!'" :hashtags="['#ScienceNearMe', '#science' + goal.category]" />
        <social-button mode="twitter" url="https://sciencenearme.org/" :title="'I\'m a science ' + goal.category + '!'" :hashtags="['#ScienceNearMe', '#science' + goal.category]" />
        <social-button mode="linkedin" url="https://sciencenearme.org/" :title="'I\'m a science ' + goal.category + '!'" :hashtags="['#ScienceNearMe', '#science' + goal.category]" />
      </div>
      <action-button principal arrow @click="start_new(goal)">
        Set a New Goal
      </action-button>
    </div>
    <div v-for="goal in failed" class="sorry">
      <strong>
        Shucks! You didn’t meet your goal. But it’s OK!
      </strong>
      <p>
        You can end this goal and try again. We believe in you! No
        matter what, when science learning is your goal, you’re a
        winner!
      </p>
      <action-button principal arrow @click="cancel_goal(goal)">
        Set a New Goal
      </action-button>
    </div>
    <h1><atom-icon /> Current Goal</h1>
    <div v-for="goal in goals" :key="goal.id" class="goal">
      <button v-if="goal.progress.length < goal.target" class="trash" title="cancel goal" @click="cancel_goal(goal)">
        <trash-icon />
      </button>
      <h2>Science {{ goal.category }}</h2>
      <strong>Attend <em>{{ goal.target }} science opportunities</em> in <em>{{ duration(goal.begin, goal.end) }}</em></strong>
      <div class="dates">
        <span>Start Date: {{ new Date(goal.begin).toLocaleDateString() }}</span>
        <span>End Date: {{ new Date(goal.end).toLocaleDateString() }}</span>
      </div>
      <div class="progression">
        <progress-bar :value="goal.progress.length" :min="0" :max="goal.target" label="Science Opportunities" />
        <progress-bar :value="Math.round(days(goal.begin, new Date()))" :min="0" :max="Math.round(days(goal.begin, goal.end))" label="Time Left" :alert-threshold="338" units="days" countdown />
      </div>
      <div class="itemized">
        <ol>
          <li v-for="opp in goal.progress">
            <nuxt-link :to="'/' + opp.opportunity.slug">
              {{ opp.opportunity.title }}
            </nuxt-link>
          </li>
        </ol>
      </div>
    </div>
  </section>
</template>

<script>
import humanizeDuration from 'humanize-duration'
import ActionButton from '~/components/ActionButton'
import SocialButton from '~/components/SocialButton'

import AtomIcon from '~/assets/img/atom.svg?inline'
import TrashIcon from '~/assets/img/trash.svg?inline'
import TrophyDabbler from '~/assets/img/trophy-dabbler.svg?inline'
import TrophyEnthusiast from '~/assets/img/trophy-enthusiast.svg?inline'
import TrophyHero from '~/assets/img/trophy-hero.svg?inline'

export default {
    name: "MyGoals",

    components: {
        ActionButton,
        SocialButton,

        AtomIcon,
        TrashIcon,
        TrophyDabbler,
        TrophyEnthusiast,
        TrophyHero,
    },

    async asyncData(context) {
        const goals = await context.$axios.$get("/api/ui/profile/goals", context.store.state.auth);

        return {
            goals,
            mode: goals.length ? 'current' : 'set',
        };
    },

    computed: {
        succeeded() {
            return this.goals.filter(goal => { return goal.progress.length >= goal.target; });
        },

        failed() {
            return this.goals.filter(goal => { return (goal.progress.length < goal.target) && (new Date(goal.end) < new Date()); })
        },
    },

    methods: {
        days(begin, end) {
            if(begin.constructor !== Date) {
                begin = new Date(begin);
            }

            if(end.constructor !== Date) {
                end = new Date(end);
            }

            return (end - begin) / (24 * 60 * 60 * 1000);
        },

        duration(begin, end) {
            if(begin.constructor !== Date) {
                begin = new Date(begin);
            }

            if(end.constructor !== Date) {
                end = new Date(end);
            }

            return humanizeDuration(end - begin, { largest: 2, round: true });
        },

        async fetch_goals() {
            this.goals = await this.$axios.$get("/api/ui/profile/goals", this.$store.state.auth);
            this.mode = this.goals.length ? 'current' : 'set';
        },

        async set_goal(times, days, label) {
            let dt = new Date();
            const begin = dt.toISOString();
            dt.setDate(dt.getDate() + days);
            const end = dt.toISOString();

            const goal = {
                category: label,
                target: times,
                begin,
                end,
                status: "working",
            };

            await this.$axios.$post("/api/ui/profile/goals", goal, this.$store.state.auth);
            await this.fetch_goals();
        },

        async cancel_goal(goal) {
            if(new Date(goal.end) < new Date()) {
                const updated = {
                    ...goal,
                    status: 'failed',
                };
                await this.$axios.$put("/api/ui/profile/goals/" + goal.id, updated, this.$store.state.auth);
            }
            else {
                await this.$axios.$delete("/api/ui/profile/goals/" + goal.id, this.$store.state.auth);
            }
            await this.fetch_goals();
        },

        async start_new(goal) {
            const updated = {
                ...goal,
                status: 'succeeded',
            };
            await this.$axios.$put("/api/ui/profile/goals/" + goal.id, updated, this.$store.state.auth);
            await this.fetch_goals();
        },
    },
}
</script>

<style lang="scss" scoped>
.goals {
    > h1 {
        display: flex;
        align-items: center;
        font-family: $snm-font-heading;
        font-weight: bold;
        font-size: $snm-font-small;
        line-height: 36px;
        color: $snm-color-background-dark;
        margin-top: 1rem;

        svg {
            height: 1.5em;
        }
    }
}

.set-goal {
    > h1 {
        justify-content: center;

        svg {
            position: absolute;
            left: calc(50% - 5em);
        }
    }

    > p {
        padding: 0.5rem;
        text-align: center;
        padding-bottom: 2rem;
        font-family: $snm-font-meta;
        font-size: $snm-font-small;
        color: $snm-color-tldr;
    }

    > .goal-card {
        display: flex;
        flex-direction: row;
        border: 1px solid $snm-color-border;
        border-radius: 10px;
        margin: 1rem;
        padding: 1rem;

        > svg {
            margin-right: 2rem;
        }

        > div {
            display: flex;
            flex-direction: column;

            > strong {
                font-family: $snm-font-heading;
                font-weight: bold;
                font-size: $snm-font-medium-small;
                color: $snm-color-background-dark;
                line-height: 22px;
            }

            > p {
                font-family: $snm-font-content;
                font-size: $snm-font-small;
                line-height: 19px;
                color: $snm-color-tldr;
            }
        }
    }
}

.current-goal {
    > div.goal {
        position: relative;
        padding: 0.75rem;

        .trash {
            position: absolute;
            top: 0rem;
            right: 1rem;

            display: flex;
            align-items: center;
            color: $snm-color-element-med;
            border: 1px solid $snm-color-element-med;
            border-radius: 10px;
            box-shadow: 0px 3px 6px $snm-color-shadow;
            height: 40px;
            padding: 0px 1em;
            cursor: pointer;

            svg {
                * {
                    fill: currentColor;
                }
            }
        }

        > h2 {
            font-family: $snm-font-heading;
            font-weight: bold;
            font-size: $snm-font-larger;
            text-transform: capitalize;
            color: $snm-color-element-dark;
        }

        > strong {
            display: block;
            font-family: $snm-font-heading;
            font-weight: bold;
            font-size: $snm-font-small;
            color: $snm-color-element-dark;

            > em {
                font-family: $snm-font-heading;
                font-weight: bold;
                font-size: $snm-font-small;
                font-style: normal;
                color: $snm-color-element-med;
            }
        }

        > .dates {
            display: flex;
            flex-direction: column;
            padding-bottom: 0.75rem;
            border-bottom: 1px solid $snm-color-border;
            margin-bottom: 0.75rem;

            > span {
                font-family: $snm-font-content;
                font-size: $snm-font-small;
                color: $snm-color-tldr;
                margin: 0.25rem 1rem 0.25rem 0px;
            }
        }

        > .progression {
            .progress-bar {
                margin: 1.25rem 0px;
            }
        }

        > .itemized {
            ol {
                list-style: none;
                counter-reset: index;

                li {
                    font-family: $snm-font-heading;
                    font-weight: bold;
                    font-size: $snm-font-small;
                    text-decoration: underline;
                    color: $snm-color-element-med;
                    margin-left: 3rem;
                    position: relative;
                    counter-increment: index;
                    margin-bottom: 1rem;
                }

                li::before {
                    position: absolute;
                    top: 0.2rem;
                    left: -3rem;
                    width: 1.25rem;
                    height: 1.25rem;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    font-family: $snm-font-content;
                    font-weight: bold;
                    font-size: $snm-font-smallest;
                    color: $snm-color-element-ondark;
                    content: counter(index);
                    background-color: $snm-color-heading-ondark;
                    border-radius: 50%;
                }

                li:not(:last-child)::after {
                    position: absolute;
                    top: 1.3rem;
                    left: -2.5rem;
                    content: "";
                    display: block;
                    width: 0.2rem;
                    height: 1.6rem;
                    background-color: $snm-color-heading-ondark;
                }
            }
        }
    }
}

.sorry {
    display: flex;
    flex-direction: column;
    background-color: $snm-color-note;
    border: 1px solid $snm-color-action;
    border-radius: 10px;
    margin: 1rem;
    padding: 1rem;
    align-items: flex-start;

    strong {
        font-family: $snm-font-heading;
        font-weight: bold;
        font-size: $snm-font-medium;
        color: $snm-color-background-dark;
    }

    p {
        font-family: $snm-font-content;
        font-size: $snm-font-small;
        color: $snm-color-element-dark;
    }
}

.congrats {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 0.5rem;
    border: 1px solid $snm-color-border;
    border-radius: 10px;
    margin: 0.5rem;

    .hydrogen {
        display: block;
        position: absolute;
        left: 1rem;
        top: -0.4rem;
    }

    .helium,.lithium {
        display: none;
    }

    .congratulations {
        font-family: $snm-font-heading;
        font-weight: bold;
        font-size: $snm-font-largest;
        color: $snm-color-background-dark;
    }

    .trophy {
        max-width: 50vw;
        height: 7rem;
        margin: 1rem 0px;
    }

    .you-are {
        font-family: $snm-font-content;
        font-weight: bold;
        font-size: $snm-font-medium-small;
        color: $snm-color-element-dark;
    }

    .category {
        font-family: $snm-font-heading;
        font-weight: bold;
        font-size: $snm-font-large;
        text-transform: uppercase;
        color: $snm-color-element-med;
    }

    .explain {
        font-family: $snm-font-content;
        font-size: $snm-font-small;
        color: $snm-color-element-dark;
        text-align: center;
        margin-bottom: 2rem;
    }

    .share-prompt {
        font-family: $snm-font-content;
        font-weight: bold;
        font-size: $snm-font-small;
        color: $snm-color-element-dark;
    }

    .share {
        display: flex;
        justify-content: space-around;
        width: 300px;
        margin: 1rem 0px 2rem;
    }
}

@media (min-width: $fullsize-screen) {
    .goals {
        > h1 {
            font-size: $snm-font-largest;
            width: 100%;

            svg {
                position: relative;
                top: 0px;
                left: -1rem;
                height: 2.5em;
            }
        }
    }

    .set-goal {
        display: flex;
        flex-wrap: wrap;
        justify-content: center;

        > p {
            width: 100%;
        }

        > .goal-card {
            min-width: 300px;
            max-width: 600px;
            width: calc(33% - 2rem);
        }
    }

    .current-goal {
        > div.goal {
            > .dates {
                flex-direction: row;
            }

            > .progression {
                margin-bottom: 2rem;
            }
        }
    }

    .congrats {
        .helium {
            display: block;
            position: absolute;
            right: 2rem;
            top: 8rem;
        }

        .lithium {
            display: block;
            position: absolute;
            left: 1rem;
            bottom: 2rem;
        }

        .trophy {
            max-width: 30vw;
            height: 15rem;
            margin: 5rem;
        }
    }
}
</style>
