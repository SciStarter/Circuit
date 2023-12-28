<template>
    <div id="additional-filters">
        <div>
            <b-field>
                <b-select placeholder="Format" v-model="selected_format">
                    <option
                        v-for="option in formats"
                        :value="option.format"
                        :key="option.format">
                        {{ option.title }}
                    </option>
                </b-select>
            </b-field>

            <b-field>
                <b-select v-model="selected_date" @input="handleDate" placeholder="Date">
                    <option
                        v-for="option in periods"
                        :value="option.period"
                        :key="option.period">
                        {{ option.title }}
                    </option>
                </b-select>
            </b-field>
                <b-button type="button" @click="$emit('togglefilters',true)"><filter-icon /> More Filters</b-button>
            <!-- if filters applied -->
            <button type="button"  @click="$emit('clearfilters',true)" style="height:40px;border:1px solid #b5b5b5;border-radius:4px;"><span>&times;</span> Clear Filters</button>
        </div>
        <div>
        </div>



        <b-modal v-model="show_datepicker" aria-role="dialog" aria-label="Select a custom date range" aria-modal>
            <div class="card">
                <header class="modal-card-head">
                    <p class="modal-card-title">Select a date range</p>
                    <button
                        type="button"
                        class="delete"
                        @click="$emit('close')"/>
                </header>
                <section class="modal-card-body">
                    <b-field>
                        <b-datepicker
                            inline
                            placeholder="Click to select..."
                            :min-date="minDate"
                            :max-date="maxDate">
                        </b-datepicker>
                    </b-field>
                </section>
                 <footer class="modal-card-foot">
                    <b-button
                        label="Close"
                        @click="$emit('close')" />
                    <b-button
                        label="Apply Dates"
                        type="is-primary" />
                </footer>
            </div>
        </b-modal>


    </div>
</template>

<script>
import FilterIcon from '~/assets/img/filter.svg?inline'
export default {
    name: "AdditionalFilters",
    components: {
        FilterIcon
    },
    data() {
        const today = new Date()
        return {
            show_datepicker: false,
            minDate: new Date(today.getFullYear() - 80, today.getMonth(), today.getDate()),
            maxDate: new Date(today.getFullYear() + 18, today.getMonth(), today.getDate()),
            formats: [
                {
                    format: "event_in_person",
                    title: "Live, In-Person Event"
                },
                {
                    format: "virtual",
                    title: "Virtual Events"
                },
                {
                    format: "region_citsci",
                    title: "Science Activity in My Area"
                },
                {
                    format: "on_demand",
                    title: "On Demand Science Activities"
                },
                {
                    format: "alll",
                    title: "All Science Opportunities"
                },
            ],
            selected_format: "event_in_person",
            periods: [
                {
                    period: "today",
                    title: "Today"
                },
                {
                    period: "tomorrow",
                    title: "Tomorrow"
                },
                {
                    period: "this_week",
                    title: "This Week"
                },
                {
                    period: "next_week",
                    title: "Next Week"
                },
                {
                    period: "this_month",
                    title: "This Month"
                },
                {
                    period: "next_month",
                    title: "Next Month"
                },
                {
                    period: "custom",
                    title: "Custom"
                }
            ],
            selected_date: null
        };
    },
    methods: {
        handleDate(v) {
            if (v == 'custom') {
                this.show_datepicker = true;
            }
        }
    }
}

</script>

<style lang="scss" scoped>
#additional-filters {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 1rem;

    > div {
        display: flex;
        gap: 1rem;
        flex-wrap: wrap;

        
    }
}

:deep(.modal .modal-content){
    width: auto;
    overflow: auto;
}

@media (max-width: 768px){
    :deep(.modal .modal-content){
        max-width: 960px;
        padding: 0;
        max-height: 100vh;
        margin: 0;
    }
}

@media (max-width:615px){
    #additional-filters  > div > div {
            max-width: 33%;
        }
}

@media (max-width:500px){

    #additional-filters {
        padding: 1rem;
        margin-top: -1rem;
    }

    #additional-filters > div {
        gap: 0.75rem 1rem;
    }
    #additional-filters  > div > .field {
            max-width: calc(50% - 0.5rem);
            width: 50%;
            margin-bottom: .1rem;

             :deep(.select) {
                width:100%;
            }

            :deep(select) {
                width:100%;
            }
        }
}

button {
    // font-size: .75rem;
    // padding: .25rem .5rem;
    // border:1px solid #d4d4d4;
    // border-radius: 6px;
    // margin-right: 8px;
    // cursor: pointer;

    svg {
      width: 16px;
    vertical-align: middle;
    height: 16px;
    position: relative;
    top: -1px;
    margin:-2px 0;
    }
  }
</style>