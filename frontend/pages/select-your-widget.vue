<template>
  <div class="snm-wrapper">
  <div class="snm-container base-typography snm-container-first">
    <div class="head">
      <h1>Display a Widget</h1>
      <p>Select and customize a widget to display Science Near Me projects or the Science Near Me finder below and add to your website!</p>
    </div>

    <div class="flex">
        <div class="flex1">

          <section>
            <h2>Select Which Type of Widget You'd Like</h2>
            <div>
              <b-field>
                 <b-radio v-model="widgetType"
                     native-value="project">
                     Show one or more science opportunities
                 </b-radio>
               </b-field>
               <b-field>
                  <b-radio v-model="widgetType"
                      native-value="finder">
                      Show the finder to allow people to search science opportunities
                  </b-radio>
                </b-field>
              </div>
          </section>

          <section v-if="widgetType=='project'">
            <h2>Select Your Widget Style</h2>
            <p>Make selections below on how to display science opportunities.</p>
            <div class="radio-selects">
              <h3>Header</h3>
              <b-field>
                 <b-radio v-model="header"
                     native-value="header">
                     With Header
                 </b-radio>
               </b-field>
               <b-field>
                  <b-radio v-model="header"
                      native-value="no-header">
                      Without a Header
                  </b-radio>
                </b-field>
            </div>

            <div class="radio-selects">
              <h3>How many opportunities should be displayed?</h3>
              <p>You may display up to 10 science opportunities.</p>
              <b-field>
                <b-numberinput v-model="max" min="1" max="10"  controls-position="compact"></b-numberinput>
              </b-field>
            </div>

            <div class="radio-selects">
              <h3>Widget Size</h3>
              <b-field>
                 <b-radio v-model="projectSize"
                     native-value="short-thin">
                     Short and Thin
                 </b-radio>
               </b-field>
               <b-field>
                  <b-radio v-model="projectSize"
                      native-value="tall-thin">
                      Tall and Thin
                  </b-radio>
                </b-field>
                <b-field>
                   <b-radio v-model="projectSize"
                       native-value="short-wide">
                       Short and Wide
                   </b-radio>
                 </b-field>
                 <b-field>
                    <b-radio v-model="projectSize"
                        native-value="tall-wide">
                        Tall and Wide
                    </b-radio>
                  </b-field>
              </div>

              <div class="radio-selects">
                <h3>Customization</h3>
                <b-field>
                   <b-radio v-model="customize"
                       native-value="no">
                       Select from all available opportunities on Science Near Me
                   </b-radio>
                 </b-field>
                 <b-field>
                    <b-radio v-model="customize"
                        native-value="yes">
                        Filter and customize opportunities
                    </b-radio>
                  </b-field>

                  <div v-if="customize=='yes'">
                    <div class="nested">
                      <h4>Kid-Friendly Only</h4>
                      <b-field>
                        <b-checkbox v-model="filters.kid">Show only kid-friendly opportunities</b-checkbox>
                      </b-field>
                    </div>

                    <div class="nested">
                    <h4>Location</h4>
                    <b-field>
                       <b-radio v-model="filters.location"
                           native-value="global">
                           Global
                       </b-radio>
                     </b-field>
                     <b-field>
                        <b-radio v-model="filters.location"
                            native-value="national">
                            National
                        </b-radio>
                      </b-field>
                      <div v-if="filters.location=='national'" class="nested">
                        <b-field label="Country">
                           <b-select placeholder="Select a Country"></b-select>
                         </b-field>
                      </div>
                      <b-field>
                         <b-radio v-model="filters.location"
                             native-value="regional">
                             Regional
                         </b-radio>
                       </b-field>
                       <div v-if="filters.location=='regional'" class="nested">
                         <b-field label="Country">
                            <b-select placeholder="Select a Country"></b-select>
                          </b-field>
                          <b-field label="State/Province/Region">
                             <b-select placeholder="Select a Region"></b-select>
                           </b-field>
                       </div>
                       <b-field>
                          <b-radio v-model="filters.location"
                              native-value="near">
                              Within 50 miles of a location
                          </b-radio>
                        </b-field>
                        <div v-if="filters.location=='near'" class="nested">
                            put the map here
                        </div>
                  </div>
                  <div class="nested">
                    <h4>Include Online Only Opportunities</h4>
                    <b-field>
                       <b-radio v-model="filters.online"
                           native-value="yes">
                           Include online only opportunities
                       </b-radio>
                     </b-field>
                     <b-radio v-model="filters.online"
                         native-value="no">
                         Do not include online only opportunities
                     </b-radio>
                   </b-field>
                  </div>
                  <div class="nested">
                    <h4>Activity Type</h4>
                    <b-field>
                        <b-checkbox v-model="filters.activities">
                            checkboxes of all activity types here
                        </b-checkbox>
                    </b-field>
                  </div>
                  <div class="nested">
                    <h4>Organization</h4>
                    <p>Limit your results to one or more host organizations. Beging typing the organization and select when it displays in the dropdown menu.</p>

                    <b>[autocomplete of host orgs here. Allow multiselect. Partner orgs on finder filters.]</b>
                    <!-- <b-autocomplete
                      v-model="query.partner_text"
                      :data="suggested_partners"
                      :name="'new-' + Math.random()"
                      field="name"
                      clearable
                      keep-first
                      select-on-click-outside
                      @select="selected_partner = $event"
                      /> -->

                  </div>

                </div>

              </div>
          </section>

          <section v-if="widgetType=='finder'">
            <h2>Select Your Widget Style</h2>
            <div class="radio-selects">
              <h3>Widget Size</h3>
              <b-field>
                 <b-radio v-model="finderSize"
                     native-value="finder-thin">
                     Thin
                 </b-radio>
               </b-field>
               <b-field>
                  <b-radio v-model="finderSize"
                      native-value="finder-wide">
                      Wide
                  </b-radio>
                </b-field>
              </div>
          </section>

        </div>
        <div class="flex2">
          <iframe :src="'/'+URLparams" :width="width" :height="height" scrolling="no"></iframe>
          <textarea>&lt;iframe src="{{link}}" width="{{width}}" height="{{height}}" scrolling="no"&gt;&lt;/iframe&gt;</textarea>

        </div>
    </div>


  </div>
</div>
</template>

<script>
export default {
    data() {
      return {
        widgetType: 'project',
        header: 'header',
        max:1,
        projectSize: 'short-thin',
        customize: 'no',
        finderSize: 'finder-thin',
        filters: {
          kid: false,
          location: 'global',
          address: undefined,
          activities:[],
          online: 'yes'
        },
        sizes:{
          'short-thin':{
            header: {
              width: 200,
              height: 325
            },
            'no-header':
            {
              width: 200,
              height: 275
            }
          },
          'tall-thin':{
            header: {
              width: 200,
              height: 575
            },
            'no-header':{
              width: 200,
              height: 525
            }
          },
          'short-wide':{
            header: {
              width: 375,
              height: 250
            },
            'no-header':{
              width: 375,
              height: 200
            }
          },
          'tall-wide':{
            header: {
              width: 375,
              height: 475
            },
            'no-header':{
              width: 375,
              height: 425
            },
          },
          'finder-thin': {
            width: 200,
            height: 445
          },
          'finder-wide':{
            width: 375,
            height: 310
          }
        }
      }
    },
    computed:{
      URLparams(){
        if (this.widgetType == 'project') {
          return `widget?layout=${this.projectSize}&style=${this.header}&max=${this.max}`
        } else {
          return `widget?layout=${this.finderSize}`
        }
      },
      link(){
        return `https://sciencenearme.org/${this.URLparams}`
      },
      width(){
        if (this.widgetType == 'project') {
          return this.sizes[this.projectSize][this.header].width
        } else {
          return this.sizes[this.finderSize].width
        }
      },
      height(){
        if (this.widgetType == 'project') {
          return this.sizes[this.projectSize][this.header].height
        } else {
          return this.sizes[this.finderSize].height
        }
      }
    }
}
</script>

<style lang="scss" scoped>

.flex {
  flex-direction: column;
}
.flex1, .flex2 {
  flex:1 1 auto;
}

.flex2 {
  display: flex;
  flex-direction:column;
  align-items:center;

  textarea {
    width:100%;
    font-size:16px;
    height:150px;
    margin-top:2rem;
  }
}

.radio-selects {
  border-radius:6px;
  border:1px solid $snm-color-border;
  margin:10px 0;
  padding:10px;
}

section {
  margin-bottom:2rem;
}

.nested {
  padding: 10px 20px;
  margin-bottom:10px;
  border-bottom:1px solid $snm-color-border;
  &:last-child {
    border-bottom:0;
  }
  .nested {
    padding:0 30px;
    border-bottom:0;
    margin-bottom:30px;
    margin-top:-10px;
  }
}

@media (min-width:768px) {

.flex {
  flex-direction: row;
}
.flex1 {
  flex: 1 1 auto;
  padding-right:20px;
}

.flex2 {
  flex:0 0 375px;
  align-self: flex-start;
  position:sticky;
  top:20px;
}
}

</style>
