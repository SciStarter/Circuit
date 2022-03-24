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
        <b-field :type="validation.title" message="64 character maximum">
          <template #label>
            Name of Opportunity<span class="required">*</span>
          </template>
          <b-input v-model="value.title" has-counter maxlength="64"></b-input>
        </b-field>
        <b-field :type="validation.organization_name" message="This is the organization hosting the event, project, or attraction. This might be your organization, a chapter, or similar. This is displayed in search results and opportunity pages.">
          <template #label>
            Host Organization<span class="required">*</span>
          </template>
          <b-input v-model="value.organization_name"></b-input>
        </b-field>
        <b-field :type="validation.name" :message="'This opportunity is on Science Near Me under the auspices of the selected Science Near Me partner.' + (editMode ? ' If this needs to change, you must contact Science Near me.' : '')">
          <template #label>
            Science Near Me partner<span class="required">*</span>
          </template>
          <b-input v-model="partner.name" disabled></b-input>
        </b-field>

        <b-field :type="validation.location">
          <template #label>
            Where is your opportunity?<span class="required">*</span>
          </template>

          <div class="control validation-target" :class="{'is-danger': validation.location}">
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
                  <b-field :type="validation.partner_opp_url" message="Must start with http:// or https://">
                    <template #label>
                      External link To participate<span class="required">*</span>
                    </template>
                    <b-input type="url" :value="value.partner_opp_url" @input="value.partner_opp_url = $event.replace(/ /g, '')"></b-input>
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
                  <b-field :type="validation.location_name" message="Begin typing and select location">
                    <template #label>
                      Search for an address or location<span class="required">*</span>
                    </template>
                    <lookup-geometry v-model="value.location_name" @polygon="location_poly" @point="location_point" @license="location_license" />
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
                  <b-field :type="validation.location_name" message="Begin typing and select location">
                    <template #label>
                      Search for an address or location<span class="required">*</span>
                    </template>
                    <lookup-geometry v-model="value.location_name" @polygon="location_poly" @point="location_point" @license="location_license" />
                  </b-field>
                  <b-field :type="validation.partner_opp_url" message="Must start with http:// or https://">
                    <template #label>
                      External link To participate<span class="required">*</span>
                    </template>
                    <b-input type="url" :value="value.partner_opp_url" @input="value.partner_opp_url = $event.replace(/ /g, '')"></b-input>
                  </b-field>
                </div>
              </transition>
            </label>
          </div>
        </b-field>

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
              <div class="set-info" v-if="end_datetime">
                <div class="flex">
                  <h2>End Date</h2>
                  <div class="push-right">
                    <a class="action" @click="show_end_date=true"><edit-icon /></a>
                    <a class="action" @click="end_datetime=null"><close-icon /></a>
                  </div>
                </div>
                <p>{{end_datetime.toLocaleDateString()}}</p>
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
              <b-field :type="validation.timezone">
                <template #label>
                  Select a time zone<span class="required">*</span>
                </template>
                <b-select v-model="value.timezone" placeholder="Select a time zone">
                  <option v-for="tz in timezones" :value="tz" :key="tz">
                    {{ tz.replace(/_/g, " ") }}
                  </option>
                </b-select>
              </b-field>
              <div v-if="time_periods.length == 0 || time_periods.length == 1" class="times-flex">
                <div class="flex">
                  <b-field>
                    <template #label>
                      Starts on:<span class="required">*</span>
                    </template>
                    <b-datepicker
                      class="validation-target"
                      :class="{'is-danger': validation.begin_datetime}"
                      v-model="begin_datetime"
                      :max-date="end_datetime"
                      placeholder="Click to select..."
                      icon="calendar-today"
                      :icon-right="begin_datetime ? 'close-circle' : ''"
                      icon-right-clickable
                      @icon-right-click="begin_datetime=null"
                      trap-focus>
                    </b-datepicker>
                    <b-timepicker
                      class="validation-target"
                      :class="{'is-danger': validation.begin_datetime}"
                      v-model="begin_datetime"
                      placeholder="Click to select..."
                      icon="clock"
                      editable>
                    </b-timepicker>
                  </b-field>
                  <b-field :type="validation.end_datetime">
                    <template #label>
                      Ends on:
                    </template>
                    <b-datepicker
                      v-model="end_datetime"
                      :min-date="begin_datetime"
                      placeholder="Click to select..."
                      icon="calendar-today"
                      :icon-right="end_datetime ? 'close-circle' : ''"
                      icon-right-clickable
                      @icon-right-click="end_datetime=null"
                      trap-focus>
                    </b-datepicker>
                    <b-timepicker
                      v-model="end_datetime"
                      placeholder="Click to select..."
                      icon="clock"
                      editable>
                    </b-timepicker>
                  </b-field>
                </div>
              </div>

              <div v-if="time_periods.length > 1" class="set-info" style="margin-bottom:1rem;">
                <div class="flex">
                  <h2>Time Periods</h2>
                  <div class="push-right">
                    <a class="action" @click="()=>{time_periods_dates = time_periods.map(pair => pair[0]); show_time_periods = true;}"><edit-icon /></a>
                    <a class="action" @click="()=>{time_periods_dates = []; time_periods = [];}"><close-icon /></a>
                  </div>
                </div>
                <p v-for="pair in time_periods_display">
                  <span v-if="pair[0] && pair[1] && pair[0].getFullYear() == pair[1].getFullYear() && pair[0].getMonth() == pair[1].getMonth() && pair[0].getDate() == pair[1].getDate()">{{pair[0].toLocaleDateString()}} {{pair[0].toLocaleTimeString()}} - {{pair[1].toLocaleTimeString()}}</span>
                  <span v-else>{{pair[0] ? (pair[0].toLocaleString() + ' -') : 'ongoing through'}} {{pair[1].toLocaleString()}}</span>
                </p>
              </div>
              <div v-else><action-button primary tight @click="show_time_periods=true"><div class="icon"><time-icon /></div> Add &amp; customize time periods</action-button></div>

              <div v-if="can_recur && recurrence" class="set-info">
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
                  <b-select v-model="value.recurrence" placeholder="Select a frequency">
                    <option value="once">Once</option>
                    <option value="daily">Daily</option>
                    <option value="weekly">Weekly</option>
                  </b-select>
                </b-field>
                <b-field>
                  <template #label>
                    Recurrence end date<span class="required">*</span>
                  </template>
                  <b-datepicker
                    :value="value.end_recurrence ? new Date(value.end_recurrence) : null"
                    @input="value.end_recurrence = $event.toISOString()"
                    placeholder="Click to select..."
                    icon="calendar-today"
                    :icon-right="value.end_recurrence ? 'close-circle' : ''"
                    icon-right-clickable
                    @icon-right-click="value.end_recurrence=null"
                    trap-focus>
                  </b-datepicker>
                </b-field>
              </div>
              <div v-else-if="can_recur"><action-button primary tight @click="recurrence=true">+ Add recurrence</action-button></div>
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
              <b-field :type="validation.organization_website" message="Must start with http:// or https://">
                <template #label>
                  External link to learn more<span class="required">*</span>
                </template>
                <b-input :value="value.organization_website" @input="value.organization_website = $event.replace(/ /g, '')"></b-input>
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
        <b-field :type="validation.short_desc" message="164 character limit">
          <template #label>
            Short Summary (appears in search results)<span class="required">*</span>
          </template>
          <b-input v-model="value.short_desc" maxlength="164" has-counter type="textarea"></b-input>
          <p class="help mb">Tell prospective participants what to expect from your opportunity in a short, friendly sentence.</p>
        </b-field>
        <b-field :type="validation.description" class="no-message">
          <template #label>
            Description of opportunity<span class="required">*</span>
          </template>
          <b-input v-model="value.description" type="textarea" class="desc"></b-input>
        </b-field>

        <b-field :type="validation.descriptors">
          <template #label>
            Select the activity types that fit your opportunity best<span class="required">*</span>
          </template>

          <div class="control validation-target" :class="{'is-danger': validation.opp_descriptor}">
            <b-input :value="descriptors_filter" @input="descriptors_filter = $event.toLowerCase()" type="input" placeholder="Type to filter activity list" class="filter"/>
            <div class="checkbox-wrap">
              <template v-for="a in descriptors">
                <b-field v-if="a[1].toLowerCase().indexOf(descriptors_filter) >= 0">
                  <b-checkbox v-model="value.opp_descriptor" :native-value="a[0]">{{a[1]}}</b-checkbox>
                </b-field>
              </template>
            </div>
          </div>
        </b-field>

        <b-field :type="validation.cost" class="mb">
          <template #label>
            Associated Cost<span class="required">*</span>
          </template>
          <b-radio v-model="value.cost" native-value="free">
            Free
          </b-radio>
          <b-radio v-model="value.cost" native-value="cost">
            Cost
          </b-radio>
        </b-field>

        <label class="label">Age required to participate<span class="required">*</span></label>
        <div class="flex">
          <b-field :type="validation.min_age" label="Minimum Age">
            <b-checkbox v-model="has_minimum">There is a minimum age for participants</b-checkbox>
            <b-numberinput v-if="has_minimum" controls-position="compact" v-model="value.min_age"></b-numberinput>
          </b-field>
          <b-field :type="validation.max_age" label="Maximum Age">
            <b-checkbox v-model="has_maximum">There is a maximum age for participants</b-checkbox>
            <b-numberinput v-if="has_maximum" controls-position="compact" v-model="value.max_age" :min="value.min_age"></b-numberinput>
          </b-field>
        </div>

        <hr />

        <legend>Keywords and Key phrases</legend>
        <p class="help mb">Help your participants find your opportunity on Science Near Me. These help the search functionality!</p>

        <b-field :type="validation.tags" message="Separate with a comma. The most popular keywords are below, or add your own">
          <template #label>
            Add keywords and key phrases<span class="required">*</span>
          </template>
          <b-taginput
            v-model="value.tags"
            ellipsis
            icon="label"
            placeholder="Add a tag"
            aria-close-label="Delete this tag">
          </b-taginput>
        </b-field>
        <p>Most used keywords</p>
        <action-button v-for="k in mostUsed" :key="k" tertiary tight @click="value.tags.push(k)">{{k}}</action-button>
      </div><!-- state 2 -->

      <div v-if="state==3">
        <legend>Display Image</legend>
        <p class="help mb">This is the image that will show when people see your opportunity’s record. If no image URL is provided, participants will see a default image.</p>

        <label class="label">Display Image</label>
        <div class="flex display-image-wrapper">
          <div>
            <img v-if="value.image_url" :src="value.image_url" class="display-image">
          </div>
          <b-field :type="validation.image_url" label="Image URL" message="Must start with http:// or https://">
            <b-input type="url" :value="value.image_url" @input="value.image_url = $event.replace(/ /g, '')" />
          </b-field>
        </div>

        <hr />

        <legend>Additional Information</legend>
        <p class="help mb">While not required, this information will tell prospective participants more about your opportunity and help them find your opportunity.</p>

        <b-field :type="validation.ticket_required" label="Ticket Required" class="mb">
          <b-radio v-model="value.ticket_required" :native-value="true">
            Yes
          </b-radio>
          <b-radio v-model="value.ticket_required" :native-value="false">
            No
          </b-radio>
        </b-field>

        <!-- RSVP isn't in our data model yet -->
        <!-- <b-field label="RSVP Required" class="mb"> -->
        <!--   <b-radio v-model="rsvp" native-value="yes"> -->
        <!--     Yes -->
        <!--   </b-radio> -->
        <!--   <b-radio v-model="rsvp" native-value="no"> -->
        <!--     No -->
        <!--   </b-radio> -->
        <!-- </b-field> -->

        <b-field :type="validation.topics" label="Select the topics that fit your opportunity best">
          <b-input :value="topics_filter" @input="topics_filter = $event.toLowerCase()" type="input" placeholder="Type to filter topic list" class="filter"/>
          <div class="checkbox-wrap">
            <template v-for="t in topics">
              <b-field v-if="t[1].toLowerCase().indexOf(topics_filter) >= 0">
                <b-checkbox v-model="value.opp_topics" :native-value="t[0]">{{t[1]}}</b-checkbox>
              </b-field>
            </template>
          </div>
        </b-field>

        <b-field :type="validation.opp_venue" label="Select the venue type(s) that fit your opportunity best" class="inline-checks">
          <b-checkbox v-model="value.opp_venue" native-value="indoors">Indoors</b-checkbox>
          <b-checkbox v-model="value.opp_venue" native-value="outdoors">Outdoors</b-checkbox>
        </b-field>

        <hr />

        <legend>Social Media</legend>
        <p class="help mb">All of this information is optional. You can always add or edit later through your dashboard.</p>

        <b-field :type="validation.opp_hashtags" message="Separate with a comma">
          <template #label>
            Opportunity Hashtags
            <p class="help">When people use social media to talk about and share your opportunity, what hashtags would you like them to use? (e.g. #iowasciencefest21)</p>
          </template>

          <b-taginput
            v-model="value.opp_hashtags"
            ellipsis
            icon="label"
            placeholder="Add a tag"
            aria-close-label="Delete this tag"
            class="hashtags">
          </b-taginput>
        </b-field>

        <b-field label="Twitter Handle" message="must start with @">
          <b-input v-model="value.opp_social_handles.twitter"></b-input>
        </b-field>

        <b-field label="Instagram Handle" message="must start with @">
          <b-input v-model="value.opp_social_handles.instagram"></b-input>
        </b-field>

        <b-field label="Facebook Page" message="must start with http:// or https://">
          <b-input v-model="value.opp_social_handles.facebook" type="url"></b-input>
        </b-field>


      </div><!-- state 3 -->

    </div><!-- .opp-form-wrapper -->


    <div class="form-actions">
      <div class="snm-container">
        <template v-if="editMode">
          <action-button primary :disabled="saveDisabled" @click="save">Save &amp; Continue Editing</action-button>
          <action-button primary :disabled="saveDisabled" @click="save_and_view">Save &amp; View</action-button>
        </template>
        <template v-else>
          <action-button v-if="state==2 || state==3"  @click="go_state(state-1)" gray>Back</action-button>
          <action-button v-if="state==1" @click="go_state(state+1)" primary :disabled="nextDisabled1">Next Step</action-button>
          <action-button v-if="state==2" @click="go_state(state+1)" primary :disabled="nextDisabled2">Next Step</action-button>
          <action-button v-if="state<3" tertiary @click="save_and_view">Save and Complete Later</action-button>
          <action-button v-if="state==3" primary @click="save_and_publish">Save and Publish</action-button>
          <action-button v-if="state==3" tertiary @click="save_and_view">Save and Publish Later</action-button>
        </template>
        <template v-if="saveState=='saved'">
          <div class="save-feedback"><div class="icon"><correct-icon /></div><span> saved</span></div>
        </template>
        <template v-else-if="saveState=='saving'">
          <div class="save-feedback saving"><img src="~/assets/img/loading-buffering.gif" class="icon" /><span> saving</span></div>
        </template>
        <template v-else-if="saveState=='error'">
          <div class="save-feedback error"><div class="icon"><cross-icon /></div><span> unable to save</span></div>
        </template>
      </div>
    </div><!-- .form-actions -->

  </form>


  <b-modal v-model="show_end_date" :width="640" aria-role="dialog" aria-label="Log in" aria-modal class="form-modal" @close="end_datetime=null">
    <div class="card">
      <h1>Select an End Date<span class="close" @click="show_end_date = false">&times;</span></h1>
      <p>If your ongoing opportunity has an end date, select below.</p>
      <div class="flex flex-center">
        <b-datepicker v-model="end_datetime" inline></b-datepicker>
      </div>
      <div class="flex flex-center">
        <action-button tertiary @click="()=>{show_end_date=false;end_datetime=null;}">clear</action-button>
        <action-button primary @click="show_end_date=false">save</action-button>
      </div>
    </div>
  </b-modal>

  <b-modal v-model="show_time_periods" :width="800" aria-role="dialog" aria-label="Log in" aria-modal class="form-modal">
    <div class="card">
      <h1>Add and Customize Dates and Times <span class="close" @click="show_time_periods = false">&times;</span></h1>
      <p>Select dates on the calendar. Each date must have at least one time period set on the right.</p>
      <div class="flex" id="modal-dates">
        <b-field>
          <template #label>
            Select Dates<span class="required">*</span>
          </template>
          <b-datepicker :value="time_periods_dates" @input="time_periods_dates_set" inline multiple></b-datepicker>
        </b-field>
        <div id="time-periods">
          <label class="label">Add times to each date<span class="required">*</span></label>
          <div class="tp-list">

            <!-- date has no time set -->
            <div v-for="(pair, idx) in time_periods_local" class="tp-item">
              <div class="flex">
                <h2>{{ pair[0] ? pair[0].toLocaleDateString() : '' }}</h2>
              </div>
              <div class="flex">
                <b-timepicker
                  :value="pair[0]"
                  placeholder="Start Time"
                  icon="clock"
                  editable
                  @input="time_periods_set(idx, 0, $event)">
                </b-timepicker>
                <b-timepicker
                  :value="pair[1]"
                  position="is-bottom-left"
                  placeholder="End Time"
                  icon="clock"
                  editable
                  @input="time_periods_set(idx, 1, $event)">
                </b-timepicker>
              </div>
            </div>
          </div>
        </div><!-- #time-periods -->
      </div>
      <div class="flex flex-center">
        <action-button tertiary @click="()=>{show_time_periods = false; time_periods_dates = []; time_periods = [];}">clear</action-button>
        <action-button primary @click="show_time_periods=false">save</action-button>
      </div>
    </div>
  </b-modal>

</div>
</template>

<script>
import debounce from 'lodash/debounce'

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

function dbg(x) {
    console.log(x);
    return x;
}

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

        value: {
            type: Object,
            required: true,
        },

        timezones: {
            type: Array,
            required: true,
        },

        descriptors: {
            type: Array,
            required: true,
        },

        topics: {
            type: Array,
            required: true,
        },
    },

    data() {
        return{
            state: 1,
            nextDisabled1: false,
            nextDisabled2: false,
            saveState: '',
            saveDisabled: false,
            when:null,
            learn:null,
            show_end_date: false,
            recurrence: false,
            show_time_periods: false,
            show_time: false,
            mostUsed: ['museum', 'astronomy', 'afterschool', 'library', 'kids', 'citizen science', 'nature'],
            topics_filter: '',
            descriptors_filter: '',
            skip_updates: 0,
            timeout: 0,
            validation: {},
            time_periods_dates: [],
        }
    },

    fetch() {
        this.initialize();
    },

    computed: {
        has_minimum: {
            get() {
                return this.value.min_age > 0;
            },

            set(val) {
                if(val) {
                    this.value.min_age = 1;
                }
                else {
                    this.value.min_age = 0;
                }
            }
        },

        has_maximum: {
            get() {
                return this.value.max_age < 999;
            },

            set(val) {
                if(val) {
                    this.value.max_age = 120;
                }
                else {
                    this.value.max_age = 999;
                }
            }
        },

        can_recur() {
            if(this.full_span === null) {
                return false;
            }

            const millis = this.full_span[1] - this.full_span[0];
            return millis < 1000 * 60 * 60 * 24 * 7;
        },

        full_span() {
            return this.time_periods.reduce((accum, val) => {
                if(accum === null) {
                    return [Math.min.apply(null, val), Math.max.apply(null, val)];
                }
                else {
                    const min = Math.min.apply(null, val);
                    const max = Math.max.apply(null, val);
                    accum[0] = Math.min(accum[0], min);
                    accum[1] = Math.max(accum[1], max);
                    return accum;
                }
            }, null);
        },

        time_periods_ISO() {
            const starts = this.value.start_datetimes;
            const ends = this.value.end_datetimes;

            const pairs =  Array(Math.max(starts.length, ends.length))
                  .fill()
                  .map((_,i) => [starts[i], ends[i]]);

            return pairs
        },

        time_periods_local() {
            return this.time_periods_ISO.map(x => {
                return [x[0] ? new Date(x[0].substring(0, 19)) : x[0], x[1] ? new Date(x[1].substring(0, 19)) : x[1]];
            });
        },

        time_periods_display() {
            let pairs = [...this.time_periods_ISO];

            pairs.sort();

            return pairs.map(x => {
                return [x[0] ? new Date(x[0].substring(0, 19)) : x[0], x[1] ? new Date(x[1].substring(0, 19)) : x[1]];
            });
        },

        time_periods: {
            get() {
                const starts = this.value.start_datetimes;
                const ends = this.value.end_datetimes;

                const pairs =  Array(Math.max(starts.length, ends.length))
                      .fill()
                      .map((_,i) => [new Date(starts[i]), new Date(ends[i])]);

                return pairs;
            },

            async set(val) {
                const pairs = val.filter(pair => !!pair[0] && !!pair[1]);

                const starts = [];
                const ends = [];

                for(let pair of pairs) {
                    starts.push(await this.datetime_repr(pair[0]));
                    ends.push(await this.datetime_repr(pair[1]));
                }

                this.value.start_datetimes = starts;
                this.value.end_datetimes = ends;
            },
        },

        begin_datetime: {
            get() {
                const l = this.value.start_datetimes.length;
                if(l > 0) {
                    return new Date(this.value.start_datetimes[0].substring(0, 19));
                }
                else {
                    return null;
                }
            },

            async set(val) {
                if(val === null) {
                    this.value.start_datetimes = [];
                }
                else {
                    let repr = await this.datetime_repr(val);
                    if(repr != null) {
                        this.value.start_datetimes = [repr];
                    }
                }
            },
        },

        end_datetime: {
            get() {
                const l = this.value.end_datetimes.length;
                if(l > 0) {
                    return new Date(this.value.end_datetimes[l - 1].substring(0, 19));
                }
                else {
                    return null;
                }
            },

            async set(val) {
                if(val === null) {
                    this.value.end_datetimes = [];
                    this.value.has_end = false;
                }
                else {
                    let repr = await this.datetime_repr(val, '23:59:59.999');
                    if(repr != null) {
                        this.value.end_datetimes = [repr];
                        this.value.has_end = true;
                    }
                }
            },
        },

        location: {
            get() {
                if(this.value.is_online && (this.value.location_type == 'at' || this.value.location_type == 'near')) {
                    return 'both';
                }
                else if(this.value.is_online) {
                    return 'online';
                }
                else if(this.value.location_type == 'at' || this.value.location_type == 'near') {
                    return 'physical';
                }
                else {
                    return null;
                }
            },

            set(val) {
                if(val == 'both') {
                    this.value.is_online = true;
                    this.value.location_type = 'near';
                }
                else if(val == 'online') {
                    this.value.is_online = true;
                    this.value.location_type = 'any';
                    this.value.location_name = '';
                    this.value.location_point = null;
                    this.value.location_polygon = null;
                }
                else if(val == 'physical') {
                    this.value.is_online = false;
                    this.value.location_type = 'near';
                }
                else {
                    console.error("Unrecognized location value: " + val);
                }
            },
        },
    },

    watch: {
        "value.timezone": async function(val) {
            let starts = [];
            let ends = [];

            for(let dt of this.value.start_datetimes) {
                let offset = await this.offset_on_day(dt.substring(0, 10), val);
                starts.push(dt.substring(0, 19) + offset);
            }

            for(let dt of this.value.end_datetimes) {
                let offset = await this.offset_on_day(dt.substring(0, 10), val);
                ends.push(dt.substring(0, 19) + offset);
            }

            this.value.start_datetimes = starts;
            this.value.end_datetimes = ends;
        },

        partner(val, old) {
            this.value.partner = val.uid;
            this.value.partner_name = val.name;
            this.value.partner_url = val.url;
            this.value.partner_logo_url = val.image_url;
        },

        value: {
            handler() {
                if(this.skip_updates > 0) {
                    this.skip_updates -= 1;
                }
                else {
                    this.saveState = '';
                    if(!this.editMode) {
                        clearTimeout(this.timeout);
                        this.timeout = setTimeout(() => this.save(true), 5000);
                    }
                }
            },
            deep: true
        },
    },

    methods: {
        initialize() {
            this.value.partner = this.partner.uid;
            this.value.partner_name = this.partner.name;
            this.value.partner_url = this.partner.url;
            this.value.partner_logo_url = this.partner.image_url;

            if(!this.value.timezone) {
                this.value.timezone = Intl.DateTimeFormat().resolvedOptions().timeZone;
            }

            this.learn = this.value.organization_website ? 'link' : 'none';
            this.when = this.value.start_datetimes.length ? 'time' : 'ongoing';
            this.time_periods_dates = this.time_periods.map(pair => pair[0]);
        },

        time_periods_dates_set(val) {
            const current = this.time_periods;
            const updated = [];

            for(let pair of current) {
                let idx = val.indexOf(pair[0]);

                if(idx < 0) {
                    continue;
                }
                else {
                    val.splice(idx, 1);
                    updated.push(pair);
                }
            }

            for(let dt of val) {
                const end = new Date(dt);
                end.setHours(23);
                end.setMinutes(59);
                end.setSeconds(59);
                end.setMilliseconds(999)
                updated.push([dt, end]);
            }

            this.time_periods = updated;
        },

        invalid(name, invalid) {
            if(invalid) {
                this.$set(this.validation, name, 'is-danger');
            }
            else {
                this.$set(this.validation, name, undefined);
            }

            return invalid;
        },

        validate_state(state) {
            let valid = true;

            if(state == 0 || state == 1) {
                if(this.invalid('title', !this.value.title)) valid = false;
                if(this.invalid('organization_name', !this.value.organization_name)) valid = false;
                if(this.invalid('location', !this.location)) valid = false;
                if(this.invalid('partner_opp_url', this.location === 'online' && (!this.value.partner_opp_url || !this.value.partner_opp_url.startsWith('http')))) valid = false;
                if(this.invalid('partner_opp_url', this.location === 'both' && (!this.value.partner_opp_url || !this.value.partner_opp_url.startsWith('http')))) valid = false;
                if(this.invalid('location_name', this.location === 'physical' && !this.value.location_name)) valid = false;
                if(this.invalid('location_name', this.location === 'both' && !this.value.location_name)) valid = false;
                if(this.invalid('organization_website', this.learn === 'link' && (!this.value.organization_website || !this.value.organization_website.startsWith('http')))) valid = false;
                if(this.invalid('begin_datetime', this.when === 'time' && this.time_periods.length < 1)) valid = false;
            }

            if(state == 0 || state == 2) {
                if(this.invalid('short_desc', !this.value.short_desc)) valid = false;
                if(this.invalid('description', !this.value.description)) valid = false;
                if(this.invalid('tags', !this.value.tags.length)) valid = false;
                if(this.invalid('opp_descriptor', !this.value.opp_descriptor.length)) valid = false;
            }

            return valid;
        },

        go_state(new_state) {
            if(new_state < this.state) {
                window.scrollTo(0, 0);
                this.state = new_state;
            }
            else if(this.validate_state(this.state)) {
                window.scrollTo(0, 0);
                this.state = new_state;
            }
            else {
                this.$buefy.toast.open({
                    duration: 5000,
                    message: `Please fill in all of the required fields`,
                    position: 'is-bottom',
                    type: 'is-danger'
                });
            }
        },

        async save(quiet) {
            if(this.value.uid == "00000000-0000-0000-0000-000000000000") {
                if(!this.validate_state(this.state) && this.validation.title) {
                    if(!quiet) {
                        this.$buefy.dialog.alert("We need at least an opportunity name before we can save.");
                    }
                    return;
                }
            }
            else if(!this.validate_state(this.editMode ? 0 : this.state)) {
                this.saveState = 'error';
                if(!this.quiet) {
                    this.$buefy.toast.open({
                        duration: 5000,
                        message: `Please fill in all of the required fields`,
                        position: 'is-bottom',
                        type: 'is-danger'
                    });
                }
                return;
            }

            this.saveState = 'saving';
            try {
                if(this.value.uid == "00000000-0000-0000-0000-000000000000") {
                    const opp = await this.$axios.$post('/api/ui/opportunity/', this.value, this.$store.state.auth);
                    this.skip_updates += 1;
                    this.$emit('input', opp);
                    this.saveState = 'saved';
                    return true;
                }
                else {
                    const opp = await this.$axios.$put('/api/ui/opportunity/' + this.value.uid, this.value, this.$store.state.auth);
                    this.skip_updates += 1;
                    this.$emit('input', opp);
                    this.saveState = 'saved';
                    return true;
                }
            }
            catch(err) {
                this.saveState = 'error';
                if(!quiet) {
                    this.$buefy.dialog.alert("Couldn't save. Make sure you have all the required fields filled in.")
                }
                return false;
            }
        },

        async save_and_my_opps() {
            if(await this.save()) {
                this.$router.push(this.partner !== null ? {name: 'exchange-uid-opps', params: {uid: this.partner.uid}} : {name: 'my-opportunities'});
            }
        },

        async save_and_publish() {
            this.value.withdrawn = false;
            if(await this.save()) {
                this.$router.push(this.partner !== null ? {name: 'exchange-uid-slug', params: {uid: this.partner.uid, slug: this.value.slug}} : {name: 'slug', params: {slug: this.value.slug}});
            }
        },

        async save_and_view() {
            if(await this.save()) {
                this.$router.push(this.partner !== null ? {name: 'exchange-uid-slug', params: {uid: this.partner.uid, slug: this.value.slug}} : {name: 'slug', params: {slug: this.value.slug}});
            }
        },

        async time_periods_set(idx, cell, datetime) {
            // const val = [...this.time_periods];
            // val[idx][cell] = datetime;
            // this.time_periods = val;
            if(cell == 0) {
                this.value.start_datetimes.splice(idx, 1, await this.datetime_repr(datetime));
            }
            else if(cell == 1) {
                this.value.end_datetimes.splice(idx, 1, await this.datetime_repr(datetime));
            }
        },

        async datetime_repr(datetime, default_time, override_timezone) {
            if(datetime === null || datetime === undefined) {
                return null;
            }

            if(!this.value.timezone) {
                return null;
            }

            return this.build_datetime(
                this.date_part(datetime),
                this.time_part(datetime, default_time),
                await this.offset_on_day(datetime, override_timezone ? override_timezone : this.value.timezone),
            );
        },

        async offset_on_day(date, timezone) {
            let zone = await this.$axios.$get('/api/ui/timezone?name=' + timezone + '&date=' + (date.getFullYear ? this.date_part(date) : date), this.$store.state.auth);
            return zone.offset;
        },

        build_datetime(date, time, offset) {
            return '' + date + 'T' + time + offset;
        },

        offset_part(date) {
            let minutes = -date.getTimezoneOffset();
            let hh = Math.abs(minutes) / 60;
            let mm = Math.abs(minutes) % 60;

            return ((minutes < 0) ? '-' : '+') + ('00' + hh).slice(-2) + ':' + ('00' + mm).slice(-2);
        },

        time_part(date, fallback) {
            let ret =  '' + ('00' + date.getHours()).slice(-2) + ':' + ('00' + date.getMinutes()).slice(-2) + ':' + ('00' + date.getSeconds()).slice(-2);
            if(fallback !== undefined && ret == '00:00:00') {
                return fallback;
            }
            return ret;
        },

        date_part(date) {
            return '' + date.getFullYear() + '-' + ('00' + (date.getMonth() + 1)).slice(-2) + '-' + ('00' + date.getDate()).slice(-2);
        },

        location_point(point) {
            this.value.location_type = 'near';
            this.value.location_point = point;
            this.value.location_polygon = null;
        },

        location_poly(poly) {
            this.value.location_type = 'near';
            this.value.location_point = null;
            this.value.location_polygon = poly;
        },

        location_license(text) {
            this.value.extra_data.location_license = text;
        },
    },
}
</script>

<style lang="scss">

.validation-target {
    border: 1px solid transparent;
    border-radius: 6px;
    &.is-danger {
        border: 1px solid #f14668;
    }
}

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
        display:flex;
        line-height:1;
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
            flex-shrink:0;
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
  .hashtags.control {
    width:100%;
  }

}
@media (max-width:1124px){
  .times-flex .flex {
    flex-direction:column;
  }
}

@media (max-width:799px) {
  #modal-dates {
    flex-direction:column;
  }
  #time-periods {
    margin-left:0;
  }
}

@media (max-width:600px) {
  .add {
    margin-left: 0;
  }
  .form-actions {
    button {
      margin:10px 2px!important;
      padding: 12px 12px!important;
    }
  }
}

.display-image-wrapper {
    flex-direction: row-reverse;

    >:first-child {
        flex-grow: 1;
        margin-left: 2rem;
    }
}

@media (max-width:560px) {
  .display-image-wrapper {
    flex-direction:column;
  }
}

@media (max-width: 480px){
  .times-flex .flex .field-body .field.has-addons {
    flex-direction:column!important;
    .datepicker {
      margin-right:0!important;
      margin-bottom:10px;
    }
  }
  .form-actions {
    button {
      margin:10px 2px 10px 0!important;
      padding: 12px 8px!important;
    }
  }
}

@media (max-width:460px) {
  .tp-item .flex {
    flex-direction:column;
  }
  #time-periods .tp-item .timepicker:first-child {
    margin-right:0;
  }
  .save-feedback span {
    display:none;
  }
}



</style>
