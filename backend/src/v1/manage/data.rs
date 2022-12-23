use common::{
    model::{analytics::OverviewDemographics, person::Permission},
    Database,
};
use http_types::{Method, StatusCode};
use sailfish::TemplateOnce;
use tide_fluent_routes::{
    routebuilder::{RouteBuilder, RouteBuilderExt},
    RouteSegment,
};
use uuid::Uuid;

use crate::v1::redirect;

use super::{authorized_admin, IntoResponse};

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes
        .get(data)
        .at("demographics", |r| r.get(demographics).post(demographics))
}

#[derive(TemplateOnce, Default)]
#[template(path = "manage/data.stpl")]
struct DataPage;

pub async fn data(req: tide::Request<Database>) -> tide::Result {
    let _admin = match authorized_admin(&req, &Permission::ManageContent).await {
        Ok(person) => person,
        Err(resp) => return Ok(resp),
    };

    Ok(DataPage.into_response(StatusCode::Ok)?)
}

#[derive(TemplateOnce, Default)]
#[template(path = "manage/demographics.stpl")]
struct DemographicsPage;

pub async fn demographics(mut req: tide::Request<Database>) -> tide::Result {
    let _admin = match authorized_admin(&req, &Permission::ManageContent).await {
        Ok(person) => person,
        Err(resp) => return Ok(resp),
    };

    if let Method::Post = req.method() {
        let body = req.body_string().await?;

        let mut in_header = true;

        let mut demographics = OverviewDemographics::default();

        for line in body.lines() {
            if in_header {
                if line.starts_with("Type,Name,") {
                    in_header = false;
                }

                continue;
            }

            if line.starts_with("--") {
                break;
            }

            let cols: Vec<_> = line.split(',').collect();

            match cols[0] {
                "GENDER" => match cols[1] {
                    "Male" => {
                        demographics.sex.male.index = cols[2].parse()?;
                        demographics.sex.male.proportion = cols[3].parse()?;
                        demographics.sex.male.national = cols[4].parse()?;
                    }
                    "Female" => {
                        demographics.sex.female.index = cols[2].parse()?;
                        demographics.sex.female.proportion = cols[3].parse()?;
                        demographics.sex.female.national = cols[4].parse()?;
                    }
                    _ => {
                        dbg!(cols);
                        continue;
                    }
                },
                "AGE" => match cols[1] {
                    "18-20" => {
                        demographics.age.eighteen_twenty.values.index = cols[2].parse()?;
                        demographics.age.eighteen_twenty.values.proportion = cols[3].parse()?;
                        demographics.age.eighteen_twenty.values.national = cols[4].parse()?;
                    }
                    "21-24" => {
                        demographics.age.twentyone_twentyfour.values.index = cols[2].parse()?;
                        demographics.age.twentyone_twentyfour.values.proportion =
                            cols[3].parse()?;
                        demographics.age.twentyone_twentyfour.values.national = cols[4].parse()?;
                    }
                    "25-29" => {
                        demographics.age.twentyfive_twentynine.values.index = cols[2].parse()?;
                        demographics.age.twentyfive_twentynine.values.proportion =
                            cols[3].parse()?;
                        demographics.age.twentyfive_twentynine.values.national = cols[4].parse()?;
                    }
                    "30-34" => {
                        demographics.age.thirty_thirtyfour.values.index = cols[2].parse()?;
                        demographics.age.thirty_thirtyfour.values.proportion = cols[3].parse()?;
                        demographics.age.thirty_thirtyfour.values.national = cols[4].parse()?;
                    }
                    "35-39" => {
                        demographics.age.thirtyfive_thirtynine.values.index = cols[2].parse()?;
                        demographics.age.thirtyfive_thirtynine.values.proportion =
                            cols[3].parse()?;
                        demographics.age.thirtyfive_thirtynine.values.national = cols[4].parse()?;
                    }
                    "40-44" => {
                        demographics.age.forty_fortyfour.values.index = cols[2].parse()?;
                        demographics.age.forty_fortyfour.values.proportion = cols[3].parse()?;
                        demographics.age.forty_fortyfour.values.national = cols[4].parse()?;
                    }
                    "45-49" => {
                        demographics.age.fortyfive_fortynine.values.index = cols[2].parse()?;
                        demographics.age.fortyfive_fortynine.values.proportion = cols[3].parse()?;
                        demographics.age.fortyfive_fortynine.values.national = cols[4].parse()?;
                    }
                    "50-54" => {
                        demographics.age.fifty_fiftyfour.values.index = cols[2].parse()?;
                        demographics.age.fifty_fiftyfour.values.proportion = cols[3].parse()?;
                        demographics.age.fifty_fiftyfour.values.national = cols[4].parse()?;
                    }
                    "55-59" => {
                        demographics.age.fiftyfive_fiftynine.values.index = cols[2].parse()?;
                        demographics.age.fiftyfive_fiftynine.values.proportion = cols[3].parse()?;
                        demographics.age.fiftyfive_fiftynine.values.national = cols[4].parse()?;
                    }
                    "60-64" => {
                        demographics.age.sixty_sixtyfour.values.index = cols[2].parse()?;
                        demographics.age.sixty_sixtyfour.values.proportion = cols[3].parse()?;
                        demographics.age.sixty_sixtyfour.values.national = cols[4].parse()?;
                    }
                    "65+" => {
                        demographics.age.sixtyfive_plus.values.index = cols[2].parse()?;
                        demographics.age.sixtyfive_plus.values.proportion = cols[3].parse()?;
                        demographics.age.sixtyfive_plus.values.national = cols[4].parse()?;
                    }
                    _ => {
                        dbg!(cols);
                        continue;
                    }
                },
                "EDUCATION" => match cols[1] {
                    "No College" => {
                        demographics.education.no_college.index = cols[2].parse()?;
                        demographics.education.no_college.proportion = cols[3].parse()?;
                        demographics.education.no_college.national = cols[4].parse()?;
                    }
                    "College" => {
                        demographics.education.college.index = cols[2].parse()?;
                        demographics.education.college.proportion = cols[3].parse()?;
                        demographics.education.college.national = cols[4].parse()?;
                    }
                    "Grad. Sch." => {
                        demographics.education.grad_school.index = cols[2].parse()?;
                        demographics.education.grad_school.proportion = cols[3].parse()?;
                        demographics.education.grad_school.national = cols[4].parse()?;
                    }
                    _ => {
                        dbg!(cols);
                        continue;
                    }
                },
                "INCOME" => match cols[1] {
                    "$0-50k" => {
                        demographics.income.zero_fifty.index = cols[2].parse()?;
                        demographics.income.zero_fifty.proportion = cols[3].parse()?;
                        demographics.income.zero_fifty.national = cols[4].parse()?;
                    }
                    "$50-100k" => {
                        demographics.income.fifty_hundred.index = cols[2].parse()?;
                        demographics.income.fifty_hundred.proportion = cols[3].parse()?;
                        demographics.income.fifty_hundred.national = cols[4].parse()?;
                    }
                    "$100-150k" => {
                        demographics.income.hundred_hundredfifty.index = cols[2].parse()?;
                        demographics.income.hundred_hundredfifty.proportion = cols[3].parse()?;
                        demographics.income.hundred_hundredfifty.national = cols[4].parse()?;
                    }
                    "$150k+" => {
                        demographics.income.hundredfifty_plus.index = cols[2].parse()?;
                        demographics.income.hundredfifty_plus.proportion = cols[3].parse()?;
                        demographics.income.hundredfifty_plus.national = cols[4].parse()?;
                    }
                    _ => {
                        dbg!(cols);
                        continue;
                    }
                },
                "CHILDRENEXPANDED" => match cols[1] {
                    "No Children under 17" => {
                        demographics.children.none.index = cols[2].parse()?;
                        demographics.children.none.proportion = cols[3].parse()?;
                        demographics.children.none.national = cols[4].parse()?;
                    }
                    "Some Children under 17" => {
                        demographics.children.some.index = cols[2].parse()?;
                        demographics.children.some.proportion = cols[3].parse()?;
                        demographics.children.some.national = cols[4].parse()?;
                    }
                    _ => {
                        dbg!(cols);
                        continue;
                    }
                },
                "ETHNICITY" => match cols[1] {
                    "Cauc." => {
                        demographics.ethnicity.caucasian.index = cols[2].parse()?;
                        demographics.ethnicity.caucasian.proportion = cols[3].parse()?;
                        demographics.ethnicity.caucasian.national = cols[4].parse()?;
                    }
                    "Afr. Am." => {
                        demographics.ethnicity.african_american.index = cols[2].parse()?;
                        demographics.ethnicity.african_american.proportion = cols[3].parse()?;
                        demographics.ethnicity.african_american.national = cols[4].parse()?;
                    }
                    "Asian" => {
                        demographics.ethnicity.asian.index = cols[2].parse()?;
                        demographics.ethnicity.asian.proportion = cols[3].parse()?;
                        demographics.ethnicity.asian.national = cols[4].parse()?;
                    }
                    "Hisp" => {
                        demographics.ethnicity.hispanic.index = cols[2].parse()?;
                        demographics.ethnicity.hispanic.proportion = cols[3].parse()?;
                        demographics.ethnicity.hispanic.national = cols[4].parse()?;
                    }
                    "Other" => {
                        demographics.ethnicity.other.index = cols[2].parse()?;
                        demographics.ethnicity.other.proportion = cols[3].parse()?;
                        demographics.ethnicity.other.national = cols[4].parse()?;
                    }
                    _ => {
                        dbg!(cols);
                        continue;
                    }
                },
                "AGEANDGENDER" => match cols[1] {
                    "Male 18-20" => {
                        demographics.age.eighteen_twenty.male.index = cols[2].parse()?;
                        demographics.age.eighteen_twenty.male.proportion = cols[3].parse()?;
                        demographics.age.eighteen_twenty.male.national = cols[4].parse()?;
                    }
                    "Male 21-24" => {
                        demographics.age.twentyone_twentyfour.male.index = cols[2].parse()?;
                        demographics.age.twentyone_twentyfour.male.proportion = cols[3].parse()?;
                        demographics.age.twentyone_twentyfour.male.national = cols[4].parse()?;
                    }
                    "Male 25-29" => {
                        demographics.age.twentyfive_twentynine.male.index = cols[2].parse()?;
                        demographics.age.twentyfive_twentynine.male.proportion = cols[3].parse()?;
                        demographics.age.twentyfive_twentynine.male.national = cols[4].parse()?;
                    }
                    "Male 30-34" => {
                        demographics.age.thirty_thirtyfour.male.index = cols[2].parse()?;
                        demographics.age.thirty_thirtyfour.male.proportion = cols[3].parse()?;
                        demographics.age.thirty_thirtyfour.male.national = cols[4].parse()?;
                    }
                    "Male 35-39" => {
                        demographics.age.thirtyfive_thirtynine.male.index = cols[2].parse()?;
                        demographics.age.thirtyfive_thirtynine.male.proportion = cols[3].parse()?;
                        demographics.age.thirtyfive_thirtynine.male.national = cols[4].parse()?;
                    }
                    "Male 40-44" => {
                        demographics.age.forty_fortyfour.male.index = cols[2].parse()?;
                        demographics.age.forty_fortyfour.male.proportion = cols[3].parse()?;
                        demographics.age.forty_fortyfour.male.national = cols[4].parse()?;
                    }
                    "Male 45-49" => {
                        demographics.age.fortyfive_fortynine.male.index = cols[2].parse()?;
                        demographics.age.fortyfive_fortynine.male.proportion = cols[3].parse()?;
                        demographics.age.fortyfive_fortynine.male.national = cols[4].parse()?;
                    }
                    "Male 50-54" => {
                        demographics.age.fifty_fiftyfour.male.index = cols[2].parse()?;
                        demographics.age.fifty_fiftyfour.male.proportion = cols[3].parse()?;
                        demographics.age.fifty_fiftyfour.male.national = cols[4].parse()?;
                    }
                    "Male 55-59" => {
                        demographics.age.fiftyfive_fiftynine.male.index = cols[2].parse()?;
                        demographics.age.fiftyfive_fiftynine.male.proportion = cols[3].parse()?;
                        demographics.age.fiftyfive_fiftynine.male.national = cols[4].parse()?;
                    }
                    "Male 60-64" => {
                        demographics.age.sixty_sixtyfour.male.index = cols[2].parse()?;
                        demographics.age.sixty_sixtyfour.male.proportion = cols[3].parse()?;
                        demographics.age.sixty_sixtyfour.male.national = cols[4].parse()?;
                    }
                    "Male 65+" => {
                        demographics.age.sixtyfive_plus.male.index = cols[2].parse()?;
                        demographics.age.sixtyfive_plus.male.proportion = cols[3].parse()?;
                        demographics.age.sixtyfive_plus.male.national = cols[4].parse()?;
                    }
                    "Female 18-20" => {
                        demographics.age.eighteen_twenty.female.index = cols[2].parse()?;
                        demographics.age.eighteen_twenty.female.proportion = cols[3].parse()?;
                        demographics.age.eighteen_twenty.female.national = cols[4].parse()?;
                    }
                    "Female 21-24" => {
                        demographics.age.twentyone_twentyfour.female.index = cols[2].parse()?;
                        demographics.age.twentyone_twentyfour.female.proportion =
                            cols[3].parse()?;
                        demographics.age.twentyone_twentyfour.female.national = cols[4].parse()?;
                    }
                    "Female 25-29" => {
                        demographics.age.twentyfive_twentynine.female.index = cols[2].parse()?;
                        demographics.age.twentyfive_twentynine.female.proportion =
                            cols[3].parse()?;
                        demographics.age.twentyfive_twentynine.female.national = cols[4].parse()?;
                    }
                    "Female 30-34" => {
                        demographics.age.thirty_thirtyfour.female.index = cols[2].parse()?;
                        demographics.age.thirty_thirtyfour.female.proportion = cols[3].parse()?;
                        demographics.age.thirty_thirtyfour.female.national = cols[4].parse()?;
                    }
                    "Female 35-39" => {
                        demographics.age.thirtyfive_thirtynine.female.index = cols[2].parse()?;
                        demographics.age.thirtyfive_thirtynine.female.proportion =
                            cols[3].parse()?;
                        demographics.age.thirtyfive_thirtynine.female.national = cols[4].parse()?;
                    }
                    "Female 40-44" => {
                        demographics.age.forty_fortyfour.female.index = cols[2].parse()?;
                        demographics.age.forty_fortyfour.female.proportion = cols[3].parse()?;
                        demographics.age.forty_fortyfour.female.national = cols[4].parse()?;
                    }
                    "Female 45-49" => {
                        demographics.age.fortyfive_fortynine.female.index = cols[2].parse()?;
                        demographics.age.fortyfive_fortynine.female.proportion = cols[3].parse()?;
                        demographics.age.fortyfive_fortynine.female.national = cols[4].parse()?;
                    }
                    "Female 50-54" => {
                        demographics.age.fifty_fiftyfour.female.index = cols[2].parse()?;
                        demographics.age.fifty_fiftyfour.female.proportion = cols[3].parse()?;
                        demographics.age.fifty_fiftyfour.female.national = cols[4].parse()?;
                    }
                    "Female 55-59" => {
                        demographics.age.fiftyfive_fiftynine.female.index = cols[2].parse()?;
                        demographics.age.fiftyfive_fiftynine.female.proportion = cols[3].parse()?;
                        demographics.age.fiftyfive_fiftynine.female.national = cols[4].parse()?;
                    }
                    "Female 60-64" => {
                        demographics.age.sixty_sixtyfour.female.index = cols[2].parse()?;
                        demographics.age.sixty_sixtyfour.female.proportion = cols[3].parse()?;
                        demographics.age.sixty_sixtyfour.female.national = cols[4].parse()?;
                    }
                    "Female 65+" => {
                        demographics.age.sixtyfive_plus.female.index = cols[2].parse()?;
                        demographics.age.sixtyfive_plus.female.proportion = cols[3].parse()?;
                        demographics.age.sixtyfive_plus.female.national = cols[4].parse()?;
                    }
                    _ => {
                        dbg!(cols);
                        continue;
                    }
                },
                _ => {
                    dbg!(cols);
                    continue;
                }
            }
        }

        sqlx::query!(
            r#"
INSERT INTO c_demographics (
  "about",
   "data"
) VALUES (
  $1,
  $2
)
ON CONFLICT ("about") DO UPDATE SET
  "data" = EXCLUDED."data"
        "#,
            Uuid::nil(),
            serde_json::to_value(demographics)?
        )
        .execute(req.state())
        .await?;

        return Ok(redirect(req.url().path()));
    }

    Ok(DemographicsPage.into_response(StatusCode::Ok)?)
}
