<template>
<div id="opportunity-form">

  <div class="snm-container">

    <ol :class="{'nav-tabs': editMode, 'track': !editMode}">
      <li><a :class="{'tab-link':editMode, 'active':state==1}" @click="(editMode) => state=1"><span v-if="!editMode">1</span>Basic Information</a></li>
      <li><a :class="{'tab-link':editMode, 'active':state==2}" @click="(editMode) => state=2"><span v-if="!editMode">2</span>Required Fields</a></li>
      <li><a :class="{'tab-link':editMode, 'active':state==3}" @click="(editMode) => state=3"><span v-if="!editMode">3</span>Additional Fields</a></li>
    </ol>

  </div>

  <form class="snm-container">

    <div class="opp-form-wrapper">

      <div v-if="state==1">
        <div class="legend-flex">
          <legend>Basic Information</legend>
          <div class="required">* required</div>
        </div>
        <b-field message="64 character maximum">
          <template #label>
            Name of Opportunity<span class="required">*</span>
          </template>
          <b-input v-model="opportunity.name" has-counter maxlength="64"></b-input>
        </b-field>
        <b-field message="This is the organization hosting the event, project, or attraction. This might be your organization, a chapter, or similar.">
          <template #label>
            Host Organization<span class="required">*</span>
          </template>
          <b-input v-model="opportunity.organization_name"></b-input>
        </b-field>
        <b-field :message="'This opportunity is on Science Near Me under the auspices of the selected Science Near Me partner.' + (editMode ? 'If this needs to change, you must contact Science Near me.' : '')">
          <template #label>
            Science Near Me partner<span class="required">*</span>
          </template>
          <b-input v-model="partner.name" disabled></b-input>
        </b-field>

        <label class="label">Where is your opportunity?<span class="required">*</span></label>

        <label class="button-radio" :class="{'open':location=='online','unselected':location=='physical' || location=='both'}">
          <input type="radio" v-model="location" name="location" value="online" />
          <div class="icon-flex">
            <div class="br-img">
              <website-icon />
            </div>
            <div class="br-text">
              <h1>Online Only</h1>
              <p>An external link where people can participate.</p>
            </div>
          </div>
          <transition name="slide">
            <div v-if="location=='online'" class="add">
              <b-field message="Must start with http:// or https://">
                <template #label>
                  External link To participate<span class="required">*</span>
                </template>
                <b-input type="url" v-model="opportunity.partner_opp_url"></b-input>
              </b-field>
            </div>
          </transition>
        </label>

        <label class="button-radio" :class="{'open':location=='physical','unselected':location=='online' || location=='both'}">
          <input type="radio" v-model="location" name="location" value="physical" />
          <div class="icon-flex">
            <div class="br-img">
              <location-icon />
            </div>
            <div class="br-text">
              <h1>Physical Location</h1>
              <p>An address or area.</p>
            </div>
          </div>
          <transition name="slide">
            <div v-if="location=='physical'" class="add">
              <b-field message="Begin typing and select location">
                <template #label>
                  Search for an address or location<span class="required">*</span>
                </template>
                <lookup-geometry v-model="opportunity.location_name" @polygon="location_poly" @point="location_point" @license="location_license" />
              </b-field>
            </div>
          </transition>
        </label>

        <label class="button-radio" :class="{'open':location=='both','unselected':location=='online' || location=='physical'}">
          <input type="radio" v-model="location" name="location" value="both" />
          <div class="icon-flex">
            <div class="br-img">
              <both-icon />
            </div>
            <div class="br-text">
              <h1>Physical Location &amp; Online</h1>
              <p>An address or area and an external link where people can participate.</p>
            </div>
          </div>
          <transition name="slide">
            <div v-if="location=='both'" class="add">
              <b-field message="Begin typing and select location">
                <template #label>
                  Search for an address or location<span class="required">*</span>
                </template>
                <!-- <b-autocomplete
                     :loading="place_loading"
                     :data="place_matches"
                     field="near"
                     :value="filters.near"
                     :name="'new-' + Math.random()"
                     :clearable="true"
                     placeholder="e.g. Iowa City, IA"
                     @typing="place_completions"
                     @select="place_select"
                     autocomplete="off"
                     label="Place"
                     /> -->
              </b-field>
              <b-field message="Must start with http:// or https://">
                <template #label>
                  External link To participate<span class="required">*</span>
                </template>
                <b-input type="url" v-model="opportunity.partner_opp_url"></b-input>
              </b-field>
            </div>
          </transition>
        </label>

        <label class="label push-down">When is your opportunity?<span class="required">*</span></label>

        <label class="button-radio" :class="{'open':when=='ongoing','unselected':when=='time'}">
          <input type="radio" v-model="when" name="when" value="ongoing" />
          <div class="icon-flex">
            <div class="br-img">
              <infinity-icon />
            </div>
            <div class="br-text">
              <h1>On Demand/Always Available</h1>
              <p>Ongoing with no set time periods. May have an end date.</p>
            </div>
          </div>
          <transition name="slide">
            <div v-if="when=='ongoing'" class="add">
              <div class="set-info" v-if="endDate">
                <div class="flex">
                  <h2>End Date</h2>
                  <div class="push-right">
                    <a class="action" @click="show_end_date=true"><edit-icon /></a>
                    <a class="action" @click="endDate=null"><close-icon /></a>
                  </div>
                </div>
                <p>{{endDate}}</p>
              </div>
              <action-button v-else primary tight @click="show_end_date=true">+ Add an end date</action-button>
            </div>
          </transition>
        </label>

        <label class="button-radio" :class="{'open':when=='time','unselected':when=='ongoing'}">
          <input type="radio" v-model="when" name="when" value="time" />
          <div class="icon-flex">
            <div class="br-img">
              <time-icon />
            </div>
            <div class="br-text">
              <h1>At a Set Time</h1>
              <p>Time periods and dates that may or may not recur.</p>
            </div>
          </div>
          <transition name="slide">
            <div v-if="when=='time'" class="add">
              <b-field>
                <template #label>
                  Select a time zone<span class="required">*</span>
                </template>
                <b-select placeholder="Select a time zone">
                  <!-- <option
                       v-for="option in data"
                       :value="option.id"
                       :key="option.id">
                       {{ option.user.first_name }}
                       </option> -->
                </b-select>
              </b-field>
              <div v-if="timePeriods.length == 0" class="times-flex">
                <div class="flex">
                  <b-field>
                    <template #label>
                      Starts on:<span class="required">*</span>
                    </template>
                    <b-datepicker
                      v-model="selected"
                      placeholder="Click to select..."
                      icon="calendar-today"
                      :icon-right="selected ? 'close-circle' : ''"
                      icon-right-clickable
                      @icon-right-click="clearDate"
                      trap-focus>
                    </b-datepicker>
                    <b-timepicker
                      placeholder="Click to select..."
                      icon="clock">
                    </b-timepicker>
                  </b-field>
                  <b-field>
                    <template #label>
                      Ends on:
                    </template>
                    <b-datepicker
                      v-model="selected"
                      placeholder="Click to select..."
                      icon="calendar-today"
                      :icon-right="selected ? 'close-circle' : ''"
                      icon-right-clickable
                      @icon-right-click="clearDate"
                      trap-focus>
                    </b-datepicker>
                    <b-timepicker
                      placeholder="Click to select..."
                      icon="clock">
                    </b-timepicker>
                  </b-field>
                </div>
              </div>

              <div v-if="timePeriods.length > 0" class="set-info" style="margin-bottom:1rem;">
                <div class="flex">
                  <h2>Time Periods</h2>
                  <div class="push-right">
                    <a class="action" @click="show_time_periods=true"><edit-icon /></a>
                  </div>
                </div>
                <p>June 8, 2022 8:00pm–9:00pm</p>
                <p>June 9, 2022 8:00pm–9:00pm, 10:00pm–11:00pm</p>
              </div>
              <div v-else><action-button primary tight @click="show_time_periods=true"><div class="icon"><time-icon /></div> Add &amp; customize time periods</action-button></div>

              <div v-if="recurrence" class="set-info">
                <div class="flex">
                  <h2>Recurrence</h2>
                  <div class="push-right">
                    <a class="action" @click="recurrence=false"><close-icon /></a>
                  </div>
                </div>
                <p style="margin-bottom:1rem;">Customize recurrences by selecting dates above and then setting frequency. Monthly recurrences should be set by selecting time periods above.</p>
                <b-field>
                  <template #label>
                    Select a frequency<span class="required">*</span>
                  </template>
                  <b-select placeholder="Select a frequency">
                    <option>Daily</option>
                    <option>Weekly</option>
                  </b-select>
                </b-field>
                <b-field>
                  <template #label>
                    Recurrence end date<span class="required">*</span>
                  </template>
                  <b-datepicker
                    v-model="selected"
                    placeholder="Click to select..."
                    icon="calendar-today"
                    :icon-right="selected ? 'close-circle' : ''"
                    icon-right-clickable
                    @icon-right-click="clearDate"
                    trap-focus>
                  </b-datepicker>
                </b-field>
              </div>
              <div v-else><action-button primary tight @click="recurrence=true">+ Add recurrence</action-button></div>
            </div>
          </transition>
        </label>

        <label class="label push-down">How can people learn more about your opportunity?<span class="required">*</span></label>

        <label class="button-radio" :class="{'open':learn=='link','unselected':learn=='none'}">
          <input type="radio" v-model="learn" name="learn" value="link" />
          <div class="icon-flex">
            <div class="br-img">
              <link-icon />
            </div>
            <div class="br-text">
              <h1>External Link</h1>
              <p>A web page with more information.</p>
            </div>
          </div>
          <transition name="slide">
            <div v-if="learn=='link'" class="add">
              <b-field message="Must start with http:// or https://">
                <template #label>
                  External link to learn more<span class="required">*</span>
                </template>
                <b-input v-model="onlineLink2"></b-input>
              </b-field>
            </div>
          </transition>
        </label>

        <label class="button-radio" :class="{'open':learn=='none','unselected':learn=='link'}">
          <input type="radio" v-model="learn" name="learn" value="none" />
          <div class="icon-flex">
            <div class="br-img">
              <snm-icon />
            </div>
            <div class="br-text">
              <h1>Only on Science Near Me</h1>
              <p>Be sure to add clear instructions on how to find out more in your opportunity’s description.</p>
            </div>
          </div>
        </label>


      </div><!-- state1 -->

      <div v-if="state==2">
        <div class="legend-flex">
          <legend>Required Fields</legend>
          <div class="required">* required</div>
        </div>
        <b-field message="164 character limit">
          <template #label>
            Short description of opportunity<span class="required">*</span>
          </template>
          <b-input v-model="shortdesc" maxlength="164" has-counter type="textarea"></b-input>
        </b-field>
        <b-field class="no-message">
          <template #label>
            Description of opportunity<span class="required">*</span>
          </template>
          <b-input v-model="desc" type="textarea" class="desc"></b-input>
        </b-field>

        <b-field>
          <template #label>
            Select the activity types that fit your opportunity best<span class="required">*</span>
          </template>
          <b-input type="input" placeholder="Type to filter activity list" class="filter"/>
          <div class="checkbox-wrap">
            <b-field v-for="a in activities">
              <b-checkbox>{{a}}</b-checkbox>
            </b-field>
          </div>
        </b-field>

        <b-field class="mb">
          <template #label>
            Associated Cost<span class="required">*</span>
          </template>
          <b-radio v-model="cost"
                   native-value="free">
            Free
          </b-radio>
          <b-radio v-model="cost"
                   native-value="cost">
            Cost
          </b-radio>
        </b-field>

        <label class="label">Age required to participate<span class="required">*</span></label>
        <div class="flex">
          <b-field label="Minimum Age">
            <b-numberinput controls-position="compact" v-model="min"></b-numberinput>
          </b-field>
          <b-field label="Maximum Age">
            <b-numberinput controls-position="compact" v-model="max"></b-numberinput>
          </b-field>
        </div>
        <p class="help mb">If there is no age requirement, set minimum to 0 and maximum to 120</p>

        <hr />

        <legend>Keywords and Key phrases</legend>
        <p class="help mb">Help your participants find your opportunity on Science Near Me. These help the search functionality!</p>

        <b-field message="Separate with a comma, select from established keywords as you type, the most popular keywords below, or add your own">
          <template #label>
            Add keywords and key phrases<span class="required">*</span>
          </template>
          <b-taginput
            v-model="tags"
            ellipsis
            icon="label"
            placeholder="Add a tag"
            aria-close-label="Delete this tag">
          </b-taginput>
        </b-field>
        <p>Most used keywords</p>
        <action-button v-for="k in mostUsed" tertiary tight>{{k}}</action-button>
      </div><!-- state 2 -->

      <div v-if="state==3">
        <legend>Display Image</legend>
        <p class="help mb">This is the image that will show when people see your opportunity’s record. If no image URL is provided, participants will see a default image.</p>

        <label class="label">Display Image</label>
        <div class="flex">
          <img :src="imgSrc" class="display-image"/>
          <b-field label="Image URL" message="Must start with http:// or https://">
            <b-input type="url" />
          </b-field>
        </div>

        <hr />

        <legend>Additional Information</legend>
        <p class="help mb">While not required, this information will tell prospective participants more about your opportunity and help them find your opportunity.</p>

        <b-field label="Ticket Required" class="mb">
          <b-radio v-model="ticket"
                   native-value="yes">
            Yes
          </b-radio>
          <b-radio v-model="ticket"
                   native-value="no">
            No
          </b-radio>
        </b-field>

        <b-field label="RSVP Required" class="mb">
          <b-radio v-model="rsvp"
                   native-value="yes">
            Yes
          </b-radio>
          <b-radio v-model="rsvp"
                   native-value="no">
            No
          </b-radio>
        </b-field>

        <b-field label="Select the topics that fit your opportunity best">
          <b-input type="input" placeholder="Type to filter topic list" class="filter"/>
          <div class="checkbox-wrap">
            <b-field v-for="a in topics">
              <b-checkbox>{{a}}</b-checkbox>
            </b-field>
          </div>
        </b-field>

        <b-field label="Select the venue(s) that fit your opportunity best" class="inline-checks">
          <b-checkbox v-for="a in venues">{{a}}</b-checkbox>
        </b-field>

        <hr />

        <legend>Social Media</legend>
        <p class="help mb">All of this information is optional. You can always add or edit later through your dashboard.</p>

        <b-field message="Separate with a comma">
          <template #label>
            Opportunity Hashtags
            <p class="help">When people use social media to talk about and share your opportunity, what hashtags would you like them to use? (e.g. #iowasciencefest21)</p>
          </template>

          <b-taginput
            v-model="hashtags"
            ellipsis
            icon="label"
            placeholder="Add a tag"
            aria-close-label="Delete this tag">
          </b-taginput>
        </b-field>

        <b-field label="Twitter Handle" message="must start with @">
          <b-input v-model="twitter"></b-input>
        </b-field>

        <b-field label="Instagram Handle" message="must start with @">
          <b-input v-model="instagram"></b-input>
        </b-field>

        <b-field label="Facebook Page" message="must start with http:// or https://">
          <b-input v-model="facebook" type="url"></b-input>
        </b-field>


      </div><!-- state 3 -->

    </div><!-- .opp-form-wrapper -->


    <div class="form-actions">
      <div class="snm-container">
        <template v-if="editMode">
          <action-button primary :disabled="saveDisabled">Save &amp; Continue Editing</action-button>
          <action-button primary :disabled="saveDisabled">Save &amp; View</action-button>
        </template>
        <template v-else>
          <action-button v-if="state==2 || state==3"  @click="state--" gray>Back</action-button>
          <action-button v-if="state==1" @click="state++" primary :disabled="nextDisabled1">Next Step</action-button>
          <action-button v-if="state==2" @click="state++" primary :disabled="nextDisabled2">Next Step</action-button>
          <action-button v-if="state<3" tertiary>Save and Complete Later</action-button>
          <action-button v-if="state==3" primary>Save and Publish</action-button>
          <action-button v-if="state==3" tertiary>Save and Publish Later</action-button>


          <template v-if="saveState=='saved'">
            <div class="save-feedback"><div class="icon"><correct-icon /></div> saved</div>
          </template>
          <template v-else-if="saveState=='saving'">
            <div class="save-feedback saving"><img src="~/assets/img/loading-buffering.gif" class="icon" /> saving</div>
          </template>
          <template v-else-if="saveState=='error'">
            <div class="save-feedback error"><div class="icon"><cross-icon /></div> Error Saving!</div>
          </template>

        </template>
      </div>
    </div><!-- .form-actions -->

  </form>


  <b-modal v-model="show_end_date" :width="640" aria-role="dialog" aria-label="Log in" aria-modal class="form-modal">
    <div class="card">
      <h1>Select an End Date<span class="close" @click="show_end_date = false">&times;</span></h1>
      <p>If your ongoing opportunity has an end date, select below.</p>
      <div class="flex flex-center">
        <b-datepicker v-model="endDate" inline></b-datepicker>
      </div>
      <div class="flex flex-center">
        <action-button tertiary @click="()=>{show_end_date=false,endDate=null}">cancel</action-button>
        <action-button primary @click="show_end_date=false">save</action-button>
      </div>
    </div>
  </b-modal>

  <b-modal v-model="show_time_periods" :width="800" aria-role="dialog" aria-label="Log in" aria-modal class="form-modal">
    <div class="card">
      <h1>Add and Customize Dates and Times <span class="close" @click="show_time_periods = false">&times;</span></h1>
      <p>Select dates on the calendar. Each date must have at least one time period set on the right.</p>
      <div class="flex">
        <b-field>
          <template #label>
            Select Dates<span class="required">*</span>
          </template>
          <b-datepicker v-model="timePeriods" inline multiple></b-datepicker>
        </b-field>
        <div id="time-periods">
          <label class="label">Add times to each date<span class="required">*</span></label>
          <div class="tp-list">

            <!-- date has no time set -->
            <div class="tp-item">
              <div class="flex">
                <h2>Tue, Feb 22 2022</h2>
                <div class="push-right">
                  <a class="action"><close-icon /></a><!-- this removes date -->
                </div>
              </div>
              <div class="flex">
                <b-timepicker
                  placeholder="Start Time"
                  icon="clock">
                </b-timepicker>
                <b-timepicker
                  placeholder="End Time"
                  icon="clock">
                </b-timepicker>
              </div>
            </div>
            <!-- date has one or more times set -->
            <div class="tp-item">
              <div class="flex">
                <h2>Tue, Feb 22 2022</h2>
                <a class="action push-right"><close-icon /></a><!-- this removes date -->
              </div>
              <p>12:00pm&mdash;2:00pm<a class="action inline-action"><close-icon /></a><!-- this removes time --></p>
              <p>4:00pm&mdash;8:00pm<a class="action inline-action"><close-icon /></a><!-- this removes time --></p>
              <div class="flex" v-if="show_time">
                <b-timepicker
                  placeholder="Start Time"
                  icon="clock">
                </b-timepicker>
                <b-timepicker
                  placeholder="End Time"
                  icon="clock">
                </b-timepicker>
              </div>
              <action-button v-else tertiary tight @click="show_time=true">+ Add time</action-button>
            </div>

          </div>
        </div><!-- #time-periods -->
      </div>
      <div class="flex flex-center">
        <action-button tertiary @click="()=>{show_time_periods=false}">cancel</action-button>
        <action-button primary @click="show_time_periods=false">save</action-button>
      </div>
    </div>
  </b-modal>

</div>
</template>

<script>
import EyeIcon from '~/assets/img/eye.svg?inline'
import CorrectIcon from '~/assets/img/correct.svg?inline'
import CrossIcon from '~/assets/img/cross.svg?inline'
import WebsiteIcon from '~/assets/img/website.svg?inline'
import LocationIcon from '~/assets/img/location-marker.svg?inline'
import BothIcon from '~/assets/img/both-locations.svg?inline'
import InfinityIcon from '~/assets/img/infinity.svg?inline'
import TimeIcon from '~/assets/img/time.svg?inline'
import EditIcon from '~/assets/img/edit.svg?inline'
import CloseIcon from '~/assets/img/close.svg?inline'
import LinkIcon from '~/assets/img/link.svg?inline'
import SnmIcon from '~/assets/img/atom-one-color.svg?inline'

export default {
    name: "OportunityForm",

    components: {
        EyeIcon,
        CorrectIcon,
        CrossIcon,
        WebsiteIcon,
        LocationIcon,
        BothIcon,
        InfinityIcon,
        TimeIcon,
        EditIcon,
        CloseIcon,
        LinkIcon,
        SnmIcon,
    },

    props: {
        editMode: { // this should be set true if using form to edit an existing opportunity
            type: Boolean,
            required: false,
            default: false,
        },

        partner: {
            type: Object,
            required: true,
        },

        opportunity: {
            type: Object,
            required: true,
        },
    },

    data() {
        return{
            state: 1,
            nextDisabled1: false,
            nextDisabled2: false,
            saveState: 'saved',
            saveDisabled: false,
            when:null,
            learn:null,
            show_end_date: false,
            endDate: null,
            recurrence: false,
            show_time_periods: false,
            timePeriods:[],
            show_time: false,
            activities:['bioblitz','taco truck','bioblitz','taco truck','bioblitz','taco truck','bioblitz','taco truck','bioblitz','taco truck'],
            topics:['bioblitz','taco truck','bioblitz','taco truck','bioblitz','taco truck','bioblitz','taco truck','bioblitz','taco truck'],
            venues:['bioblitz','taco truck','bioblitz','taco truck','bioblitz','taco truck','bioblitz','taco truck','bioblitz','taco truck'],
            cost: 'free',
            min:0,
            max:120,
            mostUsed: ['family friendly','wombat','taco truck','family friendly','wombat','taco truck','family friendly','wombat'],
            imgSrc: '/_nuxt/assets/img/no-image-thumb.jpg',
        }
    },

    computed: {
        location: {
            get() {
                if(this.opportunity.is_online && (this.opportunity.location_type == 'at' || this.opportunity.location_type == 'near')) {
                    return 'both';
                }
                else if(this.opportunity.is_online) {
                    return 'online';
                }
                else if(this.opportunity.location_type == 'at' || this.opportunity.location_type == 'near') {
                    return 'physical';
                }
                else {
                    return null;
                }
            },

            set(val) {
                if(val == 'both') {
                    this.opportunity.is_online = true;
                    this.opportunity.location_type = 'near';
                }
                else if(val == 'online') {
                    this.opportunity.is_online = true;
                    this.opportunity.location_type = 'any';
                    this.opportunity.location_name = '';
                    this.opportunity.location_point = null;
                    this.opportunity.location_polygon = null;
                }
                else if(val == 'physical') {
                    this.opportunity.is_online = false;
                    this.opportunity.location_type = 'near';
                }
                else {
                    console.error("Unrecognized location value: " + val);
                }
            },
        },
    },

    watch: {
        partner(val, old) {
            this.opportunity.partner = val.uid;
            this.opportunity.partner_name = val.name;
            this.opportunity.partner_url = val.url;
            this.opportunity.partner_logo_url = val.image_url;
        },
    },

    mounted() {
        this.opportunity.partner = this.partner.uid;
        this.opportunity.partner_name = this.partner.name;
        this.opportunity.partner_url = this.partner.url;
        this.opportunity.partner_logo_url = this.partner.image_url;
    },

    methods: {
        location_point(point) {
            this.opportunity.location_type = 'near';
            this.opportunity.location_point = point;
            this.opportunity.location_polygon = null;
        },

        location_poly(poly) {
            this.opportunity.location_type = 'near';
            this.opportunity.location_point = null;
            this.opportunity.location_polygon = poly;
        },

        location_license(text) {
            this.opportunity.extra_data.location_license = text;
        },
    },
}
</script>

<style lang="scss">


.opp-form-wrapper {
    padding-bottom:80px;
    .input {
        max-width:600px!important;
    }

}
.track {
    list-style-type:none;
    display: flex;
    padding-bottom:1rem;
    border-bottom: 1px solid $snm-color-border;
    margin-bottom: 2rem;
    margin-top:2rem;
    font-size:14px;
    li {
        margin-right: 16px;
    }
    a {
        font-weight:normal;
        color: #868686;
        cursor: default;
        pointer-events:none;
        > span {
            background-color: #868686;
            color: #fff;
            display:inline-block;
            width:20px;
            height:20px;
            line-height:20px;
            text-align:center;
            border-radius:100%;
            font-size:12px;
            margin-right:6px;
            font-weight:bold;
        }
        &.active{
            color:$snm-color-element-med;
            font-weight:bold;
            text-decoration:none!important;
            > span {
                background-color: $snm-color-element-med;
            }
        }
        &:hover {
            color:$snm-color-element-med;
            text-decoration:underline;
            > span {
                background-color: $snm-color-element-med;
            }
        }
    }
}

.form-actions {
    position:fixed;
    bottom:0;
    right:0;
    width: calc(100% - 280px);
    background-color: #fff;
    border-top: 1px solid $snm-color-border;
    z-index:99;

    .snm-container {
        display: flex;
        align-items: center;
        max-width:1000px;
        margin:0 auto;
    }
}

@media (max-width:1199px){
    .form-actions {
        width: calc(100% - 200px);
        padding:0 20px;
    }
}

@media (max-width:959px){
    .form-actions {
        width: 100%;
        padding:0 10px;
    }
}

.save-feedback {
    display:flex;
    margin-left:auto;
    font-size:14px;
    &.saving {
        color: $snm-color-disabled;
    }
    &.error {
        color: $snm-color-info;
    }
    .icon {
        margin-right:8px;
        margin-top:-2px;
    }
}

.legend-flex {
    margin-bottom:2rem;
    display:flex;
    align-items:center;
    font-weight:bold;
    font-size:14px;
    .required {
        margin-left:auto;
    }
}
legend {
    text-transform: uppercase;
    color: $snm-color-element-med;
    font-weight:bold;
    font-size:14px;
}

.label:not(:last-child) {
    margin-bottom:0.25rem;
}
.push-down {
    margin-top:2rem;
}

.required {
    color: $snm-color-info;
}
.help {
    font-size:13px;
    font-style:italic!important;
    font-weight:normal;
}

.button-radio input[type=radio] {
    display: none;
}

.button-radio {
    cursor:pointer;
    border: 1px solid #C4C4C4;
    background-color: #EFEFEF;
    margin-bottom:10px;
    display: block;
    border-radius:6px;
    padding:10px;

    &:hover, &.open {
        background-color: $snm-color-background-medlight;
        border-color: $snm-color-background-meddark;
    }

    svg {
        width:34px;
        height:auto;
    }

    svg path {
        fill: #155e6f;
    }

    h1 {
        font-weight:bold;
        color: #155e6f;
        margin-bottom:2px;
    }
    p {
        color: #5A5A5A;
    }
    .br-text {
        line-height:1;
    }

    &.unselected {
        svg path {
            fill:#A8A8A8;
        }
        h1, p {
            color: #A8A8A8;
        }
    }

}

.icon-flex {
    display: flex;
    align-items: center;
    .br-img {
        width:34px;
        margin-right:12px;
    }
}

.add {
    margin-top:1rem;
    margin-left:34px;

    input, select {
        border-color: #b7b7b7!important;
    }
}

.times-flex {
    .field, .field .datepicker {
        margin-right:10px!important;
    }
}

.set-info {
    background-color: #fff;
    border: 1px solid #b7b7b7;
    max-width: 500px;
    padding:12px;
    border-radius:6px;

    h2 {
        font-weight:bold;
    }
    p {
        line-height:1.1;
        font-size:14px;
        margin-bottom:0;
    }
}


.slide-enter-active {
    -moz-transition-duration: 0.2s;
    -webkit-transition-duration: 0.2s;
    -o-transition-duration: 0.2s;
    transition-duration: 0.2s;
    -moz-transition-timing-function: ease-in;
    -webkit-transition-timing-function: ease-in;
    -o-transition-timing-function: ease-in;
    transition-timing-function: ease-in;
}

.slide-leave-active {
    -moz-transition-duration: 0.1s;
    -webkit-transition-duration: 0.1s;
    -o-transition-duration: 0.1s;
    transition-duration: 0.1s;
    -moz-transition-timing-function: cubic-bezier(0, 1, 0.5, 1);
    -webkit-transition-timing-function: cubic-bezier(0, 1, 0.5, 1);
    -o-transition-timing-function: cubic-bezier(0, 1, 0.5, 1);
    transition-timing-function: cubic-bezier(0, 1, 0.5, 1);
}

.slide-enter-to, .slide-leave {
    max-height: 100px;
    overflow: hidden;
}

.slide-enter, .slide-leave-to {
    overflow: hidden;
    max-height: 0;
}

.form-modal {
    h1 {
        color: $snm-color-background-meddark;
        font-weight:bold;
        font-size:18px;
        font-family: $snm-font-heading;
        display:flex;
        justify-content:space-between;
        align-items:center;
        span {
            font-size: 44px;
            display: block;
            line-height: 1;
            font-weight: normal;
            cursor:pointer;
        }
    }
    p {
        margin-bottom:2rem;
    }
    .flex-center {
        justify-content:center;
    }
}

.push-right {
    margin-left:auto;
}

.action {
    margin-left:6px;

    svg {
        height:14px;
        width:auto;
        path {
            fill: #e5e5e5;
        }
    }
    &:hover {
        svg path {
            fill: $snm-color-background-meddark;
        }
    }

}

#time-periods {

    margin-left:20px;

    .tp-list {
        overflow: auto;
        height: 358px;
        padding:20px;
        border: 1px solid $snm-color-border;

    }

    .tp-item {
        border-bottom: 1px solid $snm-color-border;
        padding: 20px 0;
        &:first-child {
            padding-top:0;
        }
        &:last-child {
            border-bottom:0;
        }
        p {
            margin-bottom:0;
        }
        h2 {
            font-weight:bold;
        }
        .push-right{
            margin-left:auto;
        }
        .inline-action {
            margin-left:10px;
        }
        .timepicker {
            margin-top:10px;
        }
        .timepicker:first-child {
            margin-right:10px;
        }
    }
}

.opp-form-wrapper {
    .desc textarea {
        min-height: 400px!important;
        max-height: 100rem!important;
    }

    .filter {
        max-width:300px;
    }

    .checkbox-wrap {
        max-width: 300px;
        border:1px solid $snm-color-border;
        max-height:240px;
        overflow:auto;
        padding:10px;
        margin-bottom:2rem;
    }
    .no-message, .mb {
        margin-bottom:2rem!important;
    }
    .b-radio {
        margin-right:2rem!important;
    }

  input[type=number] {
    width:50px;
  }
  .b-numberinput {
    margin-right:2rem;
  }

  .display-image {
    height: auto;
    margin: 0 1rem 1rem 0;
    border: 1px solid #d9d9d9;
    max-width: 200px;
    border-radius: 6px;
    -o-object-fit: contain;
    object-fit: contain;
    max-height: 180px;
    min-width:300px;
  }
  .inline-checks .field {
    flex-wrap:wrap;
  }
  .inline-checks .b-checkbox {
    margin-right:2rem;
    margin-bottom:1rem;
  }

}

</style>
