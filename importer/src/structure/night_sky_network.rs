use super::{
    Error,
    OneOrMany::{self, Many},
    Structure,
};
use common::model::Opportunity;
use serde_json::{from_value, Value};

pub struct NightSkyNetwork;

impl Structure for NightSkyNetwork {
    type Data = Opportunity;

    fn interpret(&self, parsed: Value) -> Result<OneOrMany<Self::Data>, Error> {
        if let Value::Array(objects) = parsed {
            let mut opps = Vec::new();

            for obj in objects {
                let mut input: Opportunity = from_value(obj)?;
                input.validate()?;
                opps.push(input);
            }

            Ok(Many(opps))
        } else {
            Err(Error::Structure(
                "Expected Night Sky Network data to be an array of objects".to_string(),
            ))
        }
    }
}
