//! Uses GraphQL and the WPgraphQL and EventsQL schema to query event
//! information from The Events Calendar Pro, and also queries custom
//! fields that were configured according to the Science Near Me
//! recommendations.

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
///             "id": "cG9zdDo0MQ==",
///             "content": null,
///             "cost": "99",
///             "currencySymbol": "$",
///             "endDate": "2021-11-03 17:00:00",
///             "link": "https://calendar.kevinripka.com/event/another-event/2021-11-03/",
///             "dateGmt": "2021-06-29T15:17:35",
///             "modifiedGmt": "2021-06-29T15:17:35",
///             "origin": "events-calendar",
///             "phone": null,
///             "recurring": true,
///             "slug": "another-event-2021-11-03",
///             "startDates": [
///               "2021-09-22 08:00:00",
///               "2021-09-29 08:00:00",
///               "2021-10-06 08:00:00",
///               "2021-10-13 08:00:00",
///               "2021-10-20 08:00:00",
///               "2021-10-27 08:00:00",
///               "2021-11-03 08:00:00"
///             ],
///             "status": "publish",
///             "timezone": "America/Chicago",
///             "title": "2020 Wisconsin Science Festival Events – Available Year Round",
///             "linkedData": {
///               "location ": {
///                 "url": "https://calendar.kevinripka.com/venue/some-bar/",
///                 "type": "Place",
///                 "telephone": "123.123.1234",
///                 "name": "Some Bar",
///                 "description": null,
///                 "address": {
///                   "addressCountry": "United States",
///                   "addressLocality": "Iowa City",
///                   "addressRegion": "IA",
///                   "postalCode": "52245",
///                   "streetAddress": "123 Bar St.",
///                   "type": "PostalAddress"
///                 }
///               },
///               "organizer": {
///                 "email": "&#102;ak&#101;&#064;f&#097;&#107;&#101;&#110;&#101;w&#115;.&#099;om",
///                 "telephone": "123.345.5678",
///                 "name": "Kevin Ripka Inc.",
///                 "type": "Person",
///                 "description": null,
///                 "url": "https://calendar.kevinripka.com/organizer/kevin-ripka-inc/"
///               },
///               "description": "asdfasdf\n"
///             },
///             "eventsCategories": {
///               "edges":[]
///             },
///             "featuredImage": {
///               "node": {
///                 "sourceUrl": "https://www.wisconsinsciencefest.org/wp-content/uploads/2021/07/Events_2021_-BuildingDreams3CuttingDownHierarchies_Thumbnail-300x300.jpg"
///               }
///             },
///             "tags": {
///               "edges": [
///                 {
///                   "node": {
///                     "name": "kid friendly"
///                   }
///                 },
///                 {
///                   "node": {
///                     "name": "krunked"
///                   }
///                 }
///               ]
///             },
///             "scienceNearMeData": {
///               "descriptor": [
///                 "concert",
///                 "live_science"
///               ],
///               "facebook": null,
///               "indoorsOutdoors": ["indoors"],
///               "instagram": null,
///               "maxAge": 78,
///               "minAge":45,
///               "online":true,
///               "organizationType": "planetarium",
///               "shortDescription": "This is a naksd jfa;lksjd fl;aksjd f;lkasj f;lkasdj fl;asjd f;lkasj dfl;kajs d;flkj as;dlkf ja;slkdj f;laksj df;laksj df",
///               "socialMediaHashtags": "#stuff",
///               "ticketRequired": "no",
///               "topic": [
///                 "art",
///                 "general_science",
///                 "policy"
///               ],
///               "twitter": "@stuff",
///               "language": [
///                 "en:English,"
///               ]
///             }
///           }
///         }
///       ],
///     }
///   },
///   "extensions": {
///     "debug": []
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
          startDate
          link
          dateGmt
          duration
          excerpt
          modifiedGmt
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
          scienceNearMeData {
            descriptor
            facebook
            indoorsOutdoors
            instagram
            maxAge
            minAge
            online
            organizationType
            shortDescription
            socialMediaHashtags
            ticketRequired
            topic
            twitter
            language
          }
        }
      }
    }
  }"#;

// We won't read more than 64 MiB from the server, even if it wants to
// send us that much. It's likely an error, or outright malicious.
pub const MAX_SIZE: usize = 64 * 1024 * 1024;

#[derive(Debug)]
pub struct EventsQLWithCustom {
    endpoint: String,
}

impl EventsQLWithCustom {
    pub fn new<S: AsRef<str>>(endpoint: S) -> Self {
        EventsQLWithCustom {
            endpoint: endpoint.as_ref().to_string(),
        }
    }
}

impl super::Source for EventsQLWithCustom {
    fn load(&self) -> Result<Bytes, LoggedError> {
        let mut writer = BytesMut::new().limit(MAX_SIZE).writer();
        let mut reader = ureq::post(&self.endpoint)
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
    fn fetch_kevenripka() {
        assert_ne!(
            dbg!(
                EventsQLWithCustom::new("https://calendar.kevinripka.com/graphql")
                    .load()
                    .unwrap()
            )[..10],
            b"{\"errors\":"[..]
        );
    }

    #[test]
    fn fetch_wisconsin_science_fest() {
        assert_ne!(
            EventsQLWithCustom::new("https://www.wisconsinsciencefest.org/graphql")
                .load()
                .unwrap()[..10],
            b"{\"errors\":"[..]
        );
    }
}
