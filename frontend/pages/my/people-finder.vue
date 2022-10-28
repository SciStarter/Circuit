<template>
  <div class="people-finder snm-container">

    <template v-if="state=='home'">
     <div class="flex-header">
       <h2>Promote Your Opportunities</h2>
       <action-button principal @click="state='create'"><div class="icon"><add-icon /></div>Create a Promotion</action-button>
     </div>

     <div v-if="showLimit" class="snm-container snm-alert">
       <b-notification
               type="is-warning" :closable="false">
               <p>You have passed your limit of 1 free promotion per year. To upgrade your account to allow more promotions, please contact <a href="mailto:support@scienceenearme.org">support@scienceenearme.org</a>.</p>
           </b-notification>
     </div>

     <div v-if="showSuccess" class="snm-container snm-alert">
       <b-notification
               type="is-warning"
               aria-close-label="Close notification">
               <p>Your promotion is being reviewed by Science Near Me.</p>
           </b-notification>
     </div>

     <div class="pf-latest" v-if="promotions.completed.length > 0">
       <h2>Latest Promotion</h2>
       <div class="flex space-between">
             <div>
               <a :href="promotions.completed[0].link">{{promotions.completed[0].name}}</a>
             </div>
             <div>
               {{promotions.completed[0].date}} | {{promotions.completed[0].length}}
             </div>
         </div>
         <div class="pf-figures">
             <div>
               <h3>Website Views</h3>
               <em>{{promotions.completed[0].w_views}}</em>
             </div>
             <div>
               <h3>Website Clicks</h3>
               <em>{{promotions.completed[0].w_clicks}}</em>
               <span>({{wOpenPercent}}%)</span>
             </div>
             <div>
               <h3>Email Sends</h3>
               <em>{{promotions.completed[0].e_sends}}</em>
             </div>
             <div>
               <h3>Email Opens</h3>
               <em>{{promotions.completed[0].e_opens}}</em>
               <span>({{eOpenPercent}}%)</span>
             </div>
             <div>
               <h3>Email Clicks</h3>
               <em>{{promotions.completed[0].e_clicks}}</em>
               <span>({{eClickPercent}}%)</span>
             </div>
         </div>
     </div>

     <div class="pf-pending mb15" v-if="promotions.pending.length > 0" >
         <h2>Pending Promotions</h2>
         <b-table 
             :data="promotions.pending"
             :sort-icon="sortIcon"
             :sort-icon-size="sortIconSize"
             :default-sort="['date','desc']">
             <b-table-column field="name" label="Opportunity" width="200" sortable v-slot="props" header-class="pf-table-header">
               <a :href="props.row.link" class="pf-name">{{ props.row.name }}</a>
             </b-table-column>
             <b-table-column field="status" label="Status" v-slot="props" header-class="pf-table-header">
               <span class="pf-status" :class="{'waiting':props.row.status=='waiting for approval'}">{{ props.row.status }}</span>
             </b-table-column>
             <b-table-column field="date" label="Start Date" sortable v-slot="props" header-class="pf-table-header">
                 {{ props.row.date }}
             </b-table-column>
             <b-table-column field="length" label="Length" v-slot="props" header-class="pf-table-header">
                 {{ props.row.length }}
             </b-table-column>
             <b-table-column v-slot="props">
               <b-dropdown aria-role="menu" position="is-bottom-left">
                 <template #trigger="{ active }">
                     <b-button
                         :icon-right="active ? 'menu-up' : 'dots-vertical'" />
                 </template>
                       <b-dropdown-item aria-role="menuitem">View Promotion</b-dropdown-item>
                       <b-dropdown-item aria-role="menuitem">Duplicate Promotion</b-dropdown-item>
                   </b-dropdown>
             </b-table-column>
         </b-table>
    </div>

    <div class="pf-completed mb15">
         <h2>Completed Promotions</h2>
         <b-table v-if="promotions.completed.length > 0" 
             :data="promotions.completed"
             :sort-icon="sortIcon"
             :sort-icon-size="sortIconSize"
             :default-sort="['date','desc']">
             <b-table-column field="name" label="Opportunity" width="200" sortable v-slot="props" header-class="pf-table-header">
               <a :href="props.row.link" class="pf-name">{{ props.row.name }}</a>
             </b-table-column>
             <b-table-column field="date" label="Date Started" sortable v-slot="props" header-class="pf-table-header">
                 {{ props.row.date }}
             </b-table-column>
             <b-table-column field="length" label="Length" v-slot="props" header-class="pf-table-header">
                 {{ props.row.length }}
             </b-table-column>
             <b-table-column field="w_views" label="Website Views" v-slot="props" header-class="pf-table-header">
                 {{ props.row.w_views }}
             </b-table-column>
             <b-table-column field="w_clicks" label="Website Clicks" v-slot="props" header-class="pf-table-header">
                 {{ props.row.w_clicks }}
             </b-table-column>
             <b-table-column field="e_sends" label="Email Sends" v-slot="props" header-class="pf-table-header">
                 {{ props.row.e_sends }}
             </b-table-column>
             <b-table-column field="e_opens" label="Email Opens" v-slot="props" header-class="pf-table-header">
                 {{ props.row.e_opens }}
             </b-table-column>
             <b-table-column field="e_clicks" label="Email Clicks" v-slot="props" header-class="pf-table-header">
                 {{ props.row.e_clicks }}
             </b-table-column>
             <b-table-column v-slot="props">
               <b-dropdown aria-role="menu" position="is-bottom-left">
                 <template #trigger="{ active }">
                     <b-button
                         :icon-right="active ? 'menu-up' : 'dots-vertical'" />
                 </template>
                       <b-dropdown-item aria-role="menuitem">View Promotion</b-dropdown-item>
                       <b-dropdown-item aria-role="menuitem">Duplicate Promotion</b-dropdown-item>
                   </b-dropdown>
             </b-table-column>
         </b-table>
         <p v-else>You have no completed promotions.</p>
    </div>
    </template>


    <template v-if="state=='create'">
      <div class="flex-header create-header">
       <h2>Promote Your Opportunities</h2>
       <action-button disabled><div class="icon"><add-icon /></div>Create a Promotion</action-button>
     </div>
     <h3>Create a Promotion</h3>
     
     <div class="flex pf-card-wrap">

        <div class="pf-card pf-card1" v-if="formState==1">
          <h3>Select Your Opportunity</h3>
          <b-field>
            <b-select
                placeholder="Select an Opportunity" expanded v-model="newPromo.opp">
                <option value="Their Opp 1">Their Opp 1</option>
                <option value="Their Opp 2">Their Opp 2</option>
            </b-select>
        </b-field>
        <div class="pf-button-flex">
          <action-button tertiary @click="state='home'">Back</action-button>
          <action-button disabled v-if="!newPromo.opp">Next</action-button>
          <action-button primary v-else @click="formState=2">Next</action-button>
        </div>
        </div>

        <div v-if="formState==2" class="pf-full">
          <opportunity-card v-for="opp in matches" :key="opp.uid" :opportunity="opp" />

          <div class="pf-card pf-card2">
            <div class="pf-card-header flex">
              <h3>Select an Audience Radius</h3>
              <a>select by state</a>
            </div>
            <div class="pf-map">
              <div class="pf-map-info pf-card">Click or tap to select the area your interested in</div>
            </div>
            <div class="flex pf-fill">
              <div>
                <p class="pf-label">Location Center</p>
                <p>{{newPromo.location.name}}</p>
              </div>
              <div>
                <p class="pf-label">Radius (max 150 miles)</p>
                <p>{{newPromo.location.radius}}</p>
              </div>
            </div>
            <div class="pf-time pf-lined">
                <h3>Promotion Dates</h3>
                <div class="flex">
                  <b-field label="Select Start Date & Time">
                    <b-datetimepicker
                  `       placeholder="Type or select a date..."
                          icon="calendar-today"
                          :locale="locale"
                          editable>
                      </b-datetimepicker>
                  </b-field>
                  <b-field label="Select Length">
                    <b-select placeholder="Select a length">
                        <option>1 day</option>
                        <option>2 days</option>
                        <option>3 days</option>
                    </b-select>
                </b-field>
                </div>
            </div>
            <div class="pf-lined">
              <div class="pf-button-flex">
                <action-button tertiary @click="formState=1">Back</action-button>
                <action-button disabled v-if="!newPromo.opp">Next</action-button>
                <action-button primary v-else @click="formState=3">Next</action-button>
              </div>
            </div>
          </div>
        </div>

        <div v-if="formState==3" class="pf-full">
          <opportunity-card v-for="opp in matches" :key="opp.uid" :opportunity="opp" />
          <div class="pf-card pf-card3">
          <div class="flex pf-fill">
              <div>
                <p class="pf-label">Audience Location</p>
                <p>{{newPromo.location.name}}</p>
                <p>{{newPromo.location.radius}}</p>
              </div>
              <div>
                <p class="pf-label">Promotion Date</p>
                <p>{{newPromo.date}}</p>
                <p>{{newPromo.length}}</p>
              </div>
            </div>
          </div>
          <div class="pf-message flex pf-card">
            <div class="pf-message-form">
              <h3>Campaign</h3>
              <b-field label="Title">
                  <small>The title will serve as the header on the web promotion and as the subject of the email sent to Science Near Me users.</small>
                  <b-input v-model="newPromo.title" expanded></b-input>
              </b-field>
              <b-field label="Message">
                  <small>Limit 600 characters.</small>
                  <b-input v-model="newPromo.message" expanded maxlength="600" type="textarea"></b-input>
              </b-field>
              <b-field label="URL">
                  <small>A URL to find out more. Must begin with http:// or https://</small>
                  <b-input v-model="newPromo.url" expanded></b-input>
              </b-field>
            </div>
            <div class="pf-message-preview">
              <h3>Preview</h3>
              <!-- I took this out as I don't think there really is any difference between web and email preview. Is there??-->
              <!-- <div class="nav-tab-wrapper">
                <ul class="nav-tabs">
                    <li><a class="tab-link":class="{'active':preview=='web'}" @click="preview='web'">Web Preview</a></li>
                    <li><a class="tab-link":class="{'active':preview=='email'}" @click="preview='email'">Email Preview</a></li>
                </ul>
              </div> -->
              <div v-if="preview=='web'">
                <h4>{{newPromo.title}}</h4>
                <opportunity-card v-for="opp in matches" :key="opp.uid" :opportunity="opp" />
                <pre>{{newPromo.message}}</pre>
                <action-button principal :href="newPromo.url" v-if="newPromo.url">Find Out More</action-button>
              </div>
              <!-- <div v-if="preview=='email'">
                <h4>{{newPromo.title}}</h4>
              </div> -->
            </div>
            <div class="pf-lined">
              <div class="pf-button-flex">
                <action-button tertiary @click="formState=2">Back</action-button>
                <action-button disabled v-if="!newPromo.title || !newPromo.message || !newPromo.url">Finish</action-button>
                <action-button primary v-else @click="submitPromo">Finish</action-button>
              </div>
            </div>
          </div>
        </div>


     </div>

   </template>


  </div> 
</template>

<script>

import AddIcon from '~/assets/img/submit-opportunity.svg?inline'
import FilterIcon from '~/assets/img/filter.svg?inline'

export default {
   name: "MyPeopleFinder",

   components: {
       AddIcon,
       FilterIcon
   },


   // httpHeaders() {
   //     return {
   //         'X-XSS-Protection': '1; mode=block',
   //         'X-Frame-Options': 'DENY',
   //         'X-Content-Type-Options': 'nosniff',
   //         'Referrer-Policy': 'same-origin',
   //     };
   // },


   data() {
       return {
           state:'home', // 'create' is for the form
           showLimit: false,
           showSuccess: false,
           formState:1,
           promotions:{
             completed: 
             [
               {
                 name: 'Astrophotography SIG Meeting of Some Kind',
                 link: '/link',
                 date: '12/23/22',
                 length: '3 days',
                 w_views: 3345,
                 w_clicks: 234,
                 e_sends: 897,
                 e_opens: 345,
                 e_clicks: 452,
               },
               {
                 name: 'Astrophotography SIG Meeting of Some Kind',
                 link: '/link',
                 date: '2/23/22',
                 length: '3 days',
                 w_views: 3345,
                 w_clicks: 234,
                 e_sends: 897,
                 e_opens: 345,
                 e_clicks: 452,
               },
               {
                 name: 'Astrophotography SIG Meeting of Some Kind',
                 link: '/link',
                 date: '12/23/21',
                 length: '3 days',
                 w_views: 3345,
                 w_clicks: 234,
                 e_sends: 897,
                 e_opens: 345,
                 e_clicks: 452,
               },
             ],
             pending: 
             [
             {
                 name: 'Astrophotography SIG Meeting of Some Kind',
                 link: '/link',
                 date: '12/23/21',
                 length: '3 days',
                 w_views: 3345,
                 w_clicks: 234,
                 e_sends: 897,
                 e_opens: 345,
                 e_clicks: 452,
                 status: 'waiting for approval'
               },
               {
                 name: 'Astrophotography SIG Meeting of Some Kind',
                 link: '/link',
                 date: '12/23/21',
                 length: '3 days',
                 w_views: 3345,
                 w_clicks: 234,
                 e_sends: 897,
                 e_opens: 345,
                 e_clicks: 452,
                 status: 'approved, scheduled'
               },
             ],
           },
           newPromo: {
            opp: null,
            location: {
              name: 'Iowa City, IA',
              lat: null,
              lng: null,
              radius: '34 miles'
            },
            date: '11/14/2022',
            length: '3 days',
            title: null,
            message: null,
            url: null
           },
           preview: 'web'

       }
   },
   computed: {
     wOpenPercent(){
       if (this.promotions.completed.length > 0)
        return parseFloat(((this.promotions.completed[0].w_clicks/this.promotions.completed[0].w_views) * 100).toFixed(2));
     },
     eOpenPercent(){
       if (this.promotions.completed.length > 0)
        return parseFloat(((this.promotions.completed[0].e_opens/this.promotions.completed[0].e_sends) * 100).toFixed(2));
     },
     eClickPercent(){
       if (this.promotions.completed.length > 0)
        return parseFloat(((this.promotions.completed[0].e_clicks/this.promotions.completed[0].e_sends) * 100).toFixed(2));
     }
   },
   methods: {
    submitPromo(){
      this.formState = 1;
      this.showSuccess = true;
      this.state = 'home';
      window.scrollTo(0,0);
    }
   }

  
}
</script>

<style lang="scss" scoped>
.flex {
 display:flex;
}
.flex-header {
 display:flex;
 align-items:center;
 justify-content:space-between;
 margin-bottom:1rem;

 h2 {
   color: var(--secondary-color, $snm-color-element-med);
   font-size:1.4rem;
   font-weight:bold;
   font-family: $snm-font-heading;
 }
}

.snm-alert {
 margin-bottom:1.5rem;
}

.pf-latest {
 border-radius: 6px;
 border: 1px solid #D0D0D0;
 background-color: #EDEDED;
 padding: 1rem;
 margin-bottom: 1.5rem;
}

h1 {
 font-family: $snm-font-heading;
 font-size: 1.8rem;
 font-weight:bold;
 color: var(--secondary-color, $snm-color-element-med);
 margin-bottom:0;
}

.pf-pending {
 padding-bottom: 1.5rem;
 border-bottom: 2px solid $snm-color-element-dark;
}
.pf-pending, .pf-completed {
  h2 {
    margin-left:12px;
  }
}

.pf-status {
 font-weight: bold;
 &.waiting {
   color: $snm-color-info;
 }
}

.mb15 {
 margin-bottom:1.5rem;
}
.space-between {
 justify-content: space-between;
}
.people-finder {
 h2 {
   font-weight: bold;
   font-size: $snm-font-medium;
   font-family: $snm-font-heading;
 }
 h3 {
   font-weight: bold;
   font-size: $snm-font-medium-small;
   font-family: $snm-font-heading;
   margin-bottom: 10px;
 }
}
:deep(.pf-table-header){
   font-size: 0.8rem!important;
 }

 .pf-card :deep(.label) {
  font-size: 0.8rem!important;
 }


.pf-figures {
 display: flex;
 justify-content: space-between;
 > div {
     border: 1px solid #D0D0D0;
     background-color: white;
     text-align: center;
     padding: 0.5rem;
     border-radius: 6px;
     width: 20%;
     margin: 10px;
     &:first-child {
       margin-left:0;
     }
     &:last-child {
       margin-right:0;
     }
     h3 {
       font-weight: bold;
       font-size: 1rem;
     }
     em,span {
       font-style: normal;
       font-size: 2rem;
       font-weight: bold;
       color: $snm-color-background-meddark;
       display: block;
     }
     span {
       font-size: 1rem;
     }
 }
}

.create-header {
  border-bottom: 1px solid #CBCBCB;
  padding-bottom: 1rem;
}
.pf-full {
  width: 100%;
}

.pf-fill > div {
  flex-grow:1;
  flex-basis:0;
}

.pf-card {
  border:1px solid #D0D0D0;
  box-shadow: 0 2px 6px rgba(0,0,0,.2);
  padding: 1rem;
  border-radius: 6px;
  margin-bottom: 1.5rem;
  width: 100%;
}

.pf-card-wrap {
  justify-content: center;
  align-items: center;
}


.pf-card1 {
  min-width: 350px;
  width:auto;
}

.pf-lined {
    margin-left: -1rem;
    margin-right: -1rem;
    padding: 1rem;
    border-top: 1px solid #d0d0d0;
    margin-top:1rem;
  }

.pf-button-flex {
  display: flex;
  justify-content: space-between;
  margin: 0 -0.5rem;

  :first-child {
    flex-grow:0;
  }
  :last-child {
    flex-grow:1;
  }
}

.pf-card-header {
  align-items: baseline;
}
.pf-card-header h3 {
  margin-right: 1rem;
}

.pf-map {
  width:100%;
  min-height: 500px;
  border: 1px solid #d0d0d0;
  position: relative;
  margin-bottom: 1rem;
  .pf-map-info {
    position: absolute;
    top:10px;
    right:10px;
    width:auto;
  }
}

.pf-time .flex > div:first-child {
  margin-right: 1rem;
}

.pf-label {
  font-weight: bold;
  font-size: .8rem;
}

.pf-card2 .pf-button-flex {
    max-width:350px;
    margin:0 auto;
}

.pf-card3 .pf-fill > div:last-child {
  border-left: 1px solid #D0d0d0;
  margin-top: -1rem;
  margin-bottom: -1rem;
  padding:1rem;
}

.pf-message {
  flex-wrap: wrap;
  padding-right: 0;
  .pf-lined {
    width: calc(100% + 1rem);
    display: flex;
    justify-content: center;
    .pf-button-flex {
      max-width:350px;
      width:100%;
    }
  }
}
.pf-message h3 {
  margin-bottom: 2rem;
}
.pf-message-form {
  width: 33.33%;
  padding-right: 1rem;
  small {
    display:block;
    margin-top:-.5rem;
    color: #8D8D8D;
    font-size:.8rem;
  }
  :deep(.field.has-addons) {
    display:block;
  }
  :deep(input) {
    border-radius:6px!important;
  }
}

.pf-message-preview {
  flex-grow:1;
  border-left: 1px solid #d0d0d0;
  margin-top:-1rem;
  margin-bottom: -1rem;
  padding:1rem;

  h4 {
    font-weight: bold;
    font-family: $snm-font-heading;
    font-size: 1.2rem;
    color: $snm-color-background-meddark;
    margin-bottom: 1rem;
  }
  pre {
    background-color: transparent;
    padding:0;
    font-family: $snm-font-content;
    font-size: 1rem;
    margin-bottom: 1.5rem;
  }
  button {
    margin: 0 auto;
    display: block;
    min-width: 300px;
  }
}


@media (min-width:769px){
 .pf-name {
 text-overflow: ellipsis;
   width: 200px;
   white-space: nowrap;
   overflow: hidden;
   display: block;
 }
}




@media (max-width:768px){
  .pf-figures {
    > div {
      margin:5px;
      h3 {
       font-size: 0.8rem;
      }
    }
  }
  .pf-pending, .pf-completed {
  h2 {
    margin-left:0;
    margin-bottom: 10px;
  }
}
}
@media (max-width:530px){
  .flex-header {
    flex-direction: column;
  }
  .pf-figures {
    flex-wrap: wrap;
    justify-content: center;
    > div {
      width: 30%;
      
      &:nth-child(2), &:first-child {
        width:46%;
      }
      &:nth-child(2) {
        margin-right: 0;
      }
      &:nth-child(3) {
        margin-left: 0;
      }
    }
  }
  .pf-latest .flex.space-between {
    display: block;
  }
}

@media (max-width:767px){
  .pf-message {
    flex-direction: column;
    .pf-message-form {
      width: 100%;
    }
    .pf-message-preview {
      margin: 0;
      border: 0;
      margin-top: 2rem;
      border-top: 1px solid #d0d0d0;
      padding-top: 2rem;
      margin-left: -1rem;
    }
  }
}


@media (max-width:600px){
  .pf-card {
    width: 100%;
  }
  .pf-fill {
    flex-direction: column;
  }
  .pf-time .flex {
    flex-direction: column;
  }
  .pf-card3 .pf-fill > div:last-child {
    margin:0;
    padding:0;
    border:0;
    margin-top: 1rem;
  }
}


@media (max-width:1159px) {
 .snm-container {
   padding:1rem;
 }
 .flex-header.filter-actions {
   flex-direction:column;
   align-items: flex-start;
   .header-actions > div:first-child {
     margin-left:0;
   }
   h2 {
     margin-bottom:1rem;
   }
 }
}

@media (max-width:767px) {
 #results  {
   margin-left:-1rem;
   margin-right:-1rem;
 }
}

.filter {
 display:none;
}

@media (max-width:600px) {
 .filter-area {
   width:100%;
   justify-content:space-between;
   h2 {
     font-size:18px;
   }
 }
 .filter {
   display:block;
   width:30px;
   height:30px;
   path {
     fill: $snm-color-background-meddark
   }
 }
 .header-actions {
   top:0;
   left:0;
   background-color:$snm-color-background-medlight;
   padding:20px;
   flex-wrap: wrap;
   justify-content:space-between;
   display:none;
   > div:first-child {
     min-width:100%!important;
   }
   .field.is-floating-label, .field.is-floating-in-label{
     margin:10px 0;
     width:48%;
     .datepicker {
       width:100%;
     }
   }
   .mobile-fix {
     margin:auto;
   }
   &.show_filters {
     display:flex;
   }
 }
 .add-btn {
   display:none!important;
 }
 .no-results {
   padding:20px;
 }
}

.nav-tab-wrapper {
 width:100%;
 overflow:auto;
 .nav-tabs {
   min-width: 680px
 }
}
.nav-tab-wrapper::-webkit-scrollbar {
 display: none;
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
</style>
