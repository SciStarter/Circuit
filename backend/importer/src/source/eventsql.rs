//! Uses GraphQL and the WPgraphQL and EventsQL schema to query event
//! information from The Events Calendar Pro.

use bytes::{BufMut, Bytes, BytesMut};
use common::model::partner::LoggedError;

/// This query produces results similar to:
/// ```json
/// {
///   "data": {
///     "events": {
///       "edges": [
///         {
///           "node": {
///             "id": "cG9zdDoxNTk5OQ==",
///             "guid": "https://some-url/",
///             "content": "Click <a href=\"https://www.google.com/maps/d/u/0/edit?mid=133AFgi4qoZEpJpVC2L32_nIWbDErawmr&amp;ll=43.83100913970273%2C-90.53918123190857&amp;z=7\">HERE</a> to explore the fun activities and content from the 2020 Wisconsin Science Festival. The dots on the map include links from the recordings of our live events as well as great videos and at-home activities you can enjoy any time you'd like!\r\n\r\nMark your calendars for next year's Wisconsin Science Festival, October 21-24, 2021.",
///             "cost": null,
///             "currencySymbol": null,
///             "endDate": "2021-10-24 21:00:00",
///             "link": "https://www.wisconsinsciencefest.org/event/2020-wisconsin-science-festival-events-available-year-round/",
///             "modifiedGmt": "2021-05-14T16:39:33",
///             "origin": "community-events",
///             "phone": null,
///             "recurring": false,
///             "slug": "2020-wisconsin-science-festival-events-available-year-round",
///             "startDates": [
///               "2021-10-21 09:00:00"
///             ],
///             "status": "publish",
///             "timezone": "America/Chicago",
///             "title": "2020 Wisconsin Science Festival Events - Available Year Round",
///             "linkedData": {
///               "location": {
///                 "url": "https://www.wisconsinsciencefest.org/venue/uw-madison/",
///                 "type": "Place",
///                 "telephone": null,
///                 "name": "UW-Madison (Discovery Building)",
///                 "description": null,
///                 "address": {
///                   "addressCountry": "United States",
///                   "addressLocality": "Madison",
///                   "addressRegion": "WI",
///                   "postalCode": "53715",
///                   "streetAddress": "330 N. Orchard St.",
///                   "type": "PostalAddress"
///                 }
///               },
///               "organizer": {
///                 "email": "&#105;&#110;fo&#064;w&#105;s&#099;&#105;fes&#116;&#046;&#111;rg",
///                 "telephone": "608-316-4300",
///                 "name": "Wisconsin Science Festival",
///                 "type": "Person",
///                 "description": null,
///                 "url": "https://www.wisconsinsciencefest.org/organizer/wisconsin-science-festival/"
///               },
///               "description": "Click HERE to explore the fun activities and content from the 2020 Wisconsin Science Festival. The dots on the map include links from the recordings of our live events as well as great videos and at-home activities you can enjoy any time you\\'d like! Mark your calendars for next year\\'s Wisconsin Science Festival, October 21-24, … Continued\\n"
///             },
///             "eventsCategories": {
///               "edges": []
///             },
///             "featuredImage": {
///               "node": {
///                 "sourceUrl": "https://www.wisconsinsciencefest.org/wp-content/uploads/2020/10/Events_2020_Map_Thumbnail-300x300.jpg"
///               }
///             },
///             "tags": {
///               "edges": [
///                 {
///                   "node": {
///                     "name": "Featured Event"
///                   }
///                 }
///               ]
///             }
///           }
///         }
///       ]
///     }
///   },
///   "extensions": {
///     "debug": [
///       {
///         "type": "DEBUG_LOGS_INACTIVE",
///         "message": "GraphQL Debug logging is not active. To see debug logs, GRAPHQL_DEBUG must be enabled."
///       }
///     ]
///   }
/// }
/// ```
const QUERY: &'static str = r#"{
    events {
      edges {
        node {
          id
          guid
          content
          cost
          currencySymbol
          endDate
          link
          modifiedGmt
          dateGmt
          duration
          excerpt
          origin
          phone
          recurring
          slug
          startDates
          status
          timezone
          title
          linkedData {
            location {
              url
              type
              telephone
              name
              description
              address {
                addressCountry
                addressLocality
                addressRegion
                postalCode
                streetAddress
                type
              }
            }
            organizer {
              email
              telephone
              name
              type
              description
              url
            }
            description
          }
          eventsCategories {
            edges {
              node {
                name
              }
            }
          }
          featuredImage {
            node {
              sourceUrl(size: MEDIUM)
            }
          }
          tags {
            edges {
              node {
                name
              }
            }
          }
        }
      }
    }
  }"#;

// We won't read more than 64 MiB from the server, even if it wants to
// send us that much. It's likely an error, or outright malicious.
pub const MAX_SIZE: usize = 64 * 1024 * 1024;

#[derive(Debug)]
pub struct EventsQL {
    endpoint: String,
}

impl EventsQL {
    pub fn new<S: AsRef<str>>(endpoint: S) -> Self {
        EventsQL {
            endpoint: endpoint.as_ref().to_string(),
        }
    }
}

impl super::Source for EventsQL {
    fn load(&self) -> Result<Bytes, LoggedError> {
        let mut writer = BytesMut::new().limit(MAX_SIZE).writer();
        let mut reader = ureq::post(&self.endpoint)
            .set("User-Agent", "ScienceNearMe/1.0")
            .set("Content-Type", "application/json")
            .send_json(ureq::json!({"variables": {}, "query": QUERY}))?
            .into_reader();

        std::io::copy(&mut reader, &mut writer)?;

        Ok(writer.into_inner().into_inner().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::Source;

    #[test]
    fn fetch_museum_of_discovery_and_science() {
        assert_ne!(
            EventsQL::new("https://mods.org/graphql").load().unwrap()[..10],
            b"{\"errors\":"[..]
        );
    }
}
