type PlantBoxIdentity = u64;
type PlantDatabaseIdentity = u64;
type PlantPosition = (u32, u32);
type PlantSize = (u32, u32);

pub struct PlantArea {
    position: PlantPosition,
    size: PlantSize
}

pub struct PlantBox {
    database_id: PlantDatabaseIdentity,
    box_id: PlantBoxIdentity,
    area: PlantArea,
    name: String
}

pub struct Plant {
    database_id: PlantDatabaseIdentity,
    box_id: PlantBoxIdentity,
    area: PlantArea,
    name: String,
    binomial_name: Option<String>,
    last_time_watered: Option<u64>,
    next_time_to_be_watered: Option<u64>,
    last_time_fertilized: Option<u64>,
    next_time_to_be_fertilized: Option<u64>
}

pub struct PlantTemplate {
    database_id: PlantDatabaseIdentity,
    area: PlantArea,
    name: String,
    binomial_name: Option<String>,
    watering_interval_in_hours: Option<u64>,
    fertilizing_interval: Option<u64>
}

pub struct PlantDatabaseServer {

}

impl PlantDatabaseServer {
    
}

pub struct PlantDatabaseClient {

}

impl PlantDatabaseClient {
    fn get_plants_by_plant_box_area(plant_box_area: PlantArea) -> Vec<Plant> {
        let mut result = Vec::new();
        return result;
    }

    fn get_plants_by_plant_box_id(plant_box_id: PlantBoxIdentity) -> Vec<Plant> {
        let mut result = Vec::new();
        return result;
    }

    fn get_plants_by_plant_box_id_and_area(plant_box_id: Option<PlantBoxIdentity>, plant_box_area: Option<PlantArea>) -> Vec<Plant> {
        let mut result = Vec::new();
        return result;
    }

    fn get_plants_templates() -> Vec<PlantTemplate> {
        let mut result = Vec::new();
        return result;
    }

    fn add_or_update_plant(plant: &Plant) {

    }

    fn remove_plant_by_database_id(plant_database_id: PlantDatabaseIdentity) {

    }
}
