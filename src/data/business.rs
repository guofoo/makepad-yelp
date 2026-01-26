#[derive(Clone, Debug)]
pub struct Business {
    pub id: String,
    pub name: String,
    pub image_url: String,
    pub rating: f32,
    pub review_count: u32,
    pub price: Option<String>,
    pub categories: Vec<String>,
    pub address: String,
    pub city: String,
    pub distance_meters: Option<f64>,
    pub latitude: f64,
    pub longitude: f64,
    pub phone: Option<String>,
    pub is_open_now: Option<bool>,
}

impl Business {
    pub fn price_and_categories(&self) -> String {
        let mut parts = Vec::new();
        if let Some(ref price) = self.price {
            parts.push(price.clone());
        }
        if !self.categories.is_empty() {
            parts.push(self.categories.join(", "));
        }
        if let Some(distance) = self.distance_meters {
            let miles = distance / 1609.34;
            parts.push(format!("{:.1} mi", miles));
        }
        parts.join(" · ")
    }
}

pub fn mock_businesses() -> Vec<Business> {
    vec![
        Business {
            id: "1".into(),
            name: "Flour + Water".into(),
            image_url: "".into(),
            rating: 4.5,
            review_count: 4521,
            price: Some("$$$".into()),
            categories: vec!["Italian".into(), "Pizza".into()],
            address: "2401 Harrison St".into(),
            city: "San Francisco".into(),
            distance_meters: Some(850.0),
            latitude: 37.7599,
            longitude: -122.4125,
            phone: Some("+14158260399".into()),
            is_open_now: Some(true),
        },
        Business {
            id: "2".into(),
            name: "Tartine Bakery".into(),
            image_url: "".into(),
            rating: 4.0,
            review_count: 8234,
            price: Some("$$".into()),
            categories: vec!["Bakeries".into(), "Cafes".into()],
            address: "600 Guerrero St".into(),
            city: "San Francisco".into(),
            distance_meters: Some(1200.0),
            latitude: 37.7614,
            longitude: -122.4241,
            phone: Some("+14154872600".into()),
            is_open_now: Some(true),
        },
        Business {
            id: "3".into(),
            name: "Burma Superstar".into(),
            image_url: "".into(),
            rating: 4.0,
            review_count: 6712,
            price: Some("$$".into()),
            categories: vec!["Burmese".into(), "Asian Fusion".into()],
            address: "309 Clement St".into(),
            city: "San Francisco".into(),
            distance_meters: Some(3400.0),
            latitude: 37.7829,
            longitude: -122.4633,
            phone: Some("+14153872147".into()),
            is_open_now: Some(false),
        },
        Business {
            id: "4".into(),
            name: "Zuni Cafe".into(),
            image_url: "".into(),
            rating: 4.0,
            review_count: 3891,
            price: Some("$$$".into()),
            categories: vec!["American".into(), "Mediterranean".into()],
            address: "1658 Market St".into(),
            city: "San Francisco".into(),
            distance_meters: Some(2100.0),
            latitude: 37.7735,
            longitude: -122.4214,
            phone: Some("+14155522522".into()),
            is_open_now: Some(true),
        },
        Business {
            id: "5".into(),
            name: "La Taqueria".into(),
            image_url: "".into(),
            rating: 4.0,
            review_count: 5423,
            price: Some("$".into()),
            categories: vec!["Mexican".into(), "Tacos".into()],
            address: "2889 Mission St".into(),
            city: "San Francisco".into(),
            distance_meters: Some(1800.0),
            latitude: 37.7514,
            longitude: -122.4183,
            phone: Some("+14152857117".into()),
            is_open_now: Some(true),
        },
        Business {
            id: "6".into(),
            name: "State Bird Provisions".into(),
            image_url: "".into(),
            rating: 4.5,
            review_count: 2987,
            price: Some("$$$".into()),
            categories: vec!["American".into(), "Small Plates".into()],
            address: "1529 Fillmore St".into(),
            city: "San Francisco".into(),
            distance_meters: Some(2800.0),
            latitude: 37.7833,
            longitude: -122.4324,
            phone: Some("+14157951272".into()),
            is_open_now: Some(false),
        },
    ]
}
