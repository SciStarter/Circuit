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
    <div v-for="goal in goals" :key="goal.id">
      {{ duration(goal.begin, goal.end) }}
      {{ goal }}
    </div>
  </section>
</template>

<script>
import humanizeDuration from 'humanize-duration'
import ActionButton from '~/components/ActionButton'

import AtomIcon from '~/assets/img/atom.svg?inline'
import TrophyDabbler from '~/assets/img/trophy-dabbler.svg?inline'
import TrophyEnthusiast from '~/assets/img/trophy-enthusiast.svg?inline'
import TrophyHero from '~/assets/img/trophy-hero.svg?inline'

export default {
    components: {
        AtomIcon,
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

    methods: {
        duration(begin, end) {
            return humanizeDuration(new Date(end) - new Date(begin), { largest: 2, round: true });
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
                progress: 0,
                target: times,
                begin,
                end,
                status: "working",
            };

            await this.$axios.$post("/api/ui/profile/goals", goal, this.$store.state.auth);
            await this.fetch_goals();
        },

        async cancel_goal(id) {
            await this.$axios.$delete("/api/ui/profile/goals/" + id, this.$store.state.auth);
            await this.fetch_goals();
        },
    },
}
</script>

<style lang="scss" scoped>
.set-goal {
    > h1 {
        display: flex;
        justify-content: center;
        align-items: center;
        font-family: $snm-font-heading;
        font-weight: bold;
        font-size: $snm-font-small;
        line-height: 36px;
        color: $snm-color-background-dark;
        margin-top: 1rem;

        svg {
            position: absolute;
            left: calc(50% - 5em);
            height: 1.5em;
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

@media (min-width: $fullsize-screen) {
    .set-goal {
        display: flex;
        flex-wrap: wrap;
        justify-content: center;

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

        > p {
            width: 100%;
        }

        > .goal-card {
            min-width: 300px;
            max-width: 600px;
            width: calc(33% - 2rem);
        }
    }
}
</style>
