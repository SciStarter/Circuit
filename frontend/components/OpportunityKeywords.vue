<template>
<div v-if="keywords.length > 0" class="opportunity-keywords">
  <span v-for="kw in keywords" :key="kw">{{ kw }}</span>
</div>
<div v-else class="opportunity-keywords">
  <em>no keywords assigned</em>
</div>
</template>

<script>
const DESCRIPTOR_NAMES = {
    "advocacy_days": "Advocacy Days",
    "Bioblitz": "Bioblitz",
    "camp": "Camp",
    "citizen_science": "Citizen Science",
    "clean_up": "Clean Up",
    "club": "Club",
    "community": "Community",
    "competition": "Competition",
    "concert": "Concert",
    "conference": "Conference",
    "create-a-thon": "Create-a-Thon",
    "dance": "Dance",
    "exhibition": "Exhibition",
    "expo_style": "Expo Style",
    "festival": "Festival",
    "forum": "Forum",
    "fundraising": "Fundraising",
    "hack-a-thon": "Hack-a-Thon",
    "lecture": "Lecture",
    "live_science": "Live Science",
    "make-a-thon": "Make-a-Thon",
    "maker": "Maker",
    "maker_faire": "Maker Faire",
    "media": "Media",
    "outreach": "Outreach",
    "overnight": "Overnight",
    "panel": "Panel",
    "policy": "Policy",
    "professional_development": "Professional Development",
    "research": "Research",
    "science_blogging": "Science_Blogging",
    "science_cafe_or_pub": "Science Cafe or Pub",
    "science_on_tap": "Science On Tap",
    "science_poetry_slam": "Science Poetry Slam",
    "science_slam": "Science Slam",
    "service": "Service",
    "star_party": "Star Party",
    "story_collider": "Story Collider",
    "tinker": "Tinker",
    "tinker_faire": "Tinker Faire",
    "training": "Training",
    "volunteering": "Volunteering",
    "workshop": "Workshop",
};

const TOPIC_NAMES = {
    "agriculture": "Agriculture",
    "alcohol": "Alcohol",
    "animals": "Animals",
    "archaeology_and_cultural": "Archaeology & Cultural",
    "art": "Art",
    "astronomy_and_space": "Astronomy & Space",
    "awards": "Awards",
    "biology": "Biology",
    "birds": "Birds",
    "chemistry": "Chemistry",
    "climate_and_weather": "Climate & Weather",
    "computers_and_technology": "Computers & Technology",
    "crowd_funding": "Crowd Funding",
    "design": "Design",
    "disaster_response": "Disaster Response",
    "ecology_and_environment": "Ecology & Environment",
    "education": "Education",
    "engineering": "Engineering",
    "food": "Food",
    "general_science": "General Science",
    "geography": "Geography",
    "geology_and_earth_science": "Geology & Earth Science",
    "health_and_medicine": "Health & Medicine",
    "insects_and_pollinators": "Insects & Pollinators",
    "mathematics": "Mathematics",
    "nature_and_outdoors": "Nature & Outdoors",
    "ocean_water_marine": "Ocean Water Marine",
    "paleontology": "Paleontology",
    "physics": "Physics",
    "policy": "Policy",
    "psychology": "Psychology",
    "religion": "Religion",
    "robotics": "Robotics",
    "social_science": "Social Science",
    "sound": "Sound",
    "technology": "Technology",
    "transportation": "Transportation",
};

export default {
    props: {
        opportunity: {
            type: Object,
            required: true
        }
    },

    computed: {
        keywords() {
            let ret = [];
            let seen = {};

            if(this.opportunity.opp_descriptor) {
                for(let desc of this.opportunity.opp_descriptor) {
                    if(desc && !seen[desc]) {
                        seen[desc] = true;
                        ret.push(DESCRIPTOR_NAMES[desc] || desc);
                    }
                }
            }

            if(this.opportunity.opp_topics) {
                for(let topic of this.opportunity.opp_topics) {
                    if(topic && !seen[topic]) {
                        seen[topic] = true;
                        ret.push(TOPIC_NAMES[topic] || topic);
                    }
                }
            }

            return ret;
        }
    },
}
</script>

<style lang="scss" scoped>
span:not(:first-of-type)::before {
    content: ", ";
}
</style>
