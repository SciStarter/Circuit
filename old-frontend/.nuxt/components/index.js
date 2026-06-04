export { default as ActionButton } from '../../components/ActionButton.vue'
export { default as ActivityRegional } from '../../components/ActivityRegional.vue'
export { default as AddOpportunities } from '../../components/AddOpportunities.vue'
export { default as BubbleChart } from '../../components/BubbleChart.vue'
export { default as CalendarAdd } from '../../components/CalendarAdd.vue'
export { default as Card } from '../../components/Card.vue'
export { default as ChordDiagram } from '../../components/ChordDiagram.vue'
export { default as ChoroplethStates } from '../../components/ChoroplethStates.vue'
export { default as ComparisonBar } from '../../components/ComparisonBar.vue'
export { default as DynamicBlock } from '../../components/DynamicBlock.vue'
export { default as ExternalLink } from '../../components/ExternalLink.vue'
export { default as FindResults } from '../../components/FindResults.vue'
export { default as Footer } from '../../components/Footer.vue'
export { default as GeneralFilters } from '../../components/GeneralFilters.vue'
export { default as GeoExplorerMapRadius } from '../../components/GeoExplorerMapRadius.vue'
export { default as GeoExplorerMapState } from '../../components/GeoExplorerMapState.vue'
export { default as GeoExplorerMapUsa } from '../../components/GeoExplorerMapUsa.vue'
export { default as JustContent } from '../../components/JustContent.vue'
export { default as LDJson } from '../../components/LDJson.js'
export { default as LineChart } from '../../components/LineChart.vue'
export { default as LoadingSpinner } from '../../components/LoadingSpinner.vue'
export { default as LoginForm } from '../../components/LoginForm.vue'
export { default as LoginScistarterForm } from '../../components/LoginScistarterForm.vue'
export { default as Logo } from '../../components/Logo.vue'
export { default as LookupGeometry } from '../../components/LookupGeometry.vue'
export { default as LookupPlace } from '../../components/LookupPlace.vue'
export { default as MiniSelect } from '../../components/MiniSelect.vue'
export { default as Opportunity } from '../../components/Opportunity.vue'
export { default as OpportunityCalendar } from '../../components/OpportunityCalendar.vue'
export { default as OpportunityCard } from '../../components/OpportunityCard.vue'
export { default as OpportunityForm } from '../../components/OpportunityForm.vue'
export { default as OpportunityKeywords } from '../../components/OpportunityKeywords.vue'
export { default as OpportunityList } from '../../components/OpportunityList.vue'
export { default as OpportunityLocation } from '../../components/OpportunityLocation.vue'
export { default as OpportunityNotice } from '../../components/OpportunityNotice.vue'
export { default as OpportunityTime } from '../../components/OpportunityTime.vue'
export { default as PageView } from '../../components/PageView.vue'
export { default as Pagination } from '../../components/Pagination.vue'
export { default as PartnerForm } from '../../components/PartnerForm.vue'
export { default as PieChart } from '../../components/PieChart.vue'
export { default as ProfileItem } from '../../components/ProfileItem.vue'
export { default as ProgressBar } from '../../components/ProgressBar.vue'
export { default as ProgressGauge } from '../../components/ProgressGauge.vue'
export { default as ReadMore } from '../../components/ReadMore.vue'
export { default as SidewaysSlider } from '../../components/SidewaysSlider.vue'
export { default as SignupForm } from '../../components/SignupForm.vue'
export { default as SocialButton } from '../../components/SocialButton.vue'
export { default as Stars } from '../../components/Stars.vue'
export { default as SubFooter } from '../../components/SubFooter.vue'
export { default as Treemap } from '../../components/Treemap.vue'
export { default as UploadFile } from '../../components/UploadFile.vue'
export { default as WordCloud } from '../../components/WordCloud.vue'

// nuxt/nuxt.js#8607
function wrapFunctional(options) {
  if (!options || !options.functional) {
    return options
  }

  const propKeys = Array.isArray(options.props) ? options.props : Object.keys(options.props || {})

  return {
    render(h) {
      const attrs = {}
      const props = {}

      for (const key in this.$attrs) {
        if (propKeys.includes(key)) {
          props[key] = this.$attrs[key]
        } else {
          attrs[key] = this.$attrs[key]
        }
      }

      return h(options, {
        on: this.$listeners,
        attrs,
        props,
        scopedSlots: this.$scopedSlots,
      }, this.$slots.default)
    }
  }
}
