#[cfg(test)]
mod tests {
    use crate::types::*;
    use chrono::prelude::*;

    #[test]
    fn creation() {
        let rfc3339 = "2020-07-02T19:30:00+00:00";
        if let Ok(datetime) = rfc3339.parse::<DateTime<Utc>>() {
            assert_eq!(
                StockData::new("MSFT".to_string(), datetime).to_string(),
                format!("{},MSFT,-,-,-,-,-", rfc3339)
            );
        }
    }

    #[test]
    fn floating_number() {
        let rfc3339 = "2020-07-02T19:30:00+00:00";

        if let Ok(datetime) = rfc3339.parse::<DateTime<Utc>>() {
            let data = StockData::new("MSFT".to_string(), datetime)
                .close(44.2334_f32.into())
                .change(134.346_f32.into());

            assert_eq!(
                data.to_string(),
                format!("{},MSFT,$44.23,134.35%,-,-,-", rfc3339)
            );
        }
    }

    #[test]
    fn full_test() {
        let rfc3339 = "2020-07-02T19:30:00+00:00";

        if let Ok(datetime) = rfc3339.parse::<DateTime<Utc>>() {
            let data = StockData::new("MSFT".to_string(), datetime)
                .close(44.2334_f32.into())
                .change(134.346_f32.into())
                .min(22.22_f32.into())
                .max(88.88_f32.into())
                .sma_30(55.555_f32.into());

            assert_eq!(
                data.to_string(),
                format!("{},MSFT,$44.23,134.35%,$22.22,$88.88,$55.56", rfc3339)
            );
        }
    }
}
